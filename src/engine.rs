// Interlude: Engine and EngineLogger

#![allow(dead_code)]

use super::*;
use render_surface;
use tuple_tools::*;
use device::Device;
use rawexports::*;
use ginterface::DeviceFeatures;
use {std, log};
use vk::defs::*;
use ansi_term::*;
use std::sync::{Arc, RwLock};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::borrow::Cow;
use std::ops::Deref;
use std::rc::Rc;

// Select WindowSystem and InputSystem
#[cfg(windows)] use win32::NativeWindow;
#[cfg(unix)] use linux::NativeWindowAndServerCon as NativeWindow;

struct EngineLogger;
impl log::Log for EngineLogger
{
	fn enabled(&self, metadata: &log::LogMetadata) -> bool
	{
		metadata.level() <= log::LogLevel::Info
	}
	fn log(&self, record: &log::LogRecord)
	{
		if self.enabled(record.metadata())
		{
			println!("{}", match record.level()
			{
				log::LogLevel::Error => Style::new().bold().fg(Color::Red).paint(format!("!! [{}|{}] {}", record.target(), record.level(), record.args())),
				log::LogLevel::Warn => Style::new().bold().fg(Color::Yellow).paint(format!("== [{}|{}] {}", record.target(), record.level(), record.args())),
				_ => Style::new().bold().paint(format!("** [{}|{}] {}", record.target(), record.level(), record.args()))
			});
		}
	}
}
impl EngineLogger
{
	fn setup()
	{
		log::set_logger(|max_log_level| { max_log_level.set(log::LogLevelFilter::Info); Box::new(EngineLogger) }).unwrap();
		info!(target: "Interlude", "Initializing Engine...");
	}
}

fn mtflags_decomposite(flags: VkMemoryPropertyFlags) -> Vec<&'static str>
{
	let mut temp = Vec::new();
	if (flags & VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT) != 0 { temp.push("Device Local"); }
	if (flags & VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT) != 0 { temp.push("Host Visible"); }
	if (flags & VK_MEMORY_PROPERTY_HOST_COHERENT_BIT) != 0 { temp.push("Host Coherent"); }
	if (flags & VK_MEMORY_PROPERTY_HOST_CACHED_BIT) != 0 { temp.push("Host Cached"); }
	if (flags & VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT) != 0 { temp.push("Lazily Allocated"); }
	temp
}

/// Placeholder for applications that never uses any inputs.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EmptyInput {}

pub trait EngineCoreExports
{
	fn graphics(&self) -> &GraphicsInterface;
}
pub struct EngineBuilder<'p, InputNames: Eq + Copy + Ord>
{
	app_name: Cow<'static, str>, app_version: u32, asset_base: Option<Cow<'p, Path>>, extra_features: DeviceFeatures,
	caption: Cow<'static, str>, size: Size2, resizable: bool, ph: std::marker::PhantomData<InputNames>
}
impl<'p, InputNames: Eq + Copy + Ord> EngineBuilder<'p, InputNames>
{
	pub fn new(app_name: Cow<'static, str>, app_version: (u32, u32, u32), caption: Cow<'static, str>, size: &Size2) -> Self
	{
		EngineBuilder
		{
			app_name: app_name, app_version: VK_MAKE_VERSION!(app_version.0, app_version.1, app_version.2),
			caption: caption, size: size.clone(), resizable: false,
			asset_base: None, extra_features: DeviceFeatures::new(), ph: std::marker::PhantomData
		}
	}
	pub fn asset_base(mut self, asset_base: Cow<'p, Path>) -> Self
	{
		self.asset_base = Some(asset_base);
		self
	}
	pub fn device_feature_block_texture_compression(mut self) -> Self
	{
		self.extra_features.enable_block_texture_compression();
		self
	}
	pub fn device_feature_nonsolid_fillmode(mut self) -> Self
	{
		self.extra_features.enable_nonsolid_fillmode();
		self
	}
	pub fn resizable_window(mut self) -> Self
	{
		self.resizable = true;
		self
	}

	pub fn launch(self) -> Result<Engine<InputNames>, EngineError> { Engine::new(self) }
}
pub struct LazyLoadedResource<T>(Option<T>);
impl<T> LazyLoadedResource<T>
{
	fn new() -> Self { LazyLoadedResource(None) }
	fn get<'s, F>(&'s self, initializer: F) -> Result<&'s T, EngineError> where F: FnOnce() -> Result<T, EngineError>
	{
		if self.0.is_none()
		{
			unsafe { *std::mem::transmute::<_, *mut Option<T>>(&self.0) = Some(try!{initializer()}); }
		}
		Ok(self.0.as_ref().unwrap())
	}
}
pub struct EngineResources
{
	postprocess_vsh: LazyLoadedResource<Rc<VertexShader>>, postprocess_vsh_nouv: LazyLoadedResource<Rc<VertexShader>>,
	default_renderpass: HashMap<(Option<bool>, VkFormat), RenderPass>, presenting_renderpass: HashMap<(Option<bool>, VkFormat), RenderPass>
}
pub struct Engine<InputNames: Eq + Copy + Ord>
{
	window: Arc<RenderWindow>, input_system: Arc<RwLock<Input<InputNames>>>, gi: GraphicsInterface,
	asset_dir: PathBuf, common_resources: EngineResources
}
unsafe impl<InputNames: Eq + Copy + Ord> Send for Engine<InputNames> {}
impl<InputNames: Eq + Copy + Ord> EngineCoreExports for Engine<InputNames>
{
	fn graphics(&self) -> &GraphicsInterface { &self.gi }
}
macro_rules! FunComposite1
{
	($f: expr; $g: expr) => {|x| $f($g(x))}
}
impl<InputNames: Eq + Copy + Ord> Engine<InputNames>
{
	pub fn new(info: EngineBuilder<InputNames>) -> Result<Self, EngineError>
	{
		EngineLogger::setup();

		let EngineBuilder { app_name, app_version, extra_features, size, caption, resizable, asset_base, .. } = info;

		GraphicsInterface::new(app_name, app_version, &extra_features).and_then(|gi|
		{
			let window = NativeWindow::new(&size, &caption, resizable).and_then(|n| render_surface::make_render_window(n, &gi, &size));
			let ni = Input::new().map(FunComposite1!(Arc::new; RwLock::new));

			(window, ni).flatten().map(move |(window, ni)| Engine
			{
				window: Arc::new(window), input_system: ni, gi: gi,
				asset_dir: asset_base.map(Cow::into_owned).or_else(|| std::env::current_exe().unwrap().parent().map(Path::to_path_buf))
					.unwrap().join("assets"),
				common_resources: EngineResources::new()
			})
		})
	}

	pub fn render_window(&self) -> &Arc<RenderWindow> { &self.window }
}
// For any WindowSystems
impl<InputNames: Eq + Copy + Ord> Engine<InputNames>
{
	pub fn input_system_ref(&self) -> &Arc<RwLock<Input<InputNames>>> { &self.input_system }

	pub fn process_messages(&self) -> bool { self.window.process_messages() == ApplicationState::Continue }
	pub fn process_all_messages(&self) { self.window.process_all_messages() }
	pub fn process_events_and_messages(&self, events: &[&Event]) -> ApplicationState { self.window.process_events_and_messages(events) }
}
/// The Asset Provider that can parse AssetPath and provides some of pre-defined objects
pub trait AssetProvider
{
	fn parse_asset<P: AssetPath>(&self, path: P, extension: &str) -> PathBuf;

	fn postprocess_vsh(&self, require_uv: bool) -> EngineResult<&Rc<VertexShader>>;
	fn default_renderpass(&self, format: VkFormat, clear_mode: Option<bool>) -> EngineResult<&RenderPass>;
	fn presenting_renderpass(&self, format: VkFormat, clear_mode: Option<bool>) -> EngineResult<&RenderPass>;
}
impl<InputNames: Eq + Copy + Ord> AssetProvider for Engine<InputNames>
{
	fn parse_asset<P: AssetPath>(&self, path: P, extension: &str) -> PathBuf
	{
		path.decode(&self.asset_dir, extension)
	}

	fn postprocess_vsh(&self, require_uv: bool) -> EngineResult<&Rc<VertexShader>>
	{
		if require_uv { self.common_resources.postprocess_vsh(self) } else { self.common_resources.postprocess_vsh_nouv(self) }
	}
	fn default_renderpass(&self, format: VkFormat, clear_mode: Option<bool>) -> EngineResult<&RenderPass>
	{
		self.common_resources.default_renderpass(&self.gi, format, clear_mode)
	}
	fn presenting_renderpass(&self, format: VkFormat, clear_mode: Option<bool>) -> EngineResult<&RenderPass>
	{
		self.common_resources.presenting_renderpass(&self.gi, format, clear_mode)
	}
}
/// Asset Path, implemented for some types because optimization.
pub trait AssetPath
{
	fn decode(self, basedir: &Path, extension: &str) -> PathBuf;
}
impl<'s> AssetPath for &'s str
{
	fn decode(self, basedir: &Path, extension: &str) -> PathBuf
	{
		basedir.join(self.replace(".", "/")).with_extension(extension)
	}
}
impl<'s> AssetPath for &'s [String]
{
	fn decode(self, basedir: &Path, extension: &str) -> PathBuf
	{
		basedir.join(self.join("/")).with_extension(extension)
	}
}
impl<'s> AssetPath for &'s [&'s str]
{
	fn decode(self, basedir: &Path, extension: &str) -> PathBuf
	{
		basedir.join(self.join("/")).with_extension(extension)
	}
}
impl<'s, T> AssetPath for &'s Vec<T> where &'s [T]: AssetPath
{
	fn decode(self, basedir: &Path, extension: &str) -> PathBuf { AssetPath::decode(&self[..], basedir, extension) }
}
// Delayed Loaders for EngineResources
impl EngineResources
{
	fn new() -> Self
	{
		EngineResources
		{
			postprocess_vsh: LazyLoadedResource::new(), postprocess_vsh_nouv: LazyLoadedResource::new(),
			default_renderpass: HashMap::new(), presenting_renderpass: HashMap::new()
		}
	}

	fn postprocess_vsh<Engine: AssetProvider + Deref<Target = GraphicsInterface>>(&self, context: &Engine) -> EngineResult<&Rc<VertexShader>>
	{
		self.postprocess_vsh.get(|| VertexShader::from_asset(context, "engine.shaders.PostProcessVertex", "main",
			&[VertexBinding::PerVertex(std::mem::size_of::<PosUV>() as u32)], &[VertexAttribute(0, VkFormat::R32G32B32A32_SFLOAT, 0)]))
	}
	fn postprocess_vsh_nouv<Engine: AssetProvider + Deref<Target = GraphicsInterface>>(&self, context: &Engine) -> EngineResult<&Rc<VertexShader>>
	{
		self.postprocess_vsh_nouv.get(|| VertexShader::from_asset(context, "engine.shaders.PostProcessVertexNoUV", "main",
			&[VertexBinding::PerVertex(std::mem::size_of::<PosUV>() as u32)], &[VertexAttribute(0, VkFormat::R32G32B32A32_SFLOAT, 0)]))
	}
	fn default_renderpass(&self, context: &GraphicsInterface, format: VkFormat, clear_mode: Option<bool>) -> EngineResult<&RenderPass>
	{
		Ok(unsafe { &mut *std::mem::transmute::<_, *mut HashMap<(Option<bool>, VkFormat), RenderPass>>(&self.default_renderpass) }.entry((clear_mode, format)).or_insert({
			let attachment = AttachmentDesc
			{
				format: format, clear_on_load: clear_mode, preserve_stored_value: true,
				initial_layout: VkImageLayout::ColorAttachmentOptimal, final_layout: VkImageLayout::ShaderReadOnlyOptimal,
				.. Default::default()
			};
			let pass = PassDesc::single_fragment_output(0);
			try!(RenderPass::new(context, &[attachment], &[pass], &[]))
		}))
	}
	fn presenting_renderpass(&self, context: &GraphicsInterface, format: VkFormat, clear_mode: Option<bool>) -> EngineResult<&RenderPass>
	{
		Ok(unsafe { &mut *std::mem::transmute::<_, *mut HashMap<(Option<bool>, VkFormat), RenderPass>>(&self.presenting_renderpass) }.entry((clear_mode, format)).or_insert({
			let attachment = AttachmentDesc
			{
				format: format, clear_on_load: clear_mode, preserve_stored_value: true,
				initial_layout: VkImageLayout::ColorAttachmentOptimal, final_layout: VkImageLayout::PresentSrcKHR,
				.. Default::default()
			};
			let pass = PassDesc::single_fragment_output(0);
			try!(RenderPass::new(context, &[attachment], &[pass], &[]))
		}))
	}
}
impl<InputNames: Eq + Copy + Ord> Deref for Engine<InputNames> { type Target = GraphicsInterface; fn deref(&self) -> &Self::Target { &self.gi } }

// support function for as_ptr: returns null when the container is empty
fn as_ptr_emp<T>(v: &[T]) -> *const T { if v.is_empty() { std::ptr::null() } else { v.as_ptr() } }

pub trait CommandSubmitter
{
	fn submit_graphics_commands(&self, commands: &GraphicsCommandBuffersView, wait_for_execute: &[(&QueueFence, VkPipelineStageFlags)],
		signal_on_complete: Option<&QueueFence>, signal_on_complete_host: Option<&Fence>) -> Result<(), EngineError>;
	fn submit_transfer_commands(&self, commands: &TransferCommandBuffersView, wait_for_execute: &[(&QueueFence, VkPipelineStageFlags)],
		signal_on_complete: Option<&QueueFence>, signal_on_complete_host: Option<&Fence>) -> Result<(), EngineError>;
}
impl<InputNames: Eq + Copy + Ord> CommandSubmitter for Engine<InputNames>
{
	fn submit_graphics_commands(&self, commands: &GraphicsCommandBuffersView, wait_for_execute: &[(&QueueFence, VkPipelineStageFlags)],
		signal_on_complete: Option<&QueueFence>, signal_on_complete_host: Option<&Fence>) -> Result<(), EngineError>
	{
		let signals_on_complete = signal_on_complete.into_iter().map(qfence_raw).collect::<Vec<_>>();
		let wait_stages = wait_for_execute.into_iter().map(|&(_, s)| s).collect::<Vec<_>>();
		let wait_semaphores = wait_for_execute.into_iter().map(|&(q, _)| qfence_raw(q)).collect::<Vec<_>>();

		let subinfo = VkSubmitInfo
		{
			sType: VkStructureType::SubmitInfo, pNext: std::ptr::null(),
			commandBufferCount: commands.len() as u32, pCommandBuffers: commands.as_ptr(),
			waitSemaphoreCount: wait_semaphores.len() as u32, pWaitSemaphores: wait_semaphores.as_ptr(), pWaitDstStageMask: as_ptr_emp(&wait_stages),
			signalSemaphoreCount: signals_on_complete.len() as u32, pSignalSemaphores: signals_on_complete.as_ptr()
		};
		self.gi.device().graphics_queue.submit(&[subinfo], signal_on_complete_host.map(fence_raw)).map_err(EngineError::from)
	}
	fn submit_transfer_commands(&self, commands: &TransferCommandBuffersView, wait_for_execute: &[(&QueueFence, VkPipelineStageFlags)],
		signal_on_complete: Option<&QueueFence>, signal_on_complete_host: Option<&Fence>) -> Result<(), EngineError>
	{
		let signals_on_complete = signal_on_complete.into_iter().map(qfence_raw).collect::<Vec<_>>();
		let (wait_semaphores, wait_stages): (Vec<_>, Vec<_>) = wait_for_execute.into_iter().map(|&(q, s)| (qfence_raw(q), s)).unzip();

		let subinfo = VkSubmitInfo
		{
			sType: VkStructureType::SubmitInfo, pNext: std::ptr::null(),
			commandBufferCount: commands.len() as u32, pCommandBuffers: commands.as_ptr(),
			waitSemaphoreCount: wait_semaphores.len() as u32,
			pWaitSemaphores: wait_semaphores.as_ptr(),
			pWaitDstStageMask: as_ptr_emp(&wait_stages),
			signalSemaphoreCount: signals_on_complete.len() as u32, pSignalSemaphores: signals_on_complete.as_ptr()
		};
		self.gi.device().transfer_queue.submit(&[subinfo], signal_on_complete_host.map(fence_raw)).map_err(EngineError::from)
	}
}
pub struct CommandSender<'a>(&'a Device);
unsafe impl<'a> Send for CommandSender<'a> {}
impl<'a> CommandSubmitter for CommandSender<'a>
{
	fn submit_graphics_commands(&self, commands: &GraphicsCommandBuffersView, wait_for_execute: &[(&QueueFence, VkPipelineStageFlags)],
		signal_on_complete: Option<&QueueFence>, signal_on_complete_host: Option<&Fence>) -> Result<(), EngineError>
	{
		let signals_on_complete = signal_on_complete.into_iter().map(qfence_raw).collect::<Vec<_>>();
		let wait_stages = wait_for_execute.into_iter().map(|&(_, s)| s).collect::<Vec<_>>();
		let wait_semaphores = wait_for_execute.into_iter().map(|&(q, _)| qfence_raw(q)).collect::<Vec<_>>();

		let subinfo = VkSubmitInfo
		{
			sType: VkStructureType::SubmitInfo, pNext: std::ptr::null(),
			commandBufferCount: commands.len() as u32, pCommandBuffers: commands.as_ptr(),
			waitSemaphoreCount: wait_semaphores.len() as u32, pWaitSemaphores: wait_semaphores.as_ptr(), pWaitDstStageMask: as_ptr_emp(&wait_stages),
			signalSemaphoreCount: signals_on_complete.len() as u32, pSignalSemaphores: signals_on_complete.as_ptr()
		};
		self.0.graphics_queue.submit(&[subinfo], signal_on_complete_host.map(fence_raw)).map_err(EngineError::from)
	}
	fn submit_transfer_commands(&self, commands: &TransferCommandBuffersView, wait_for_execute: &[(&QueueFence, VkPipelineStageFlags)],
		signal_on_complete: Option<&QueueFence>, signal_on_complete_host: Option<&Fence>) -> Result<(), EngineError>
	{
		let signals_on_complete = signal_on_complete.into_iter().map(qfence_raw).collect::<Vec<_>>();
		let wait_stages = wait_for_execute.into_iter().map(|&(_, s)| s).collect::<Vec<_>>();
		let wait_semaphores = wait_for_execute.into_iter().map(|&(q, _)| qfence_raw(q)).collect::<Vec<_>>();

		let subinfo = VkSubmitInfo
		{
			sType: VkStructureType::SubmitInfo, pNext: std::ptr::null(),
			commandBufferCount: commands.len() as u32, pCommandBuffers: commands.as_ptr(),
			waitSemaphoreCount: wait_semaphores.len() as u32, pWaitSemaphores: wait_semaphores.as_ptr(), pWaitDstStageMask: as_ptr_emp(&wait_stages),
			signalSemaphoreCount: signals_on_complete.len() as u32, pSignalSemaphores: signals_on_complete.as_ptr()
		};
		self.0.transfer_queue.submit(&[subinfo], signal_on_complete_host.map(fence_raw)).map_err(EngineError::from)
	}
}

// Interlude: Engine and EngineLogger

use interlude_vk_defs::*;
use interlude_vk_funport::vkQueueSubmit;
use subsystem_layer::{NativeResultValueHandler, NativeHandleProvider};
use ginterface::DeviceFeatures;
use wsi::NativeWindowBase;
use {
	log, EngineResult, Fence, QueueFence, GraphicsCommandBuffersView, TransferCommandBuffersView, GraphicsInterface,
	RenderPass, AttachmentDesc, PassDesc, VertexAttribute, VertexBinding, PosUV, VertexShader, ApplicationState, Event,
	RenderWindow, Size2, PipelineStageFlag, Format, PackedPixelOrder, FormatType
};
use std::sync::{Arc, RwLock};
use std::path::{Path, PathBuf};
use std::borrow::Cow;
use std::ops::Deref;
use std::rc::Rc;
use std::mem::{size_of, zeroed};
use std::ptr::null;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::env;
use std::marker::PhantomData;

#[cfg(unix)] use linux::NativeInput as Input;
#[cfg(windows)] use win32::NativeInput as Input;

struct EngineLogger;
impl log::Log for EngineLogger
{
	fn enabled(&self, metadata: &log::LogMetadata) -> bool
	{
		metadata.level() <= log::LogLevel::Info
	}
	fn log(&self, record: &log::LogRecord)
	{
		use ansi_term::*;

		if self.enabled(record.metadata())
		{
			println!("{}", match record.level()
			{
				log::LogLevel::Error => Style::new().bold().fg(Color::Red).paint(format!("!! [{}|{}] {}", record.target(), record.level(), record.args())),
				log::LogLevel::Warn => Style::new().fg(Color::Yellow).paint(format!("== [{}|{}] {}", record.target(), record.level(), record.args())),
				_ => Style::new().paint(format!("** [{}|{}] {}", record.target(), record.level(), record.args()))
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

/*fn mtflags_decomposite(flags: VkMemoryPropertyFlags) -> Vec<&'static str>
{
	let mut temp = Vec::new();
	if (flags & VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT) != 0 { temp.push("Device Local"); }
	if (flags & VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT) != 0 { temp.push("Host Visible"); }
	if (flags & VK_MEMORY_PROPERTY_HOST_COHERENT_BIT) != 0 { temp.push("Host Coherent"); }
	if (flags & VK_MEMORY_PROPERTY_HOST_CACHED_BIT) != 0 { temp.push("Host Cached"); }
	if (flags & VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT) != 0 { temp.push("Lazily Allocated"); }
	temp
}*/

/// Placeholder for applications that never uses any inputs.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EmptyInput {}

pub trait EngineCoreExports
{
	fn graphics(&self) -> &GraphicsInterface;
}
pub struct EngineBuilder<'p, InputNames: Eq + Copy + Ord>
{
	app_name: &'static str, app_version: u32, asset_base: Option<Cow<'p, Path>>, extra_features: DeviceFeatures,
	caption: &'static str, size: Size2, resizable: bool, ph: PhantomData<InputNames>
}
impl<'p, InputNames: Eq + Copy + Ord> EngineBuilder<'p, InputNames>
{
	pub fn new(app_name: &'static str, app_version: (u32, u32, u32), caption: &'static str, size: &Size2) -> Self
	{
		EngineBuilder
		{
			app_name, app_version: VK_MAKE_VERSION!(app_version.0, app_version.1, app_version.2),
			caption, size: size.clone(), resizable: false,
			asset_base: None, extra_features: DeviceFeatures::new(), ph: PhantomData
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

	pub fn launch(self) -> EngineResult<Engine<InputNames>> { Engine::new(self) }
}
pub struct LazyLoadedResource<T>(Option<T>);
impl<T> LazyLoadedResource<T>
{
	fn new() -> Self { LazyLoadedResource(None) }
	fn get<'s, F>(&'s self, initializer: F) -> EngineResult<&'s T> where F: FnOnce() -> EngineResult<T>
	{
		if self.0.is_none()
		{
			unsafe { *(&self.0 as *const _ as *mut _) = Some(initializer()?) };
		}
		Ok(self.0.as_ref().unwrap())
	}
}
pub struct EngineResources
{
	postprocess_vsh: LazyLoadedResource<VertexShader>, postprocess_vsh_nouv: LazyLoadedResource<VertexShader>,
	default_renderpass: HashMap<(Option<bool>, VkFormat), RenderPass>, presenting_renderpass: HashMap<(Option<bool>, VkFormat), RenderPass>
}
pub struct Engine<InputNames: Eq + Copy + Ord>
{
	window: Rc<RenderWindow>, input_system: Arc<RwLock<Input<InputNames>>>, gi: GraphicsInterface,
	asset_dir: PathBuf, common_resources: EngineResources
}
unsafe impl<InputNames: Eq + Copy + Ord> Send for Engine<InputNames> {}
impl<InputNames: Eq + Copy + Ord> EngineCoreExports for Engine<InputNames>
{
	fn graphics(&self) -> &GraphicsInterface { &self.gi }
}
macro_rules! FunComposite1 { ($f: expr; $g: expr) => {|x| $f($g(x))} }
impl<InputNames: Eq + Copy + Ord> Engine<InputNames>
{
	pub fn new(info: EngineBuilder<InputNames>) -> EngineResult<Self>
	{
		EngineLogger::setup();

		let EngineBuilder { app_name, app_version, extra_features, size, caption, resizable, asset_base, .. } = info;
		let gi = GraphicsInterface::new(app_name, app_version, &extra_features)?;
		let window = RenderWindow::new(&gi, &size, caption, resizable).map(Rc::new)?;
		let input_system = Input::new().map(FunComposite1!(Arc::new; RwLock::new))?;

		window.show(); window.flush();
		Ok(Engine
		{
			window, input_system, gi, common_resources: EngineResources::new(),
			asset_dir: asset_base.map(Cow::into_owned).or_else(|| env::current_exe().unwrap().parent().map(Path::to_path_buf)).unwrap().join("assets")
		})
	}

	pub fn render_window(&self) -> &Rc<RenderWindow> { &self.window }
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

	fn postprocess_vsh(&self, require_uv: bool) -> EngineResult<&VertexShader>;
	fn default_renderpass(&self, format: VkFormat, clear_mode: Option<bool>) -> EngineResult<&RenderPass>;
	fn presenting_renderpass(&self, format: VkFormat, clear_mode: Option<bool>) -> EngineResult<&RenderPass>;
}
impl<InputNames: Eq + Copy + Ord> AssetProvider for Engine<InputNames>
{
	fn parse_asset<P: AssetPath>(&self, path: P, extension: &str) -> PathBuf
	{
		path.decode(&self.asset_dir, extension)
	}

	fn postprocess_vsh(&self, require_uv: bool) -> EngineResult<&VertexShader>
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

	fn postprocess_vsh<Engine: AssetProvider + Deref<Target = GraphicsInterface>>(&self, context: &Engine) -> EngineResult<&VertexShader>
	{
		self.postprocess_vsh.get(|| VertexShader::from_asset(context, "engine.shaders.PostProcessVertex", "main",
			&[VertexBinding::PerVertex(size_of::<PosUV>() as u32)], &[VertexAttribute(0, Format::Component(32, PackedPixelOrder::RGBA, FormatType::Float), 0)]))
	}
	fn postprocess_vsh_nouv<Engine: AssetProvider + Deref<Target = GraphicsInterface>>(&self, context: &Engine) -> EngineResult<&VertexShader>
	{
		self.postprocess_vsh_nouv.get(|| VertexShader::from_asset(context, "engine.shaders.PostProcessVertexNoUV", "main",
			&[VertexBinding::PerVertex(size_of::<PosUV>() as u32)], &[VertexAttribute(0, Format::Component(32, PackedPixelOrder::RGBA, FormatType::Float), 0)]))
	}
	fn default_renderpass(&self, context: &GraphicsInterface, format: VkFormat, clear_mode: Option<bool>) -> EngineResult<&RenderPass>
	{
		let map: &mut HashMap<(Option<bool>, VkFormat), RenderPass> = unsafe { &mut *(&self.default_renderpass as *const _ as *mut _) };
		match map.entry((clear_mode, format))
		{
			Entry::Occupied(v) => Ok(v.into_mut()),
			Entry::Vacant(v) =>
			{
				let attachment = AttachmentDesc
				{
					format: format, clear_on_load: clear_mode, preserve_stored_value: true,
					initial_layout: VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL, final_layout: VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL,
					.. Default::default()
				};
				Ok(v.insert(RenderPass::new(context, &[attachment], &[PassDesc::single_fragment_output(0)], &[])?))
			}
		}
	}
	fn presenting_renderpass(&self, context: &GraphicsInterface, format: VkFormat, clear_mode: Option<bool>) -> EngineResult<&RenderPass>
	{
		let map: &mut HashMap<(Option<bool>, VkFormat), RenderPass> = unsafe { &mut *(&self.presenting_renderpass as *const _ as *mut _) };
		match map.entry((clear_mode, format))
		{
			Entry::Occupied(v) => Ok(v.into_mut()),
			Entry::Vacant(v) =>
			{
				let attachment = AttachmentDesc
				{
					format: format, clear_on_load: clear_mode, preserve_stored_value: true,
					initial_layout: VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL, final_layout: VK_IMAGE_LAYOUT_PRESENT_SRC_KHR,
					.. Default::default()
				};
				Ok(v.insert(RenderPass::new(context, &[attachment], &[PassDesc::single_fragment_output(0)], &[])?))
			}
		}
	}
}
impl<InputNames: Eq + Copy + Ord> Deref for Engine<InputNames> { type Target = GraphicsInterface; fn deref(&self) -> &Self::Target { &self.gi } }

// support function for as_ptr: returns null when the container is empty
fn as_ptr_emp<T>(v: &[T]) -> *const T { if v.is_empty() { null() } else { v.as_ptr() } }

pub trait CommandSubmitter
{
	fn submit_graphics_commands(&self, commands: &GraphicsCommandBuffersView, wait_for_execute: &[(&QueueFence, &PipelineStageFlag)],
		signal_on_complete: Option<&QueueFence>, signal_on_complete_host: Option<&Fence>) -> EngineResult<()>;
	fn submit_transfer_commands(&self, commands: &TransferCommandBuffersView, wait_for_execute: &[(&QueueFence, &PipelineStageFlag)],
		signal_on_complete: Option<&QueueFence>, signal_on_complete_host: Option<&Fence>) -> EngineResult<()>;
}
impl<InputNames: Eq + Copy + Ord> CommandSubmitter for Engine<InputNames>
{
	fn submit_graphics_commands(&self, commands: &GraphicsCommandBuffersView, wait_for_execute: &[(&QueueFence, &PipelineStageFlag)],
		signal_on_complete: Option<&QueueFence>, signal_on_complete_host: Option<&Fence>) -> EngineResult<()>
	{
		let signals_on_complete = signal_on_complete.into_iter().map(NativeHandleProvider::native).collect::<Vec<_>>();
		let wait_stages = wait_for_execute.into_iter().map(|&(_, s)| s.into_flag()).collect::<Vec<_>>();
		let wait_semaphores = wait_for_execute.into_iter().map(|&(q, _)| q.native()).collect::<Vec<_>>();

		unsafe { vkQueueSubmit(self.gi.device().graphics_queue, 1, &VkSubmitInfo
		{
			commandBufferCount: commands.len() as u32, pCommandBuffers: commands.as_ptr(),
			waitSemaphoreCount: wait_semaphores.len() as u32, pWaitSemaphores: wait_semaphores.as_ptr(), pWaitDstStageMask: as_ptr_emp(&wait_stages),
			signalSemaphoreCount: signals_on_complete.len() as u32, pSignalSemaphores: signals_on_complete.as_ptr(), .. Default::default()
		}, signal_on_complete_host.map(NativeHandleProvider::native).unwrap_or(zeroed())) }.into_result()
	}
	fn submit_transfer_commands(&self, commands: &TransferCommandBuffersView, wait_for_execute: &[(&QueueFence, &PipelineStageFlag)],
		signal_on_complete: Option<&QueueFence>, signal_on_complete_host: Option<&Fence>) -> EngineResult<()>
	{
		let signals_on_complete = signal_on_complete.into_iter().map(NativeHandleProvider::native).collect::<Vec<_>>();
		let wait_stages = wait_for_execute.into_iter().map(|&(_, s)| s.into_flag()).collect::<Vec<_>>();
		let wait_semaphores = wait_for_execute.into_iter().map(|&(q, _)| q.native()).collect::<Vec<_>>();

		unsafe { vkQueueSubmit(self.gi.device().transfer_queue, 1, &VkSubmitInfo
		{
			commandBufferCount: commands.len() as u32, pCommandBuffers: commands.as_ptr(),
			waitSemaphoreCount: wait_semaphores.len() as u32, pWaitSemaphores: wait_semaphores.as_ptr(), pWaitDstStageMask: as_ptr_emp(&wait_stages),
			signalSemaphoreCount: signals_on_complete.len() as u32, pSignalSemaphores: signals_on_complete.as_ptr(), .. Default::default()
		}, signal_on_complete_host.map(NativeHandleProvider::native).unwrap_or(zeroed())) }.into_result()
	}
}
/*
pub struct CommandSender<'a>(&'a Device);
unsafe impl<'a> Send for CommandSender<'a> {}
impl<'a> CommandSubmitter for CommandSender<'a>
{
	fn submit_graphics_commands<PS: PipelineStageFlag>(&self, commands: &GraphicsCommandBuffersView, wait_for_execute: &[(&QueueFence, PS)],
		signal_on_complete: Option<&QueueFence>, signal_on_complete_host: Option<&Fence>) -> EngineResult<()>
	{
		let signals_on_complete = signal_on_complete.into_iter().map(NativeHandleProvider::native).collect::<Vec<_>>();
		let wait_stages = wait_for_execute.into_iter().map(|&(_, s)| s.into()).collect::<Vec<_>>();
		let wait_semaphores = wait_for_execute.into_iter().map(|&(q, _)| q.native()).collect::<Vec<_>>();

		unsafe { vkQueueSubmit(self.0.graphics_queue, 1, &VkSubmitInfo
		{
			commandBufferCount: commands.len() as u32, pCommandBuffers: commands.as_ptr(),
			waitSemaphoreCount: wait_semaphores.len() as u32, pWaitSemaphores: wait_semaphores.as_ptr(), pWaitDstStageMask: as_ptr_emp(&wait_stages),
			signalSemaphoreCount: signals_on_complete.len() as u32, pSignalSemaphores: signals_on_complete.as_ptr(), .. Default::default()
		}, signal_on_complete_host.map(NativeHandleProvider::native).unwrap_or(zeroed())) }.into_result()
	}
	fn submit_transfer_commands<PS: PipelineStageFlag>(&self, commands: &TransferCommandBuffersView, wait_for_execute: &[(&QueueFence, PS)],
		signal_on_complete: Option<&QueueFence>, signal_on_complete_host: Option<&Fence>) -> EngineResult<()>
	{
		let signals_on_complete = signal_on_complete.into_iter().map(NativeHandleProvider::native).collect::<Vec<_>>();
		let wait_stages = wait_for_execute.into_iter().map(|&(_, s)| s.into()).collect::<Vec<_>>();
		let wait_semaphores = wait_for_execute.into_iter().map(|&(q, _)| q.native()).collect::<Vec<_>>();

		unsafe { vkQueueSubmit(self.0.transfer_queue, 1, &VkSubmitInfo
		{
			commandBufferCount: commands.len() as u32, pCommandBuffers: commands.as_ptr(),
			waitSemaphoreCount: wait_semaphores.len() as u32, pWaitSemaphores: wait_semaphores.as_ptr(), pWaitDstStageMask: as_ptr_emp(&wait_stages),
			signalSemaphoreCount: signals_on_complete.len() as u32, pSignalSemaphores: signals_on_complete.as_ptr(), .. Default::default()
		}, signal_on_complete_host.map(NativeHandleProvider::native).unwrap_or(zeroed())) }.into_result()
	}
}
*/

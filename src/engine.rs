// Interlude: Engine and EngineLogger

#![allow(dead_code)]

use super::internals::*;
use {std, log, vk};
use vk::ffi::*;
use ansi_term::*;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use libc::size_t;
use std::os::raw::*;
use std::ffi::CStr;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

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

pub struct DeviceFeatures(VkPhysicalDeviceFeatures);
impl DeviceFeatures
{
	pub fn new() -> Self
	{
		DeviceFeatures(VkPhysicalDeviceFeatures
		{
			geometryShader: true as VkBool32,
			.. Default::default()
		})
	}
	pub fn enable_multidraw_indirect(mut self) -> Self
	{
		self.0.multiDrawIndirect = true as VkBool32;
		self
	}
	pub fn enable_draw_indirect_first_instance(mut self) -> Self
	{
		self.0.drawIndirectFirstInstance = true as VkBool32;
		self
	}
	pub fn enable_block_texture_compression(mut self) -> Self
	{
		self.0.textureCompressionBC = true as VkBool32;
		self
	}
}

pub trait EngineExports<WS: WindowServer>
{
	fn get_window_server(&self) -> &Arc<WS>;
}
pub trait EngineCoreExports
{
	fn get_instance(&self) -> &Rc<vk::Instance>;
	fn get_device(&self) -> &DeviceExports;
	fn get_memory_type_index_for_device_local(&self) -> u32;
	fn get_memory_type_index_for_host_visible(&self) -> u32;
	fn get_compatible_memory_type_index(&self, bits: u32) -> Result<u32, EngineError>;
	fn is_optimized_debug_render_support(&self) -> bool;
}
// Core Functions in Engine
pub trait EngineCore : EngineCoreExports + CommandSubmitter
{
	// Factory Methods //
	fn create_fence(&self) -> Result<Fence, EngineError>;
	fn create_queue_fence(&self) -> Result<QueueFence, EngineError>;
	fn create_render_pass(&self, attachments: &[&AttachmentDesc], passes: &[&PassDesc], deps: &[&PassDependency]) -> Result<RenderPass, EngineError>;
	fn create_framebuffer(&self, mold: &RenderPass, attachments: &[&ImageView], form: &Size3) -> Result<Framebuffer, EngineError>;
	fn create_vertex_shader_from_asset(&self, asset_path: &str, entry_point: &str, vertex_bindings: &[VertexBinding], vertex_attributes: &[VertexAttribute])
		-> Result<ShaderProgram, EngineError>;
	fn create_geometry_shader_from_asset(&self, asset_path: &str, entry_point: &str) -> Result<ShaderProgram, EngineError>;
	fn create_fragment_shader_from_asset(&self, asset_path: &str, entry_point: &str) -> Result<ShaderProgram, EngineError>;
	fn create_pipeline_layout(&self, descriptor_sets: &[&DescriptorSetLayout], push_constants: &[PushConstantDesc]) -> Result<PipelineLayout, EngineError>;
	fn create_graphics_pipelines(&self, builders: &[&GraphicsPipelineBuilder]) -> Result<Vec<GraphicsPipeline>, EngineError>;
	fn create_double_buffer(&self, prealloc: &BufferPreallocator) -> Result<(DeviceBuffer, StagingBuffer), EngineError>;
	fn create_double_image(&self, prealloc: &ImagePreallocator) -> Result<(DeviceImage, Option<StagingImage>), EngineError>;
	fn create_descriptor_set_layout(&self, bindings: &[Descriptor]) -> Result<DescriptorSetLayout, EngineError>;
	fn create_sampler(&self, state: &SamplerState) -> Result<Sampler, EngineError>;
	fn create_image_view_1d(&self, res: &Rc<Image1D>, format: VkFormat, c_map: ComponentMapping, subres: ImageSubresourceRange)
		-> Result<ImageView1D, EngineError>;
	fn create_image_view_2d(&self, res: &Rc<Image2D>, format: VkFormat, c_map: ComponentMapping, subres: ImageSubresourceRange)
		-> Result<ImageView2D, EngineError>;
	fn create_image_view_3d(&self, res: &Rc<Image3D>, format: VkFormat, c_map: ComponentMapping, subres: ImageSubresourceRange)
		-> Result<ImageView3D, EngineError>;
	fn create_postprocess_vertex_shader_from_asset(&self, asset_path: &str, entry_point: &str) -> Result<ShaderProgram, EngineError>;

	// Allocation Methods //
	fn allocate_graphics_command_buffers(&self, count: usize) -> Result<GraphicsCommandBuffers, EngineError>;
	fn allocate_bundled_command_buffers(&self, count: usize) -> Result<BundledCommandBuffers, EngineError>;
	fn allocate_transfer_command_buffers(&self, count: usize) -> Result<TransferCommandBuffers, EngineError>;
	fn allocate_transient_transfer_command_buffers(&self, count: usize) -> Result<TransientTransferCommandBuffers, EngineError>;
	fn allocate_transient_graphics_command_buffers(&self, count: usize) -> Result<TransientGraphicsCommandBuffers, EngineError>;
	fn preallocate_all_descriptor_sets(&self, layouts: &[&DescriptorSetLayout]) -> Result<DescriptorSets, EngineError>;
	fn buffer_preallocate(&self, structure_sizes: &[(usize, BufferDataType)]) -> BufferPreallocator;

	// Asset Path //
	fn parse_asset(&self, asset_path: &str, extension: &str) -> std::ffi::OsString;
	fn _parse_asset(asset_base: &PathBuf, asset_path: &str, extension: &str) -> std::ffi::OsString;
	fn get_postprocess_vsh(&self, require_uv: bool) -> &ShaderProgram;

	// Device Configurations/Operations //
	fn update_descriptors(&self, write_infos: &[DescriptorSetWriteInfo]);
	fn wait_device(&self) -> Result<(), EngineError>;
	fn submit_transient_commands(&self, tt: Option<TransientTransferCommandBuffers>, gt: Option<TransientGraphicsCommandBuffers>) -> Result<(), EngineError>;

	// Detached Functions //
	fn new_command_sender(&self) -> CommandSender;
	fn graphics_queue_ref(&self) -> &vk::Queue;
}
pub struct Engine<WS: WindowServer, IS: InputSystem<InputNames>, InputNames: PartialEq + Eq + Clone + Copy + std::hash::Hash>
{
	window_system: Arc<WS>, input_system: Arc<RwLock<IS>>,
	instance: Rc<vk::Instance>, #[allow(dead_code)] debug_callback: vk::DebugReportCallback,
	memory_types: VkPhysicalDeviceMemoryProperties,
	device: Device, pools: CommandPool, pipeline_cache: Rc<vk::PipelineCache>,
	asset_dir: std::path::PathBuf,
	physical_device_limits: VkPhysicalDeviceLimits,
	memory_type_index_for_device_local: u32, memory_type_index_for_host_visible: u32,
	optimized_debug_render: bool,
	// CommonResources //
	pub postprocess_vsh: ShaderProgram, pub postprocess_vsh_nouv: ShaderProgram,
	// Phantom Data //
	ph: std::marker::PhantomData<InputNames>
}
unsafe impl<WS: WindowServer, IS: InputSystem<InputNames>, InputNames: PartialEq + Eq + Clone + Copy + std::hash::Hash>
	Send for Engine<WS, IS, InputNames> {}
impl<WS: WindowServer, IS: InputSystem<InputNames>, InputNames: PartialEq + Eq + Clone + Copy + std::hash::Hash>
	Drop for Engine<WS, IS, InputNames>
{
	fn drop(&mut self) { self.device.wait_for_idle().unwrap(); }
}
impl<WS: WindowServer, IS: InputSystem<InputNames>, InputNames: PartialEq + Eq + Clone + Copy + std::hash::Hash>
	EngineExports<WS> for Engine<WS, IS, InputNames>
{
	fn get_window_server(&self) -> &Arc<WS> { &self.window_system }
}
impl<WS: WindowServer, IS: InputSystem<InputNames>, InputNames: PartialEq + Eq + Clone + Copy + std::hash::Hash>
	EngineCoreExports for Engine<WS, IS, InputNames>
{
	fn get_instance(&self) -> &Rc<vk::Instance> { &self.instance }
	fn get_device(&self) -> &DeviceExports { &self.device }
	fn get_memory_type_index_for_device_local(&self) -> u32 { self.memory_type_index_for_device_local }
	fn get_memory_type_index_for_host_visible(&self) -> u32 { self.memory_type_index_for_host_visible }
	fn get_compatible_memory_type_index(&self, bits: u32) -> Result<u32, EngineError>
	{
		let least_index = bits.trailing_zeros();
		if least_index >= self.memory_types.memoryTypeCount { Err(EngineError::GenericError("Argument does not contain any bits")) }
		else { Ok(least_index as u32) }
	}
	fn is_optimized_debug_render_support(&self) -> bool { self.optimized_debug_render }
}
// For XServer
#[cfg(unix)] impl<InputNames: PartialEq + Eq + Clone + Copy + std::hash::Hash>
	Engine<super::linux::XServer, super::input::UnixInputSystem<InputNames>, InputNames>
{
	pub fn new<StrT: AsRef<Path>>(app_name: &str, app_version: u32, asset_base: Option<StrT>, extra_features: DeviceFeatures) -> Result<Self, EngineError>
	{
		Engine::new_with_window_system(app_name, app_version, asset_base, extra_features, "VK_KHR_xcb_surface", super::linux::connect_xserver)
	}
}
// For Win32Server
#[cfg(windows)] impl<InputNames: PartialEq + Eq + Clone + Copy + std::hash::Hash>
	Engine<super::win32::Win32Server, super::input::Win32InputSystem<InputNames>, InputNames>
{
	pub fn new<StrT: AsRef<Path>>(app_name: &str, app_version: u32, asset_base: Option<StrT>, extra_features: DeviceFeatures) -> Result<Self, EngineError>
	{
		Engine::new_with_window_system(app_name, app_version, asset_base, extra_features, "VK_KHR_win32_surface", super::win32::connect_win32_server)
	}
}
// For any WindowSystems
impl<WS: WindowServer, IS: InputSystem<InputNames>, InputNames: PartialEq + Eq + Clone + Copy + std::hash::Hash> Engine<WS, IS, InputNames>
{
	pub fn window_system_ref(&self) -> &Arc<WS> { &self.window_system }
	pub fn input_system_ref(&self) -> &Arc<RwLock<IS>> { &self.input_system }

	pub fn process_messages(&self) -> bool
	{
		self.window_system.process_events() == ApplicationState::Continue
	}
	pub fn process_all_messages(&self)
	{
		self.window_system.process_all_events()
	}
	pub fn create_render_window(&self, size: &Size2, title: &str) -> Result<Box<RenderWindow>, EngineError>
	{
		info!(target: "Interlude", "Creating Render Window \"{}\" ({}x{})", title, size.0, size.1);
		Window::<WS::NativeWindowT>::create_unresizable(self, size, title).map(|x| x as Box<RenderWindow>)
	}

	fn diagnose_adapter(server_con: &WS, adapter: &vk::PhysicalDevice, queue_index: u32)
	{
		// Feature Check //
		let features = adapter.features();
		info!(target: "Interlude::DiagAdapter", "adapter features");
		info!(target: "Interlude::DiagAdapter", "-- independentBlend: {}", bool_to_str(features.independentBlend));
		info!(target: "Interlude::DiagAdapter", "-- geometryShader: {}", bool_to_str(features.geometryShader));
		info!(target: "Interlude::DiagAdapter", "-- multiDrawIndirect: {}", bool_to_str(features.multiDrawIndirect));
		info!(target: "Interlude::DiagAdapter", "-- drawIndirectFirstInstance: {}", bool_to_str(features.drawIndirectFirstInstance));
		info!(target: "Interlude::DiagAdapter", "-- shaderTessellationAndGeometryPointSize: {}", bool_to_str(features.shaderTessellationAndGeometryPointSize));
		info!(target: "Interlude::DiagAdapter", "-- depthClamp: {}", bool_to_str(features.depthClamp));
		info!(target: "Interlude::DiagAdapter", "-- depthBiasClamp: {}", bool_to_str(features.depthBiasClamp));
		info!(target: "Interlude::DiagAdapter", "-- wideLines: {}", bool_to_str(features.wideLines));
		info!(target: "Interlude::DiagAdapter", "-- alphaToOne: {}", bool_to_str(features.alphaToOne));
		info!(target: "Interlude::DiagAdapter", "-- multiViewport: {}", bool_to_str(features.multiViewport));
		info!(target: "Interlude::DiagAdapter", "-- shaderCullDistance: {}", bool_to_str(features.shaderCullDistance));
		info!(target: "Interlude::DiagAdapter", "-- shaderClipDistance: {}", bool_to_str(features.shaderClipDistance));
		info!(target: "Interlude::DiagAdapter", "-- shaderResourceResidency: {}", bool_to_str(features.shaderResourceResidency));
		// if features.depthClamp == false as VkBool32 { panic!("DepthClamp Feature is required in device"); }

		// Vulkan and XCB Integration Check //
		if !server_con.is_vk_presentation_support(adapter, queue_index) { panic!("Vulkan Presentation is not supported by window system"); }
	}
	fn new_with_window_system<ConF, StrT>(app_name: &str, app_version: u32, asset_base: Option<StrT>, extra_features: DeviceFeatures,
		surface_ex_name: &str, connect_f: ConF) -> Result<Self, EngineError> where ConF: FnOnce() -> Result<Arc<WS>, EngineError>, StrT: AsRef<Path>
	{
		// Setup Engine Logger //
		log::set_logger(|max_log_level| { max_log_level.set(log::LogLevelFilter::Info); Box::new(EngineLogger) }).unwrap();
		info!(target: "Interlude", "Initializing Engine...");

		let window_server = try!(connect_f());

		let instance = try!(vk::Instance::new(app_name, app_version, "Interlude Computer-Graphics Engine", VK_MAKE_VERSION!(0, 0, 1),
			&["VK_LAYER_LUNARG_standard_validation"], &["VK_KHR_surface", surface_ex_name, "VK_EXT_debug_report"]).map(|x| Rc::new(x)));
		let dbg_callback = try!(vk::DebugReportCallback::new(&instance, device_report_callback));
		let adapter = try!(instance.adapters().map_err(|e| EngineError::from(e))
			.and_then(|aa| aa.into_iter().next().ok_or(EngineError::GenericError("PhysicalDevices are not found")))
			.map(|a| Rc::new(vk::PhysicalDevice::from(a, &instance))));
		let features = adapter.features();
		let (odr, extra_features) = if features.multiDrawIndirect != 0 && features.drawIndirectFirstInstance != 0
		{
			// Required for optimized debug rendering
			(true, extra_features.enable_multidraw_indirect().enable_draw_indirect_first_instance())
		}
		else
		{
			info!(target: "Interlude::DiagAdapter", "MultiDrawIndirect or DrawIndirectFirstInstance features are not available.");
			(false, extra_features)
		};
		let device =
		{
			let queue_family_properties = adapter.queue_family_properties();
			let graphics_qf = try!(queue_family_properties.iter().enumerate().find(|&(_, fp)| (fp.queueFlags & VK_QUEUE_GRAPHICS_BIT) != 0)
				.map(|(i, _)| i as u32).ok_or(EngineError::GenericError("Unable to find Graphics Queue")));
			let transfer_qf = queue_family_properties.iter().enumerate().filter(|&(i, _)| i as u32 != graphics_qf)
				.find(|&(_, fp)| (fp.queueFlags & VK_QUEUE_TRANSFER_BIT) != 0).map(|(i, _)| i as u32);
			Self::diagnose_adapter(&*window_server, &adapter, graphics_qf);
			let device_features = extra_features.0;
			try!(Device::new(&adapter, device_features, graphics_qf, transfer_qf, &queue_family_properties[graphics_qf as usize]))
		};
		let pools = try!(CommandPool::new(&device));
		let pipeline_cache = Rc::new(try!(vk::PipelineCache::new_empty(&device, &[])));

		let memory_types = adapter.memory_properties();
		let mt_index_for_device_local = try!(memory_types.memoryTypes[..memory_types.memoryTypeCount as usize].iter()
			.enumerate().find(|&(_, &VkMemoryType(flags, _))| (flags & VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT) != 0)
			.map(|(i, _)| i as u32).ok_or(EngineError::GenericError("Device Local Memory is not found")));
		let mt_index_for_host_visible = try!(memory_types.memoryTypes[..memory_types.memoryTypeCount as usize].iter()
			.enumerate().find(|&(_, &VkMemoryType(flags, _))| (flags & VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT) != 0)
			.map(|(i, _)| i as u32).ok_or(EngineError::GenericError("Host Visible Memory is not found")));

		info!(target: "Interlude", "MemoryType[Device Local] Index = {}: {:?}", mt_index_for_device_local, mtflags_decomposite(memory_types.memoryTypes[mt_index_for_device_local as usize].0));
		info!(target: "Interlude", "MemoryType[Host Visible] Index = {}: {:?}", mt_index_for_host_visible, mtflags_decomposite(memory_types.memoryTypes[mt_index_for_host_visible as usize].0));

		let asset_base = asset_base.map(|b| b.as_ref().to_path_buf())
			.unwrap_or(std::env::current_exe().unwrap().parent().map(|x| x.to_path_buf()).unwrap()).join("assets");
		let (ppvsh, ppvsh_nouv) = try!(Self::init_common_resources(&device, &asset_base));
		Ok(Engine
		{
			window_system: window_server, input_system: Arc::new(RwLock::new(try!(IS::new()))),
			instance: instance, debug_callback: dbg_callback, device: device, pools: pools,
			memory_types: memory_types,
			pipeline_cache: pipeline_cache, asset_dir: asset_base,
			physical_device_limits: adapter.properties().limits,
			memory_type_index_for_device_local: mt_index_for_device_local,
			memory_type_index_for_host_visible: mt_index_for_host_visible,
			optimized_debug_render: odr,
			postprocess_vsh: ppvsh, postprocess_vsh_nouv: ppvsh_nouv,
			ph: std::marker::PhantomData
		})
	}
	fn init_common_resources(device: &Rc<vk::Device>, asset_base: &PathBuf) -> Result<(ShaderProgram, ShaderProgram), EngineError>
	{
		info!(target: "Interlude::CommonResource", "Loading Vertex Shader for PostProcessing...");

		std::fs::File::open(Self::_parse_asset(asset_base, "engine.shaders.PostProcessVertex", "spv")).map_err(EngineError::from).and_then(|mut fp|
		{
			let mut bin = Vec::new();
			fp.read_to_end(&mut bin).map(move |_| bin).map_err(EngineError::from)
		}).and_then(|b| vk::ShaderModule::new(device, &b).map_err(EngineError::from)).map(|m|
			ShaderProgram::new_vertex(m, "main", &[VertexBinding::PerVertex(std::mem::size_of::<PosUV>() as u32)], &[VertexAttribute(0, VkFormat::R32G32B32A32_SFLOAT, 0)])
		).and_then(|ppvsh| std::fs::File::open(Self::_parse_asset(asset_base, "engine.shaders.PostProcessVertexNoUV", "spv")).map_err(EngineError::from).and_then(|mut fp|
			{
				let mut bin = Vec::new();
				fp.read_to_end(&mut bin).map(move |_| bin).map_err(EngineError::from)
			}).and_then(|b| vk::ShaderModule::new(device, &b).map_err(EngineError::from))
			.map(move |m| (ppvsh, ShaderProgram::new_vertex(m, "main", &[VertexBinding::PerVertex(std::mem::size_of::<PosUV>() as u32)], &[VertexAttribute(0, VkFormat::R32G32B32A32_SFLOAT, 0)])))
		)
	}
}
// For WindowServer independent parts
impl<WS: WindowServer, IS: InputSystem<InputNames>, InputNames: PartialEq + Eq + Clone + Copy + std::hash::Hash>
	EngineCore for Engine<WS, IS, InputNames>
{
	fn create_fence(&self) -> Result<Fence, EngineError>
	{
		vk::Fence::new(&self.device).map(Fence::new).map_err(EngineError::from)
	}
	fn create_queue_fence(&self) -> Result<QueueFence, EngineError>
	{
		vk::Semaphore::new(&self.device).map(QueueFence::new).map_err(EngineError::from)
	}
	fn create_render_pass(&self, attachments: &[&AttachmentDesc], passes: &[&PassDesc], deps: &[&PassDependency]) -> Result<RenderPass, EngineError>
	{
		let attachments_native = attachments.iter().map(|&x| x.into()).collect::<Vec<_>>();
		let subpasses_native = passes.iter().map(|&x| x.into()).collect::<Vec<_>>();
		let deps_native = deps.iter().map(|&x| x.into()).collect::<Vec<_>>();
		vk::RenderPass::new(&self.device, &VkRenderPassCreateInfo
		{
			sType: VkStructureType::RenderPassCreateInfo, pNext: std::ptr::null(), flags: 0,
			attachmentCount: attachments_native.len() as u32, pAttachments: attachments_native.as_ptr(),
			subpassCount: subpasses_native.len() as u32, pSubpasses: subpasses_native.as_ptr(),
			dependencyCount: deps_native.len() as u32, pDependencies: deps_native.as_ptr()
		}).map(RenderPass::new).map_err(EngineError::from)
	}
	fn create_framebuffer(&self, mold: &RenderPass, attachments: &[&ImageView], form: &Size3) -> Result<Framebuffer, EngineError>
	{
		let attachments_native: Vec<_> = attachments.into_iter().map(|x| x.get_native()).collect();
		let &Size3(width, height, layers) = form;
		let info = VkFramebufferCreateInfo
		{
			sType: VkStructureType::FramebufferCreateInfo, pNext: std::ptr::null(), flags: 0,
			renderPass: ***mold.get_internal(),
			attachmentCount: attachments_native.len() as u32, pAttachments: attachments_native.as_ptr(),
			width: width, height: height, layers: layers
		};
		vk::Framebuffer::new(&self.device, &info).map(|f| Framebuffer::new(f, mold.get_internal(), VkExtent2D(width, height))).map_err(EngineError::from)
	}
	fn allocate_graphics_command_buffers(&self, count: usize) -> Result<GraphicsCommandBuffers, EngineError>
	{
		self.pools.graphics().allocate(VkCommandBufferLevel::Primary, count).map_err(EngineError::from)
			.map(|v| GraphicsCommandBuffers::new(self.pools.graphics(), v))
	}
	fn allocate_bundled_command_buffers(&self, count: usize) -> Result<BundledCommandBuffers, EngineError>
	{
		self.pools.graphics().allocate(VkCommandBufferLevel::Secondary, count).map_err(EngineError::from)
			.map(|v| BundledCommandBuffers::new(self.pools.graphics(), v))
	}
	fn allocate_transfer_command_buffers(&self, count: usize) -> Result<TransferCommandBuffers, EngineError>
	{
		self.pools.transfer().allocate(VkCommandBufferLevel::Primary, count).map_err(EngineError::from)
			.map(|v| TransferCommandBuffers::new(self.pools.transfer(), v))
	}
	fn allocate_transient_transfer_command_buffers(&self, count: usize) -> Result<TransientTransferCommandBuffers, EngineError>
	{
		self.pools.transient().allocate(VkCommandBufferLevel::Primary, count).map_err(EngineError::from)
			.map(|v| TransientTransferCommandBuffers::new(self.pools.transient(), self.device.get_transfer_queue(), v))
	}
	fn allocate_transient_graphics_command_buffers(&self, count: usize) -> Result<TransientGraphicsCommandBuffers, EngineError>
	{
		self.pools.transient_graphics().allocate(VkCommandBufferLevel::Primary, count).map_err(EngineError::from)
			.map(|v| TransientGraphicsCommandBuffers::new(self.pools.transient_graphics(), self.device.get_graphics_queue(), v))
	}
	fn create_vertex_shader_from_asset(&self, asset_path: &str, entry_point: &str, vertex_bindings: &[VertexBinding], vertex_attributes: &[VertexAttribute])
		-> Result<ShaderProgram, EngineError>
	{
		let entity_path = self.parse_asset(asset_path, "spv");
		info!(target: "Interlude", "Loading Vertex Shader {:?}", entity_path);
		std::fs::File::open(entity_path).map_err(EngineError::from).and_then(|mut fp|
		{
			let mut bin: Vec<u8> = Vec::new();
			fp.read_to_end(&mut bin).map(move |_| bin).map_err(EngineError::from)
		}).and_then(|b| vk::ShaderModule::new(self.device.get_internal(), &b).map_err(EngineError::from))
		.map(|m| ShaderProgram::new_vertex(m, entry_point, vertex_bindings, vertex_attributes))
	}
	fn create_geometry_shader_from_asset(&self, asset_path: &str, entry_point: &str) -> Result<ShaderProgram, EngineError>
	{
		let entity_path = self.parse_asset(asset_path, "spv");
		info!(target: "Interlude", "Loading Geometry Shader {:?}", entity_path);
		std::fs::File::open(entity_path).map_err(EngineError::from).and_then(|mut fp|
		{
			let mut bin: Vec<u8> = Vec::new();
			fp.read_to_end(&mut bin).map(move |_| bin).map_err(EngineError::from)
		}).and_then(|b| vk::ShaderModule::new(self.device.get_internal(), &b).map_err(EngineError::from))
		.map(|m| ShaderProgram::new_geometry(m, entry_point))
	}
	fn create_fragment_shader_from_asset(&self, asset_path: &str, entry_point: &str) -> Result<ShaderProgram, EngineError>
	{
		let entity_path = self.parse_asset(asset_path, "spv");
		info!(target: "Interlude", "Loading Fragment Shader {:?}", entity_path);
		std::fs::File::open(entity_path).map_err(EngineError::from).and_then(|mut fp|
		{
			let mut bin: Vec<u8> = Vec::new();
			fp.read_to_end(&mut bin).map(|_| bin).map_err(EngineError::from)
		}).and_then(|b| vk::ShaderModule::new(self.device.get_internal(), &b).map_err(EngineError::from))
		.map(|m| ShaderProgram::new_fragment(m, entry_point))
	}
	fn create_pipeline_layout(&self, descriptor_sets: &[&DescriptorSetLayout], push_constants: &[PushConstantDesc]) -> Result<PipelineLayout, EngineError>
	{
		vk::PipelineLayout::new(self.device.get_internal(),
			&descriptor_sets.into_iter().map(|x| **x.get_internal()).collect::<Vec<_>>(),
			&push_constants.into_iter().map(|x| x.into()).collect::<Vec<_>>()).map(PipelineLayout::new).map_err(EngineError::from)
	}
	fn create_graphics_pipelines(&self, builders: &[&GraphicsPipelineBuilder]) -> Result<Vec<GraphicsPipeline>, EngineError>
	{
		let builder_into_natives = builders.into_iter().map(|&x| x.into()).collect::<Vec<IntoNativeGraphicsPipelineCreateInfoStruct>>();
		vk::Pipeline::new_graphics(self.device.get_internal(), Some(&self.pipeline_cache),
			&builder_into_natives.iter().map(|x| x.into()).collect::<Vec<_>>())
			.map(|v| v.into_iter().map(GraphicsPipeline::new).collect::<Vec<_>>()).map_err(EngineError::from)
	}
	fn create_double_buffer(&self, prealloc: &BufferPreallocator) -> Result<(DeviceBuffer, StagingBuffer), EngineError>
	{
		(DeviceBuffer::new(self, prealloc.total_size() as VkDeviceSize, prealloc.get_usage()), StagingBuffer::new(self, prealloc.total_size() as VkDeviceSize)).flatten()
	}
	fn create_double_image(&self, prealloc: &ImagePreallocator) -> Result<(DeviceImage, Option<StagingImage>), EngineError>
	{
		let image1 = prealloc.dim1_images().iter().map(|desc| Image1D::new(self, desc.get_internal())).collect::<Result<Vec<_>, _>>();
		let image2 = prealloc.dim2_images().iter().map(|desc| Image2D::new(self, desc.get_internal())).collect::<Result<Vec<_>, _>>();
		let image3 = prealloc.dim3_images().iter().map(|desc| Image3D::new(self, desc.get_internal())).collect::<Result<Vec<_>, _>>();
		let linear_image1 = prealloc.dim1_images().iter().filter(|desc| !desc.is_device_resource()).map(|desc| desc.get_internal())
			.map(|desc| LinearImage2D::new(self, Size2(desc.extent.0, 1), desc.format)).collect::<Result<Vec<_>, EngineError>>();
		let linear_image2 = prealloc.dim2_images().iter().filter(|desc| !desc.is_device_resource()).map(|desc| desc.get_internal())
			.map(|desc| LinearImage2D::new(self, Size2(desc.extent.0, desc.extent.1), desc.format)).collect::<Result<Vec<_>, EngineError>>();
		let linear_images = (linear_image1, linear_image2).flatten().map(|(l1, l2)| l1.into_iter().chain(l2.into_iter()).collect::<Vec<_>>());

		(image1, image2, image3, linear_images).flatten().and_then(|(i1, i2, i3, l)| DeviceImage::new(self, i1, i2, i3).and_then(|dev| if !l.is_empty()
		{
			StagingImage::new(self, l).map(|stg| (dev, Some(stg)))
		} else { Ok((dev, None)) }))
	}
	fn create_descriptor_set_layout(&self, bindings: &[Descriptor]) -> Result<DescriptorSetLayout, EngineError>
	{
		let native = bindings.into_iter().enumerate().map(|(i, x)| x.into_binding(i as u32)).collect::<Vec<_>>();
		vk::DescriptorSetLayout::new(self.device.get_internal(), &native)
			.map(|d| DescriptorSetLayout::new(d, bindings)).map_err(EngineError::from)
	}
	fn preallocate_all_descriptor_sets(&self, layouts: &[&DescriptorSetLayout]) -> Result<DescriptorSets, EngineError>
	{
		let set_count = layouts.len();
		let (uniform_total, combined_sampler_total, ia_total) = layouts.iter().map(|x| x.descriptors().into_iter().fold((0, 0, 0), |(u, cs, ia), desc| match desc
		{
			&Descriptor::Uniform(n, _) => (u + n, cs, ia),
			&Descriptor::CombinedSampler(n, _) => (u, cs + n, ia),
			&Descriptor::InputAttachment(n, _) => (u, cs, ia + n)
		})).fold((0, 0, 0), |(u, cs, ia), (u2, cs2, ia2)| (u + u2, cs + cs2, ia + ia2));
		let pool_sizes =
			[Descriptor::Uniform(uniform_total, vec![]), Descriptor::CombinedSampler(combined_sampler_total, vec![]), Descriptor::InputAttachment(ia_total, vec![])]
			.into_iter().filter(|&desc| desc.count() != 0).map(|desc| desc.into_pool_size()).collect::<Vec<_>>();

		vk::DescriptorPool::new(self.device.get_internal(), set_count, &pool_sizes).and_then(|pool|
		pool.allocate(&layouts.into_iter().map(|x| **x.get_internal()).collect::<Vec<_>>())
			.map(|sets| DescriptorSets::new(pool, sets))).map_err(EngineError::from)
	}
	fn create_sampler(&self, state: &SamplerState) -> Result<Sampler, EngineError>
	{
		Sampler::new(self, &state.into())
	}
	fn create_image_view_1d(&self, res: &Rc<Image1D>, format: VkFormat, c_map: ComponentMapping, subres: ImageSubresourceRange) -> Result<ImageView1D, EngineError>
	{
		ImageView1D::new(self, res, format, c_map, subres)
	}
	fn create_image_view_2d(&self, res: &Rc<Image2D>, format: VkFormat, c_map: ComponentMapping, subres: ImageSubresourceRange) -> Result<ImageView2D, EngineError>
	{
		ImageView2D::new(self, res, format, c_map, subres)
	}
	fn create_image_view_3d(&self, res: &Rc<Image3D>, format: VkFormat, c_map: ComponentMapping, subres: ImageSubresourceRange) -> Result<ImageView3D, EngineError>
	{
		ImageView3D::new(self, res, format, c_map, subres)
	}
	fn wait_device(&self) -> Result<(), EngineError> { self.device.get_internal().wait_for_idle().map_err(EngineError::from) }

	fn create_postprocess_vertex_shader_from_asset(&self, asset_path: &str, entry_point: &str) -> Result<ShaderProgram, EngineError>
	{
		self.create_vertex_shader_from_asset(asset_path, entry_point,
			&[VertexBinding::PerVertex(std::mem::size_of::<PosUV>() as u32)], &[VertexAttribute(0, VkFormat::R32G32B32A32_SFLOAT, 0)])
	}

	fn parse_asset(&self, asset_path: &str, extension: &str) -> std::ffi::OsString
	{
		Self::_parse_asset(&self.asset_dir, asset_path, extension)
	}
	fn _parse_asset(asset_base: &PathBuf, asset_path: &str, extension: &str) -> std::ffi::OsString
	{
		asset_base.join(asset_path.replace(".", "/")).with_extension(extension).into()
	}

	fn buffer_preallocate(&self, structure_sizes: &[(usize, BufferDataType)]) -> BufferPreallocator
	{
		let uniform_alignment = self.physical_device_limits.minUniformBufferOffsetAlignment as usize;
		let usage_flags = structure_sizes.iter().fold(0, |flags_accum, &(_, data_type)| match data_type
		{
			BufferDataType::Vertex => flags_accum | VK_BUFFER_USAGE_VERTEX_BUFFER_BIT,
			BufferDataType::Index => flags_accum | VK_BUFFER_USAGE_INDEX_BUFFER_BIT,
			BufferDataType::Uniform => flags_accum | VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT,
			BufferDataType::IndirectCallParam => flags_accum | VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT
		});
		let offsets = structure_sizes.into_iter().chain(&[(0, BufferDataType::Vertex)]).scan(0usize, |offset_accum, &(size, data_type)|
		{
			let current = match data_type
			{
				BufferDataType::Vertex | BufferDataType::Index | BufferDataType::IndirectCallParam => *offset_accum,
				BufferDataType::Uniform => ((*offset_accum as f64 / uniform_alignment as f64).ceil() as usize) * uniform_alignment as usize
			};
			*offset_accum = current + size;
			Some(current)
		}).collect::<Vec<_>>();

		info!(target: "Interlude::BufferPreallocator", "Preallocation Results: ");
		info!(target: "Interlude::BufferPreallocator", "-- Minimum Alignment for Uniform Buffer: {} bytes", uniform_alignment);
		info!(target: "Interlude::BufferPreallocator", "-- Preallocated Offsets: {:?}", offsets);

		BufferPreallocator::new(usage_flags, offsets)
	}
	fn update_descriptors(&self, write_infos: &[DescriptorSetWriteInfo])
	{
		let write_infos_native_interp = write_infos.into_iter().map(|x| Into::<IntoWriteDescriptorSetNativeStruct>::into(x)).collect::<Vec<_>>();
		let write_infos_native = write_infos_native_interp.iter().map(|x| Into::<VkWriteDescriptorSet>::into(x)).collect::<Vec<_>>();
		unsafe { vkUpdateDescriptorSets(***self.device.get_internal(), write_infos_native.len() as u32, write_infos_native.as_ptr(),
			0, std::ptr::null()) };
	}

	fn submit_transient_commands(&self, tt: Option<TransientTransferCommandBuffers>, gt: Option<TransientGraphicsCommandBuffers>) -> Result<(), EngineError>
	{
		if let &Some(ref t) = &tt
		{
			let sub = VkSubmitInfo
			{
				sType: VkStructureType::SubmitInfo, pNext: std::ptr::null(),
				commandBufferCount: t.len() as u32, pCommandBuffers: t.as_ptr(),
				waitSemaphoreCount: 0, pWaitSemaphores: std::ptr::null(), pWaitDstStageMask: std::ptr::null(),
				signalSemaphoreCount: 0, pSignalSemaphores: std::ptr::null()
			};
			try!(self.device.get_transfer_queue().submit(&[sub], None));
		}
		if let &Some(ref g) = &gt
		{
			let sub = VkSubmitInfo
			{
				sType: VkStructureType::SubmitInfo, pNext: std::ptr::null(),
				commandBufferCount: g.len() as u32, pCommandBuffers: g.as_ptr(),
				waitSemaphoreCount: 0, pWaitSemaphores: std::ptr::null(), pWaitDstStageMask: std::ptr::null(),
				signalSemaphoreCount: 0, pSignalSemaphores: std::ptr::null()
			};
			try!(self.device.get_graphics_queue().submit(&[sub], None));
		}
		if tt.is_some() { try!(self.device.get_transfer_queue().wait_for_idle()); }
		if gt.is_some() { try!(self.device.get_graphics_queue().wait_for_idle()); }
		Ok(())
	}

	fn new_command_sender(&self) -> CommandSender { CommandSender(&self.device) }
	fn graphics_queue_ref(&self) -> &vk::Queue { self.device.get_graphics_queue() }

	fn get_postprocess_vsh(&self, require_uv: bool) -> &ShaderProgram { if require_uv { &self.postprocess_vsh } else { &self.postprocess_vsh_nouv } }
}

unsafe extern "system" fn device_report_callback(flags: VkDebugReportFlagsEXT, object_type: VkDebugReportObjectTypeEXT, _: u64,
	_: size_t, message_code: i32, _: *const c_char, message: *const c_char, _: *mut c_void) -> VkBool32
{
	if (flags & VK_DEBUG_REPORT_ERROR_BIT_EXT) != 0
	{
		error!(target: format!("Vulkan DebugCall [{:?}]", object_type).as_str(), "({}){}", message_code, CStr::from_ptr(message).to_str().unwrap());
	}
	else if (flags & VK_DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT) != 0
	{
		warn!(target: format!("Vulkan PerformanceDebug [{:?}]", object_type).as_str(), "({}){}", message_code, CStr::from_ptr(message).to_str().unwrap());
	}
	else if (flags & VK_DEBUG_REPORT_WARNING_BIT_EXT) != 0
	{
		warn!(target: format!("Vulkan DebugCall [{:?}]", object_type).as_str(), "({}){}", message_code, CStr::from_ptr(message).to_str().unwrap());
	}
	else
	{
		info!(target: format!("Vulkan DebugCall [{:?}]", object_type).as_str(), "({}){}", message_code, CStr::from_ptr(message).to_str().unwrap());
	}
	false as VkBool32
}

pub trait CommandSubmitter
{
	fn submit_graphics_commands(&self, commands: &GraphicsCommandBuffersView, wait_for_execute: &[(&QueueFence, VkPipelineStageFlags)],
		signal_on_complete: Option<&QueueFence>, signal_on_complete_host: Option<&Fence>) -> Result<(), EngineError>;
	fn submit_transfer_commands(&self, commands: &TransferCommandBuffersView, wait_for_execute: &[(&QueueFence, VkPipelineStageFlags)],
		signal_on_complete: Option<&QueueFence>, signal_on_complete_host: Option<&Fence>) -> Result<(), EngineError>;
}
impl<WS: WindowServer, IS: InputSystem<InputNames>, InputNames: PartialEq + Eq + Clone + Copy + std::hash::Hash>
	CommandSubmitter for Engine<WS, IS, InputNames>
{
	fn submit_graphics_commands(&self, commands: &GraphicsCommandBuffersView, wait_for_execute: &[(&QueueFence, VkPipelineStageFlags)],
		signal_on_complete: Option<&QueueFence>, signal_on_complete_host: Option<&Fence>) -> Result<(), EngineError>
	{
		let signals_on_complete = signal_on_complete.map(|q| vec![**q.get_internal()]).unwrap_or(vec![]);
		let wait_stages = if wait_for_execute.is_empty() { vec![VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT] }
		else { wait_for_execute.into_iter().map(|&(_, s)| s).collect::<Vec<_>>() };
		let wait_semaphores = wait_for_execute.into_iter().map(|&(q, _)| **q.get_internal()).collect::<Vec<_>>();

		let subinfo = VkSubmitInfo
		{
			sType: VkStructureType::SubmitInfo, pNext: std::ptr::null(),
			commandBufferCount: commands.len() as u32, pCommandBuffers: commands.as_ptr(),
			waitSemaphoreCount: wait_semaphores.len() as u32, pWaitSemaphores: wait_semaphores.as_ptr(), pWaitDstStageMask: wait_stages.as_ptr(),
			signalSemaphoreCount: signals_on_complete.len() as u32, pSignalSemaphores: signals_on_complete.as_ptr()
		};
		self.device.get_graphics_queue().submit(&[subinfo], signal_on_complete_host.map(|x| x.get_internal())).map_err(EngineError::from)
	}
	fn submit_transfer_commands(&self, commands: &TransferCommandBuffersView, wait_for_execute: &[(&QueueFence, VkPipelineStageFlags)],
		signal_on_complete: Option<&QueueFence>, signal_on_complete_host: Option<&Fence>) -> Result<(), EngineError>
	{
		let signals_on_complete = signal_on_complete.map(|q| vec![**q.get_internal()]).unwrap_or(vec![]);
		let wait_stages = if wait_for_execute.is_empty() { vec![VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT] }
		else { wait_for_execute.into_iter().map(|&(_, s)| s).collect::<Vec<_>>() };
		let wait_semaphores = wait_for_execute.into_iter().map(|&(q, _)| **q.get_internal()).collect::<Vec<_>>();

		let subinfo = VkSubmitInfo
		{
			sType: VkStructureType::SubmitInfo, pNext: std::ptr::null(),
			commandBufferCount: commands.len() as u32, pCommandBuffers: commands.as_ptr(),
			waitSemaphoreCount: wait_semaphores.len() as u32, pWaitSemaphores: wait_semaphores.as_ptr(), pWaitDstStageMask: wait_stages.as_ptr(),
			signalSemaphoreCount: signals_on_complete.len() as u32, pSignalSemaphores: signals_on_complete.as_ptr()
		};
		self.device.get_transfer_queue().submit(&[subinfo], signal_on_complete_host.map(|x| x.get_internal())).map_err(EngineError::from)
	}
}
pub struct CommandSender<'a>(&'a Device);
unsafe impl<'a> Send for CommandSender<'a> {}
impl<'a> CommandSubmitter for CommandSender<'a>
{
	fn submit_graphics_commands(&self, commands: &GraphicsCommandBuffersView, wait_for_execute: &[(&QueueFence, VkPipelineStageFlags)],
		signal_on_complete: Option<&QueueFence>, signal_on_complete_host: Option<&Fence>) -> Result<(), EngineError>
	{
		let signals_on_complete = signal_on_complete.map(|q| vec![**q.get_internal()]).unwrap_or(vec![]);
		let wait_stages = if wait_for_execute.is_empty() { vec![VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT] }
		else { wait_for_execute.into_iter().map(|&(_, s)| s).collect::<Vec<_>>() };
		let wait_semaphores = wait_for_execute.into_iter().map(|&(q, _)| **q.get_internal()).collect::<Vec<_>>();

		let subinfo = VkSubmitInfo
		{
			sType: VkStructureType::SubmitInfo, pNext: std::ptr::null(),
			commandBufferCount: commands.len() as u32, pCommandBuffers: commands.as_ptr(),
			waitSemaphoreCount: wait_semaphores.len() as u32, pWaitSemaphores: wait_semaphores.as_ptr(), pWaitDstStageMask: wait_stages.as_ptr(),
			signalSemaphoreCount: signals_on_complete.len() as u32, pSignalSemaphores: signals_on_complete.as_ptr()
		};
		self.0.get_graphics_queue().submit(&[subinfo], signal_on_complete_host.map(|x| x.get_internal())).map_err(EngineError::from)
	}
	fn submit_transfer_commands(&self, commands: &TransferCommandBuffersView, wait_for_execute: &[(&QueueFence, VkPipelineStageFlags)],
		signal_on_complete: Option<&QueueFence>, signal_on_complete_host: Option<&Fence>) -> Result<(), EngineError>
	{
		let signals_on_complete = signal_on_complete.map(|q| vec![**q.get_internal()]).unwrap_or(vec![]);
		let wait_stages = if wait_for_execute.is_empty() { vec![VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT] }
		else { wait_for_execute.into_iter().map(|&(_, s)| s).collect::<Vec<_>>() };
		let wait_semaphores = wait_for_execute.into_iter().map(|&(q, _)| **q.get_internal()).collect::<Vec<_>>();

		let subinfo = VkSubmitInfo
		{
			sType: VkStructureType::SubmitInfo, pNext: std::ptr::null(),
			commandBufferCount: commands.len() as u32, pCommandBuffers: commands.as_ptr(),
			waitSemaphoreCount: wait_semaphores.len() as u32, pWaitSemaphores: wait_semaphores.as_ptr(), pWaitDstStageMask: wait_stages.as_ptr(),
			signalSemaphoreCount: signals_on_complete.len() as u32, pSignalSemaphores: signals_on_complete.as_ptr()
		};
		self.0.get_transfer_queue().submit(&[subinfo], signal_on_complete_host.map(|x| x.get_internal())).map_err(EngineError::from)
	}
}

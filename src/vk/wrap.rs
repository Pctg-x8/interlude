// Objective Vulkan Wrapping

use super::ffi::*;
use super::traits::*;
use std;
use std::ffi::*;
use std::os::raw::*;
use libc::size_t;
use std::rc::Rc;
use xcb;

fn empty_handle<T>() -> *mut T { std::ptr::null_mut() }

impl ResultValueToObject for VkResult
{
	fn to_result(self) -> Result<(), Self> { return if self == VkResult::Success { Ok(()) } else { Err(self) } }
	fn and_then<F, T>(self, f: F) -> Result<T, Self> where F: FnOnce() -> Result<T, Self>
	{
		return if self == VkResult::Success { f() } else { Err(self) }
	}
	fn map<F, T>(self, f: F) -> Result<T, Self> where F: FnOnce() -> T
	{
		return if self == VkResult::Success { Ok(f()) } else { Err(self) }
	}
}

pub struct Instance
{
	obj: VkInstance,
	create_debug_report_callback: Option<PFN_vkCreateDebugReportCallbackEXT>,
	destroy_debug_report_callback: Option<PFN_vkDestroyDebugReportCallbackEXT>
}
impl std::ops::Drop for Instance { fn drop(&mut self) { unsafe { vkDestroyInstance(self.obj, std::ptr::null()) }; } }
impl Instance
{
	pub fn new(app_name: &str, app_version: u32, engine_name: &str, engine_version: u32, layers: &[&str], extensions: &[&str])
		-> Result<Self, VkResult>
	{
		let (app_name_c, engine_name_c, layers_c, extensions_c) = (
			CString::new(app_name).unwrap(), CString::new(engine_name).unwrap(),
			layers.into_iter().map(|&x| CString::new(x).unwrap()).collect::<Vec<_>>(),
			extensions.into_iter().map(|&x| CString::new(x).unwrap()).collect::<Vec<_>>()
		);
		let (layers_ptr_c, extensions_ptr_c) = (
			layers_c.iter().map(|x| x.as_ptr()).collect::<Vec<_>>(),
			extensions_c.iter().map(|x| x.as_ptr()).collect::<Vec<_>>()
		);
		let app = VkApplicationInfo
		{
			sType: VkStructureType::ApplicationInfo, pNext: std::ptr::null(),
			pApplicationName: app_name_c.as_ptr(), applicationVersion: app_version,
			pEngineName: engine_name_c.as_ptr(), engineVersion: engine_version,
			apiVersion: VK_API_VERSION
		};
		let info = VkInstanceCreateInfo
		{
			sType: VkStructureType::InstanceCreateInfo, pNext: std::ptr::null(), flags: 0, pApplicationInfo: &app,
			enabledLayerCount: layers_ptr_c.len() as u32, ppEnabledLayerNames: layers_ptr_c.as_ptr(),
			enabledExtensionCount: extensions_ptr_c.len() as u32, ppEnabledExtensionNames: extensions_ptr_c.as_ptr()
		};
		
		let mut instance: VkInstance = empty_handle();
		unsafe { vkCreateInstance(&info, std::ptr::null(), &mut instance) }
			.map(move || if layers.iter().find(|&&e| e == "VK_LAYER_LUNARG_standard_validation").is_some()
			{
				let create_debug_report_callback_name = CString::new("vkCreateDebugReportCallbackEXT").unwrap();
				let destroy_debug_report_callback_name = CString::new("vkDestroyDebugReportCallbackEXT").unwrap();
				let create_debug_report_callback_f: PFN_vkCreateDebugReportCallbackEXT =
					unsafe { std::mem::transmute(vkGetInstanceProcAddr(instance, create_debug_report_callback_name.as_ptr())) };
				let destory_debug_report_callback_f: PFN_vkDestroyDebugReportCallbackEXT =
					unsafe { std::mem::transmute(vkGetInstanceProcAddr(instance, destroy_debug_report_callback_name.as_ptr())) };
				
				Instance
				{
					obj: instance,
					create_debug_report_callback: Some(create_debug_report_callback_f),
					destroy_debug_report_callback: Some(destory_debug_report_callback_f)
				}
			}
			else
			{
				Instance { obj: instance, create_debug_report_callback: None, destroy_debug_report_callback: None }
			})
	}
	pub fn enumerate_adapters(&self) -> Result<Vec<VkPhysicalDevice>, VkResult>
	{
		let mut adapter_count: u32 = 0;
		unsafe { vkEnumeratePhysicalDevices(self.obj, &mut adapter_count, std::ptr::null_mut()).map(move || adapter_count) }
			.and_then(|mut adapter_count| unsafe
			{
				let mut adapters: Vec<VkPhysicalDevice> = vec![empty_handle(); adapter_count as usize];
				vkEnumeratePhysicalDevices(self.obj, &mut adapter_count, adapters.as_mut_ptr()).map(move || adapters)
			})
	}
}
pub struct DebugReportCallback { obj: VkDebugReportCallbackEXT, parent: Rc<Instance> }
impl std::ops::Drop for DebugReportCallback
{
	fn drop(&mut self)
	{
		unsafe
		{
			(self.parent.destroy_debug_report_callback.expect("Validation Layer is not presented"))(self.parent.obj, self.obj, std::ptr::null());
		}
	}
}
impl DebugReportCallback
{
	pub fn new(instance: &Rc<Instance>, f: PFN_vkDebugReportCallbackEXT) -> Result<Self, VkResult>
	{
		let info = VkDebugReportCallbackCreateInfoEXT
		{
			sType: VkStructureType::DebugReportCallbackCreateInfoEXT, pNext: std::ptr::null(),
			flags: VK_DEBUG_REPORT_ERROR_BIT_EXT | VK_DEBUG_REPORT_WARNING_BIT_EXT | VK_DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT,
			pfnCallback: f, pUserData: std::ptr::null_mut()
		};
		let mut callback: VkDebugReportCallbackEXT = empty_handle();
		unsafe { (instance.create_debug_report_callback.expect("Validation Layer is not presented"))(instance.obj, &info, std::ptr::null(), &mut callback) }
			.map(move || DebugReportCallback { obj: callback, parent: instance.clone() })
	}
}
pub struct PhysicalDevice { obj: VkPhysicalDevice, #[allow(dead_code)] parent: Rc<Instance> }
impl PhysicalDevice
{
	pub fn from(pd: VkPhysicalDevice, parent: &Rc<Instance>) -> PhysicalDevice { PhysicalDevice { obj: pd, parent: parent.clone() } }
	pub fn get_properties(&self) -> VkPhysicalDeviceProperties
	{
		let mut props: VkPhysicalDeviceProperties = unsafe { std::mem::uninitialized() };
		unsafe { vkGetPhysicalDeviceProperties(self.obj, &mut props) };
		props
	}
	pub fn get_features(&self) -> VkPhysicalDeviceFeatures
	{
		let mut features: VkPhysicalDeviceFeatures = unsafe { std::mem::uninitialized() };
		unsafe { vkGetPhysicalDeviceFeatures(self.obj, &mut features) };
		features
	}
	pub fn get_memory_properties(&self) -> VkPhysicalDeviceMemoryProperties
	{
		let mut props: VkPhysicalDeviceMemoryProperties = unsafe { std::mem::uninitialized() };
		unsafe { vkGetPhysicalDeviceMemoryProperties(self.obj, &mut props) };
		props
	}
	pub fn enumerate_queue_family_properties(&self) -> Vec<VkQueueFamilyProperties>
	{
		let mut property_count: u32 = 0;
		unsafe { vkGetPhysicalDeviceQueueFamilyProperties(self.obj, &mut property_count, std::ptr::null_mut()) };
		let mut properties: Vec<VkQueueFamilyProperties> = unsafe { vec![std::mem::uninitialized(); property_count as usize] };
		unsafe { vkGetPhysicalDeviceQueueFamilyProperties(self.obj, &mut property_count, properties.as_mut_ptr()) };
		properties
	}

	pub fn is_xcb_presentation_support(&self, queue_family_index: u32, con: *mut xcb::ffi::xcb_connection_t, vis: xcb::ffi::xcb_visualid_t) -> bool
	{
		unsafe { vkGetPhysicalDeviceXcbPresentationSupportKHR(self.obj, queue_family_index, con, vis) == 1 }
	}
	pub fn is_surface_support(&self, queue_family_index: u32, surface: &Surface) -> bool
	{
		let mut supported = false as VkBool32;
		unsafe { vkGetPhysicalDeviceSurfaceSupportKHR(self.obj, queue_family_index, surface.obj, &mut supported) };
		supported == true as VkBool32
	}
	pub fn get_surface_caps(&self, surface: &Surface) -> VkSurfaceCapabilitiesKHR
	{
		let mut caps: VkSurfaceCapabilitiesKHR = unsafe { std::mem::uninitialized() };
		unsafe { vkGetPhysicalDeviceSurfaceCapabilitiesKHR(self.obj, surface.obj, &mut caps) };
		caps
	}
	pub fn enumerate_surface_formats(&self, surface: &Surface) -> Vec<VkSurfaceFormatKHR>
	{
		let mut format_count = 0u32;
		unsafe { vkGetPhysicalDeviceSurfaceFormatsKHR(self.obj, surface.obj, &mut format_count, std::ptr::null_mut()) };
		let mut formats: Vec<VkSurfaceFormatKHR> = unsafe { vec![std::mem::uninitialized(); format_count as usize] };
		unsafe { vkGetPhysicalDeviceSurfaceFormatsKHR(self.obj, surface.obj, &mut format_count, formats.as_mut_ptr()) };
		formats
	}
	pub fn enumerate_present_modes(&self, surface: &Surface) -> Vec<VkPresentModeKHR>
	{
		let mut mode_count = 0u32;
		unsafe { vkGetPhysicalDeviceSurfacePresentModesKHR(self.obj, surface.obj, &mut mode_count, std::ptr::null_mut()) };
		warn!(target: "vk::Wrap[NV/rustc1.12.0]", "あとで外す at interlude/src/vk/wrap.rs:181");
		let mut modes: Vec<VkPresentModeKHR> = unsafe { vec![std::mem::uninitialized(); mode_count as usize] };
		unsafe { vkGetPhysicalDeviceSurfacePresentModesKHR(self.obj, surface.obj, &mut mode_count, modes.as_mut_ptr()) };
		modes
	}
}
pub struct Device { obj: VkDevice, #[allow(dead_code)] parent: Rc<PhysicalDevice> }
impl std::ops::Drop for Device { fn drop(&mut self) { self.wait_for_idle().unwrap(); unsafe { vkDestroyDevice(self.obj, std::ptr::null()) }; } }
impl NativeOwner<VkDevice> for Device { fn get(&self) -> VkDevice { self.obj } }
impl Device
{
	pub fn new(adapter: &Rc<PhysicalDevice>, queue: &[VkDeviceQueueCreateInfo],
		layers: &[&str], extensions: &[&str], enabled_features: &VkPhysicalDeviceFeatures) -> Result<Self, VkResult>
	{
		let (layers_c, extensions_c): (Vec<CString>, Vec<CString>) = (
			layers.into_iter().map(|&x| CString::new(x).unwrap()).collect(),
			extensions.into_iter().map(|&x| CString::new(x).unwrap()).collect()
		);
		let (layers_ptr_c, extensions_ptr_c) = (
			layers_c.iter().map(|x| x.as_ptr()).collect::<Vec<_>>(),
			extensions_c.iter().map(|x| x.as_ptr()).collect::<Vec<_>>()
		);
		let info = VkDeviceCreateInfo
		{
			sType: VkStructureType::DeviceCreateInfo, pNext: std::ptr::null(), flags: 0,
			queueCreateInfoCount: queue.len() as u32, pQueueCreateInfos: queue.as_ptr(),
			enabledLayerCount: layers_ptr_c.len() as u32, ppEnabledLayerNames: layers_ptr_c.as_ptr(),
			enabledExtensionCount: extensions_ptr_c.len() as u32, ppEnabledExtensionNames: extensions_ptr_c.as_ptr(),
			pEnabledFeatures: enabled_features
		};
		let mut dev: VkDevice = empty_handle();
		unsafe { vkCreateDevice(adapter.obj, &info, std::ptr::null(), &mut dev) }.map(move || Device { obj: dev, parent: adapter.clone() })
	}
	pub fn get_queue(&self, family_index: u32, index: u32) -> Queue
	{
		let mut q: VkQueue = empty_handle();
		unsafe { vkGetDeviceQueue(self.obj, family_index, index, &mut q) };
		Queue { obj: q, family_index: family_index }
	}

	pub fn update_descriptor_sets(&self, write_infos: &[VkWriteDescriptorSet], copy_infos: &[VkCopyDescriptorSet])
	{
		unsafe { vkUpdateDescriptorSets(self.obj, write_infos.len() as u32, write_infos.as_ptr(), copy_infos.len() as u32, copy_infos.as_ptr()) };
	}
	pub fn wait_for_idle(&self) -> Result<(), VkResult> { unsafe { vkDeviceWaitIdle(self.obj) }.to_result() }
}
pub struct Queue { obj: VkQueue, pub family_index: u32 }
impl Queue
{
	pub fn batched_submission(&self, infos: &[VkSubmitInfo], event_receiver: Option<&Fence>) -> Result<(), VkResult>
	{
		unsafe { vkQueueSubmit(self.obj, infos.len() as u32, infos.as_ptr(), event_receiver.map(|x| x.obj).unwrap_or(std::ptr::null_mut())) }.to_result()
	}
	pub fn submit_commands(&self, buffers: &[VkCommandBuffer],
		device_synchronizer: &[VkSemaphore], synchronizer_stages: &[VkPipelineStageFlags], device_signalizer: &[VkSemaphore],
		event_receiver: Option<&Fence>) -> Result<(), VkResult>
	{
		let submit_info = VkSubmitInfo
		{
			sType: VkStructureType::SubmitInfo, pNext: std::ptr::null(),
			waitSemaphoreCount: device_synchronizer.len() as u32, pWaitSemaphores: device_synchronizer.as_ptr(), pWaitDstStageMask: synchronizer_stages.as_ptr(),
			commandBufferCount: buffers.len() as u32, pCommandBuffers: buffers.as_ptr(),
			signalSemaphoreCount: device_signalizer.len() as u32, pSignalSemaphores: device_signalizer.as_ptr()
		};
		self.batched_submission(&[submit_info], event_receiver)
	}
	pub fn wait_for_idle(&self) -> Result<(), VkResult>
	{
		unsafe { vkQueueWaitIdle(self.obj) }.to_result()
	}
}
unsafe impl Sync for Queue {}
pub struct DeviceMemory { parent: Rc<Device>, obj: VkDeviceMemory }
impl std::ops::Drop for DeviceMemory { fn drop(&mut self) { unsafe { vkFreeMemory(self.parent.obj, self.obj, std::ptr::null()) }; } }
impl DeviceMemory
{
	pub fn alloc(device: &Rc<Device>, info: &VkMemoryAllocateInfo) -> Result<Self, VkResult>
	{
		let mut mem: VkDeviceMemory = empty_handle();
		unsafe { vkAllocateMemory(device.obj, info, std::ptr::null(), &mut mem) }.map(move || DeviceMemory { obj: mem, parent: device.clone() })
	}
	pub fn map(&self, range: std::ops::Range<VkDeviceSize>) -> Result<*mut c_void, VkResult>
	{
		let mut data_ptr: *mut c_void = std::ptr::null_mut();
		unsafe { vkMapMemory(self.parent.obj, self.obj, range.start, range.end - range.start, 0, std::mem::transmute(&mut data_ptr)) }
			.map(move || data_ptr)
	}
	pub fn unmap(&self)
	{
		unsafe { vkUnmapMemory(self.parent.obj, self.obj) };
	}
	pub fn bind_buffer(&self, buffer: &Buffer, offset: VkDeviceSize) -> Result<(), VkResult>
	{
		unsafe { vkBindBufferMemory(self.parent.obj, buffer.obj, self.obj, offset) }.to_result()
	}
	pub fn bind_image(&self, image: &Image, offset: VkDeviceSize) -> Result<(), VkResult>
	{
		unsafe { vkBindImageMemory(self.parent.obj, image.obj, self.obj, offset) }.to_result()
	}
}
pub struct Buffer { #[allow(dead_code)] parent: Rc<Device>, obj: VkBuffer }
impl std::ops::Drop for Buffer { fn drop(&mut self) { unsafe { vkDestroyBuffer(self.parent.obj, self.obj, std::ptr::null()) }; } }
impl NativeOwner<VkBuffer> for Buffer { fn get(&self) -> VkBuffer { self.obj } }
impl Buffer
{
	pub fn new(device: &Rc<Device>, info: &VkBufferCreateInfo) -> Result<Self, VkResult>
	{
		let mut buffer: VkBuffer = empty_handle();
		unsafe { vkCreateBuffer(device.obj, info, std::ptr::null(), &mut buffer) }.map(move || Buffer { obj: buffer, parent: device.clone() })
	}
}
pub struct Image { #[allow(dead_code)] parent: Rc<Device>, obj: VkImage }
impl std::ops::Drop for Image { fn drop(&mut self) { unsafe { vkDestroyImage(self.parent.obj, self.obj, std::ptr::null()) }; } }
impl NativeOwner<VkImage> for Image { fn get(&self) -> VkImage { self.obj } }
impl Image
{
	pub fn new(device: &Rc<Device>, info: &VkImageCreateInfo) -> Result<Self, VkResult>
	{
		let mut image: VkImage = empty_handle();
		unsafe { vkCreateImage(device.obj, info, std::ptr::null(), &mut image) }.map(move || Image { obj: image, parent: device.clone() })
	}
}
pub struct ImageView { #[allow(dead_code)] parent: Rc<Device>, obj: VkImageView }
impl std::ops::Drop for ImageView { fn drop(&mut self) { unsafe { vkDestroyImageView(self.parent.obj, self.obj, std::ptr::null()) }; } }
impl NativeOwner<VkImageView> for ImageView { fn get(&self) -> VkImageView { self.obj } }
impl ImageView
{
	pub fn new(device: &Rc<Device>, info: &VkImageViewCreateInfo) -> Result<Self, VkResult>
	{
		let mut view: VkImageView = empty_handle();
		unsafe { vkCreateImageView(device.obj, info, std::ptr::null(), &mut view) }.map(move || ImageView { obj: view, parent: device.clone() })
	}
}
pub struct RenderPass { #[allow(dead_code)] parent: Rc<Device>, obj: VkRenderPass }
impl std::ops::Drop for RenderPass { fn drop(&mut self) { unsafe { vkDestroyRenderPass(self.parent.obj, self.obj, std::ptr::null()) }; } }
impl NativeOwner<VkRenderPass> for RenderPass { fn get(&self) -> VkRenderPass { self.obj } }
impl RenderPass
{
	pub fn new(device: &Rc<Device>, info: &VkRenderPassCreateInfo) -> Result<Self, VkResult>
	{
		let mut pass: VkRenderPass = empty_handle();
		unsafe { vkCreateRenderPass(device.obj, info, std::ptr::null(), &mut pass) }.map(move || RenderPass { obj: pass, parent: device.clone() })
	}
}
pub struct Framebuffer { #[allow(dead_code)] parent: Rc<Device>, obj: VkFramebuffer }
impl std::ops::Drop for Framebuffer { fn drop(&mut self) { unsafe { vkDestroyFramebuffer(self.parent.obj, self.obj, std::ptr::null()) }; } }
impl NativeOwner<VkFramebuffer> for Framebuffer { fn get(&self) -> VkFramebuffer { self.obj } }
impl Framebuffer
{
	pub fn new(device: &Rc<Device>, info: &VkFramebufferCreateInfo) -> Result<Self, VkResult>
	{
		let mut fb: VkFramebuffer = empty_handle();
		unsafe { vkCreateFramebuffer(device.obj, info, std::ptr::null(), &mut fb) }.map(move || Framebuffer { obj: fb, parent: device.clone() })
	}
}

pub struct CommandPool { #[allow(dead_code)] parent: Rc<Device>, obj: VkCommandPool }
impl std::ops::Drop for CommandPool { fn drop(&mut self) { unsafe { vkDestroyCommandPool(self.parent.obj, self.obj, std::ptr::null()) }; } }
impl NativeOwner<VkCommandPool> for CommandPool { fn get(&self) -> VkCommandPool { self.obj } }
impl HasParent for CommandPool { type ParentRefType = Rc<Device>; fn parent(&self) -> &Rc<Device> { &self.parent } }
impl CommandPool
{
	pub fn new(device: &Rc<Device>, queue: &Queue, transient: bool) -> Result<Self, VkResult>
	{
		let flags = if transient { VK_COMMAND_POOL_CREATE_TRANSIENT_BIT } else { 0 };
		let info = VkCommandPoolCreateInfo
		{
			sType: VkStructureType::CommandPoolCreateInfo, pNext: std::ptr::null(),
			flags: flags, queueFamilyIndex: queue.family_index
		};
		let mut pool = empty_handle();
		unsafe { vkCreateCommandPool(device.obj, &info, std::ptr::null(), &mut pool) }.map(move || CommandPool { obj: pool, parent: device.clone() })
	}
	pub fn allocate_buffers(&self, device: &Device, buffer_level: VkCommandBufferLevel, count: u32) -> Result<Vec<VkCommandBuffer>, VkResult>
	{
		let info = VkCommandBufferAllocateInfo
		{
			sType: VkStructureType::CommandBufferAllocateInfo, pNext: std::ptr::null(),
			commandPool: self.obj, level: buffer_level, commandBufferCount: count
		};
		let mut buffers: Vec<VkCommandBuffer> = vec![std::ptr::null_mut(); count as usize];
		unsafe { vkAllocateCommandBuffers(device.obj, &info, buffers.as_mut_ptr()) }.map(move || buffers)
	}
}

pub struct ShaderModule { #[allow(dead_code)] parent: Rc<Device>, obj: VkShaderModule }
impl std::ops::Drop for ShaderModule { fn drop(&mut self) { unsafe { vkDestroyShaderModule(self.parent.obj, self.obj, std::ptr::null()) }; } }
impl NativeOwner<VkShaderModule> for ShaderModule { fn get(&self) -> VkShaderModule { self.obj } }
impl ShaderModule
{
	pub fn new(device: &Rc<Device>, binary: &[u8]) -> Result<Self, VkResult>
	{
		let info = VkShaderModuleCreateInfo
		{
			sType: VkStructureType::ShaderModuleCreateInfo, pNext: std::ptr::null(),
			flags: 0, codeSize: binary.len() as size_t, pCode: unsafe { std::mem::transmute(binary.as_ptr()) }
		};
		let mut m: VkShaderModule = empty_handle();
		unsafe { vkCreateShaderModule(device.obj, &info, std::ptr::null(), &mut m) }.map(move || ShaderModule { obj: m, parent: device.clone() })
	}
}
pub struct PipelineLayout { #[allow(dead_code)] parent: Rc<Device>, obj: VkPipelineLayout }
impl std::ops::Drop for PipelineLayout { fn drop(&mut self) { unsafe { vkDestroyPipelineLayout(self.parent.obj, self.obj, std::ptr::null()) }; } }
impl NativeOwner<VkPipelineLayout> for PipelineLayout { fn get(&self) -> VkPipelineLayout { self.obj } }
impl PipelineLayout
{
	pub fn new(device: &Rc<Device>, descriptor_set_layouts: &[VkDescriptorSetLayout], push_constants: &[VkPushConstantRange]) -> Result<Self, VkResult>
	{
		let info = VkPipelineLayoutCreateInfo
		{
			sType: VkStructureType::PipelineLayoutCreateInfo, pNext: std::ptr::null(), flags: 0,
			setLayoutCount: descriptor_set_layouts.len() as u32, pSetLayouts: descriptor_set_layouts.as_ptr(),
			pushConstantRangeCount: push_constants.len() as u32, pPushConstantRanges: push_constants.as_ptr()
		};
		let mut layout: VkPipelineLayout = empty_handle();
		unsafe { vkCreatePipelineLayout(device.obj, &info, std::ptr::null(), &mut layout) }
			.map(move || PipelineLayout { obj: layout, parent: device.clone() })
	}
}
pub struct PipelineCache { #[allow(dead_code)] parent: Rc<Device>, obj: VkPipelineCache }
impl std::ops::Drop for PipelineCache { fn drop(&mut self) { unsafe { vkDestroyPipelineCache(self.parent.obj, self.obj, std::ptr::null()) }; } }
impl NativeOwner<VkPipelineCache> for PipelineCache { fn get(&self) -> VkPipelineCache { self.obj } }
impl PipelineCache
{
	pub fn new_empty(device: &Rc<Device>) -> Result<Self, VkResult>
	{
		let info = VkPipelineCacheCreateInfo
		{
			sType: VkStructureType::PipelineCacheCreateInfo, pNext: std::ptr::null(), flags: 0,
			pInitialData: std::ptr::null(), initialDataSize: 0
		};
		let mut c: VkPipelineCache = empty_handle();
		unsafe { vkCreatePipelineCache(device.obj, &info, std::ptr::null(), &mut c) }.map(move || PipelineCache { obj: c, parent: device.clone() })
	}
}
pub struct Pipeline { parent: Rc<Device>, obj: VkPipeline }
impl std::ops::Drop for Pipeline { fn drop(&mut self) { unsafe { vkDestroyPipeline(self.parent.obj, self.obj, std::ptr::null()) }; } }
impl NativeOwner<VkPipeline> for Pipeline { fn get(&self) -> VkPipeline { self.obj } }
impl Pipeline
{
	pub fn new(device: &Rc<Device>, cache: &PipelineCache, infos: &[VkGraphicsPipelineCreateInfo]) -> Result<Vec<Self>, VkResult>
	{
		let mut objs: Vec<VkPipeline> = vec![empty_handle(); infos.len()];
		unsafe { vkCreateGraphicsPipelines(device.obj, cache.obj, infos.len() as u32, infos.as_ptr(), std::ptr::null(), objs.as_mut_ptr()) }
			.map(move || objs.into_iter().map(|p| Pipeline { obj: p, parent: device.clone() }).collect())
	}
}

pub struct Fence { parent: Rc<Device>, obj: VkFence }
impl std::ops::Drop for Fence { fn drop(&mut self) { unsafe { vkDestroyFence(self.parent.obj, self.obj, std::ptr::null()) }; } }
impl Fence
{
	pub fn new(device: &Rc<Device>) -> Result<Self, VkResult>
	{
		let info = VkFenceCreateInfo { sType: VkStructureType::FenceCreateInfo, pNext: std::ptr::null(), flags: 0 };
		let mut fence: VkFence = empty_handle();
		unsafe { vkCreateFence(device.obj, &info, std::ptr::null(), &mut fence) }.map(move || Fence { obj: fence, parent: device.clone() })
	}
	pub fn wait(&self) -> Result<(), VkResult>
	{
		unsafe { vkWaitForFences(self.parent.obj, 1, &self.obj, true as VkBool32, std::u64::MAX) }.to_result()
	}
	pub fn reset(&self) -> Result<(), VkResult>
	{
		unsafe { vkResetFences(self.parent.obj, 1, &self.obj) }.to_result()
	}
	pub fn get_status(&self) -> Result<(), VkResult>
	{
		unsafe { vkGetFenceStatus(self.parent.obj, self.obj) }.to_result()
	}
}
pub struct Semaphore { parent: Rc<Device>, obj: VkSemaphore }
impl std::ops::Drop for Semaphore { fn drop(&mut self) { unsafe { vkDestroySemaphore(self.parent.obj, self.obj, std::ptr::null()) }; } }
impl NativeOwner<VkSemaphore> for Semaphore { fn get(&self) -> VkSemaphore { self.obj } }
impl Semaphore
{
	pub fn new(device: &Rc<Device>) -> Result<Self, VkResult>
	{
		let info = VkSemaphoreCreateInfo { sType: VkStructureType::SemaphoreCreateInfo, pNext: std::ptr::null(), flags: 0 };
		let mut sem: VkSemaphore = empty_handle();
		unsafe { vkCreateSemaphore(device.obj, &info, std::ptr::null(), &mut sem) }.map(move || Semaphore { obj: sem, parent: device.clone() })
	}
}

pub struct DescriptorSetLayout { parent: Rc<Device>, obj: VkDescriptorSetLayout }
impl std::ops::Drop for DescriptorSetLayout { fn drop(&mut self) { unsafe { vkDestroyDescriptorSetLayout(self.parent.obj, self.obj, std::ptr::null()) }; } }
impl NativeOwner<VkDescriptorSetLayout> for DescriptorSetLayout { fn get(&self) -> VkDescriptorSetLayout { self.obj } }
impl DescriptorSetLayout
{
	pub fn new(device: &Rc<Device>, bindings: &[VkDescriptorSetLayoutBinding]) -> Result<Self, VkResult>
	{
		let info = VkDescriptorSetLayoutCreateInfo
		{
			sType: VkStructureType::DescriptorSetLayoutCreateInfo, pNext: std::ptr::null(), flags: 0,
			bindingCount: bindings.len() as u32, pBindings: bindings.as_ptr()
		};
		let mut dsl: VkDescriptorSetLayout = empty_handle();
		unsafe { vkCreateDescriptorSetLayout(device.obj, &info, std::ptr::null(), &mut dsl) }
			.map(move || DescriptorSetLayout { obj: dsl, parent: device.clone() })
	}
}
pub struct DescriptorPool { parent: Rc<Device>, obj: VkDescriptorPool }
impl std::ops::Drop for DescriptorPool { fn drop(&mut self) { unsafe { vkDestroyDescriptorPool(self.parent.obj, self.obj, std::ptr::null()) }; } }
impl DescriptorPool
{
	pub fn new(device: &Rc<Device>, max_sets: u32, pool_sizes: &[VkDescriptorPoolSize]) -> Result<Self, VkResult>
	{
		let info = VkDescriptorPoolCreateInfo
		{
			sType: VkStructureType::DescriptorPoolCreateInfo, pNext: std::ptr::null(), flags: 0,
			maxSets: max_sets, poolSizeCount: pool_sizes.len() as u32, pPoolSizes: pool_sizes.as_ptr()
		};
		let mut pool: VkDescriptorPool = empty_handle();
		unsafe { vkCreateDescriptorPool(device.obj, &info, std::ptr::null(), &mut pool) }.map(move || DescriptorPool { obj: pool, parent: device.clone() })
	}
	pub fn allocate_sets(&self, device: &Device, layouts: &[VkDescriptorSetLayout]) -> Result<Vec<VkDescriptorSet>, VkResult>
	{
		let info = VkDescriptorSetAllocateInfo
		{
			sType: VkStructureType::DescriptorSetAllocateInfo, pNext: std::ptr::null(),
			descriptorPool: self.obj, descriptorSetCount: layouts.len() as u32, pSetLayouts: layouts.as_ptr()
		};
		let mut objs: Vec<VkDescriptorSet> = vec![unsafe { std::mem::uninitialized() }; layouts.len()];
		unsafe { vkAllocateDescriptorSets(device.obj, &info, objs.as_mut_ptr()) }.map(move || objs)
	}
}

pub struct Sampler { parent: Rc<Device>, obj: VkSampler }
impl std::ops::Drop for Sampler { fn drop(&mut self) { unsafe { vkDestroySampler(self.parent.obj, self.obj, std::ptr::null()) }; } }
impl NativeOwner<VkSampler> for Sampler { fn get(&self) -> VkSampler { self.obj } }
impl Sampler
{
	pub fn new(device: &Rc<Device>, info: &VkSamplerCreateInfo) -> Result<Self, VkResult>
	{
		let mut sampler: VkSampler = empty_handle();
		unsafe { vkCreateSampler(device.obj, info, std::ptr::null(), &mut sampler) }.map(move || Sampler { obj: sampler, parent: device.clone() })
	}
}

pub struct Surface { parent: Rc<Instance>, obj: VkSurfaceKHR }
impl std::ops::Drop for Surface { fn drop(&mut self) { unsafe { vkDestroySurfaceKHR(self.parent.obj, self.obj, std::ptr::null()) }; } }
impl NativeOwner<VkSurfaceKHR> for Surface { fn get(&self) -> VkSurfaceKHR { self.obj } }
impl Surface
{
	pub fn new_xcb(instance: &Rc<Instance>, info: &VkXcbSurfaceCreateInfoKHR) -> Result<Surface, VkResult>
	{
		let mut surf: VkSurfaceKHR = empty_handle();
		unsafe { vkCreateXcbSurfaceKHR(instance.obj, info, std::ptr::null(), &mut surf) }.map(move || Surface { obj: surf, parent: instance.clone() })
	}
}
pub struct Swapchain { parent: Rc<Device>, #[allow(dead_code)] base: Rc<Surface>, obj: VkSwapchainKHR }
impl std::ops::Drop for Swapchain { fn drop(&mut self) { unsafe { vkDestroySwapchainKHR(self.parent.obj, self.obj, std::ptr::null()) }; } }
impl HasParent for Swapchain { type ParentRefType = Rc<Device>; fn parent(&self) -> &Rc<Device> { &self.parent } }
impl Swapchain
{
	pub fn new(device: &Rc<Device>, surface: &Rc<Surface>, info: &VkSwapchainCreateInfoKHR) -> Result<Self, VkResult>
	{
		let mut sc: VkSwapchainKHR = empty_handle();
		unsafe { vkCreateSwapchainKHR(device.obj, info, std::ptr::null(), &mut sc) }
			.map(move || Swapchain { obj: sc, base: surface.clone(), parent: device.clone() })
	}
	pub fn get_images(&self) -> Result<Vec<VkImage>, VkResult>
	{
		let mut image_count = 0u32;
		unsafe { vkGetSwapchainImagesKHR(self.parent.obj, self.obj, &mut image_count, std::ptr::null_mut()) }.and_then(move ||
		{
			let mut images: Vec<VkImage> = vec![empty_handle(); image_count as usize];
			unsafe { vkGetSwapchainImagesKHR(self.parent.obj, self.obj, &mut image_count, images.as_mut_ptr()) }.map(move || images)
		})
	}
	pub fn acquire_next_image(&self, device_synchronizer: &Semaphore) -> Result<u32, VkResult>
	{
		let mut index: u32 = 0;
		unsafe { vkAcquireNextImageKHR(self.parent.obj, self.obj, std::u64::MAX, device_synchronizer.obj, std::ptr::null_mut(), &mut index) }
			.map(move || index)
	}
	pub fn present(&self, queue: &Queue, index: u32, device_synchronizer: &[VkSemaphore]) -> Result<(), VkResult>
	{
		let info = VkPresentInfoKHR
		{
			sType: VkStructureType::PresentInfoKHR, pNext: std::ptr::null(),
			swapchainCount: 1, pSwapchains: &self.obj, pImageIndices: &index,
			waitSemaphoreCount: device_synchronizer.len() as u32, pWaitSemaphores: device_synchronizer.as_ptr(), pResults: std::ptr::null_mut()
		};
		unsafe { vkQueuePresentKHR(queue.obj, &info) }.to_result()
	}
}
impl MemoryAllocationRequired for Buffer
{
	fn get_memory_requirements(&self) -> VkMemoryRequirements
	{
		let mut memreq: VkMemoryRequirements = unsafe { std::mem::uninitialized() };
		unsafe { vkGetBufferMemoryRequirements(self.parent.obj, self.obj, &mut memreq) };
		memreq
	}
}
impl MemoryAllocationRequired for Image
{
	fn get_memory_requirements(&self) -> VkMemoryRequirements
	{
		let mut memreq: VkMemoryRequirements = unsafe { std::mem::uninitialized() };
		unsafe { vkGetImageMemoryRequirements(self.parent.obj, self.obj, &mut memreq) };
		memreq
	}
}

pub enum ImageSubresourceRange {}
impl ImageSubresourceRange
{
	pub fn default_color() -> VkImageSubresourceRange
	{
		VkImageSubresourceRange
		{
			aspectMask: VK_IMAGE_ASPECT_COLOR_BIT, baseMipLevel: 0, baseArrayLayer: 0,
			levelCount: 1, layerCount: 1
		}
	}
}

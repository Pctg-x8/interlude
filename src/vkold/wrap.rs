// Objective Vulkan Wrapping

use super::*;
use super::traits::*;
use std;
use std::ffi::*;
use std::ops::Deref;
use std::rc::Rc;
use libc::size_t;
use std::borrow::Cow;

pub fn empty_handle<T>() -> *mut T { std::ptr::null_mut() }
type VkWrapResult<T> = Result<T, VkResult>;

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

macro_rules! DeviceDataGetter
{
	($($o: expr),+ => $f: path) => 
	{{
		let mut o = unsafe { std::mem::uninitialized() };
		unsafe { $f($($o,)* &mut o) };
		o
	}}
}
macro_rules! DeviceDataEnumerator
{
	($($o: expr),+ => $f: path) =>
	{{
		let mut n = 0;
		unsafe { $f($($o,)+ &mut n, std::ptr::null_mut()) }.and_then(||
		{
			let mut o = vec![unsafe { std::mem::uninitialized() }; n as usize];
			unsafe { $f($($o,)+ &mut n, o.as_mut_ptr()) }.map(|| o)
		})
	}};
	([noexcept] $($o: expr),+ => $f: path) =>
	{{
		let mut n = 0;
		unsafe { $f($($o,)+ &mut n, std::ptr::null_mut()) };
		let mut o = vec![unsafe { std::mem::uninitialized() }; n as usize];
		unsafe { $f($($o,)+ &mut n, o.as_mut_ptr()) };
		o
	}}
}

fn c_str(source: Cow<'static, str>) -> CString
{
	match source
	{
		Cow::Borrowed(s) => CString::new(s).unwrap(),
		Cow::Owned(s) => CString::new(s).unwrap()
	}
}

pub struct Instance
{
	obj: VkInstance,
	create_debug_report_callback: Option<PFN_vkCreateDebugReportCallbackEXT>,
	destroy_debug_report_callback: Option<PFN_vkDestroyDebugReportCallbackEXT>
}
impl Drop for Instance { fn drop(&mut self) { unsafe { vkDestroyInstance(self.obj, std::ptr::null()) }; } }
impl Deref for Instance { type Target = VkInstance; fn deref(&self) -> &VkInstance { &self.obj } }
impl Instance
{
	#[allow(unused_variables)] /* layers_c, ownership holder of layer_ptr_c */
	pub fn new(app_name: Cow<'static, str>, app_version: u32, engine_name: Cow<'static, str>, engine_version: u32,
		layers: &[&str], extensions: &[&str]) -> VkWrapResult<Self>
	{
		let (app_name_c, engine_name_c) = (c_str(app_name), c_str(engine_name));
		let layers_c = layers.into_iter().map(|&s| CString::new(s).unwrap()).collect::<Vec<_>>();
		let extensions_c = extensions.into_iter().map(|&s| CString::new(s).unwrap()).collect::<Vec<_>>();
		let layers_ptr_c = layers_c.iter().map(|x| x.as_ptr()).collect::<Vec<_>>();
		let extensions_ptr_c = extensions_c.iter().map(|x| x.as_ptr()).collect::<Vec<_>>();
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
		
		let mut instance = empty_handle();
		unsafe { vkCreateInstance(&info, std::ptr::null(), &mut instance) }
			.map(|| if layers.into_iter().find(|&&e| e == "VK_LAYER_LUNARG_standard_validation").is_some()
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
	pub fn adapters(&self) -> VkWrapResult<Vec<VkPhysicalDevice>> { DeviceDataEnumerator!(**self => vkEnumeratePhysicalDevices) }
}
pub struct DebugReportCallback(VkDebugReportCallbackEXT, Rc<Instance>);
impl Deref for DebugReportCallback { type Target = VkDebugReportCallbackEXT; fn deref(&self) -> &VkDebugReportCallbackEXT { &self.0 } }
impl HasParent for DebugReportCallback { type Parent = Instance; fn parent(&self) -> &Instance { &self.1 } }
impl Drop for DebugReportCallback
{
	fn drop(&mut self)
	{
		unsafe
		{
			(self.parent().destroy_debug_report_callback.expect("Validation Layer is not presented"))(**self.parent(), **self, std::ptr::null());
		}
	}
}
impl DebugReportCallback
{
	pub fn new(instance: &Rc<Instance>, f: PFN_vkDebugReportCallbackEXT) -> VkWrapResult<Self>
	{
		let info = VkDebugReportCallbackCreateInfoEXT
		{
			sType: VkStructureType::DebugReportCallbackCreateInfoEXT, pNext: std::ptr::null(),
			flags: VK_DEBUG_REPORT_ERROR_BIT_EXT | VK_DEBUG_REPORT_WARNING_BIT_EXT | VK_DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT,
			pfnCallback: f, pUserData: std::ptr::null_mut()
		};
		let mut callback = empty_handle();
		unsafe { (instance.create_debug_report_callback.expect("Validation Layer is not presented"))(instance.obj, &info, std::ptr::null(), &mut callback) }
			.map(move || DebugReportCallback(callback, instance.clone()))
	}
}
pub struct PhysicalDevice(VkPhysicalDevice, Rc<Instance>);
impl Deref for PhysicalDevice { type Target = VkPhysicalDevice; fn deref(&self) -> &VkPhysicalDevice { &self.0 } }
impl HasParent for PhysicalDevice { type Parent = Instance; fn parent(&self) -> &Instance { &self.1 } }
impl PhysicalDevice
{
	pub fn from(pd: VkPhysicalDevice, parent: &Rc<Instance>) -> Self { PhysicalDevice(pd, parent.clone()) }

	// Getter Methods //
	pub fn properties(&self) -> VkPhysicalDeviceProperties { DeviceDataGetter!(self.0 => vkGetPhysicalDeviceProperties) }
	pub fn features(&self) -> VkPhysicalDeviceFeatures { DeviceDataGetter!(self.0 => vkGetPhysicalDeviceFeatures) }
	pub fn memory_properties(&self) -> VkPhysicalDeviceMemoryProperties { DeviceDataGetter!(self.0 => vkGetPhysicalDeviceMemoryProperties) }
	pub fn queue_family_properties(&self) -> Vec<VkQueueFamilyProperties>
	{
		DeviceDataEnumerator!([noexcept] self.0 => vkGetPhysicalDeviceQueueFamilyProperties)
	}

	// Surface //
	pub fn is_surface_support(&self, qf_index: u32, surface: &VkSurfaceKHR) -> bool
	{
		let mut supported = false as VkBool32;
		unsafe { vkGetPhysicalDeviceSurfaceSupportKHR(self.0, qf_index, *surface, &mut supported) };
		supported == true as VkBool32
	}
	pub fn surface_caps(&self, surface: &VkSurfaceKHR) -> VkSurfaceCapabilitiesKHR
	{
		let mut caps = unsafe { std::mem::uninitialized() };
		unsafe { vkGetPhysicalDeviceSurfaceCapabilitiesKHR(self.0, *surface, &mut caps) };
		caps
	}
	pub fn surface_formats(&self, surface: &VkSurfaceKHR) -> VkWrapResult<Vec<VkSurfaceFormatKHR>>
	{
		DeviceDataEnumerator!(self.0, *surface => vkGetPhysicalDeviceSurfaceFormatsKHR)
	}
	pub fn present_modes(&self, surface: &VkSurfaceKHR) -> VkWrapResult<Vec<VkPresentModeKHR>>
	{
		DeviceDataEnumerator!(self.0, *surface => vkGetPhysicalDeviceSurfacePresentModesKHR)
	}
}

// Device and Queue //
impl std::fmt::Debug for Device { fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result { write!(fmt, "<Device...>") } }
pub struct Device(VkDevice, Rc<PhysicalDevice>);
impl Drop for Device { fn drop(&mut self) { self.wait_for_idle().unwrap(); unsafe { vkDestroyDevice(self.0, std::ptr::null()) }; } }
impl Deref for Device { type Target = VkDevice; fn deref(&self) -> &VkDevice { &self.0 } }
impl HasParent for Device { type Parent = PhysicalDevice; fn parent(&self) -> &PhysicalDevice { &self.1 } }
impl Device
{
	pub fn new<NameT: AsRef<str>>(adapter: &Rc<PhysicalDevice>, queue: &[VkDeviceQueueCreateInfo], layers: &[NameT], extensions: &[NameT],
		enabled_features: &VkPhysicalDeviceFeatures) -> Result<Self, VkResult>
	{
		let (layers_c, extensions_c) = (
			layers.into_iter().map(|x| CString::new(x.as_ref()).unwrap()).collect::<Vec<_>>(),
			extensions.into_iter().map(|x| CString::new(x.as_ref()).unwrap()).collect::<Vec<_>>()
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
		let mut dev = empty_handle();
		unsafe { vkCreateDevice(adapter.0, &info, std::ptr::null(), &mut dev) }.map(|| Device(dev, adapter.clone()))
	}
	pub fn queue_at(&self, family_index: u32, index: u32) -> Queue
	{
		let mut q = empty_handle();
		unsafe { vkGetDeviceQueue(self.0, family_index, index, &mut q) };
		Queue(q, family_index)
	}

	pub fn wait_for_idle(&self) -> Result<(), VkResult> { unsafe { vkDeviceWaitIdle(self.0) }.to_result() }
	pub fn updata_descriptor_sets(&self, write_infos: &[VkWriteDescriptorSet], copy_infos: &[VkCopyDescriptorSet])
	{
		unsafe { vkUpdateDescriptorSets(self.0, write_infos.len() as u32, write_infos.as_ptr(), copy_infos.len() as u32, copy_infos.as_ptr()) };
	}
}
pub struct Queue(VkQueue, u32);
impl Deref for Queue { type Target = VkQueue; fn deref(&self) -> &VkQueue { &self.0 } }
unsafe impl Sync for Queue {}
impl Queue
{
	pub fn submit(&self, infos: &[VkSubmitInfo], event_receiver: Option<VkFence>) -> Result<(), VkResult>
	{
		unsafe { vkQueueSubmit(self.0, infos.len() as u32, infos.as_ptr(), event_receiver.unwrap_or_else(empty_handle)) }.to_result()
	}
	pub fn wait_for_idle(&self) -> Result<(), VkResult> { unsafe { vkQueueWaitIdle(self.0) }.to_result() }
	pub fn family_index(&self) -> u32 { self.1 }
}

// --- Device-Child Object Definitions --- //
macro_rules! DeviceChildObject
{
	($name: ident ($vkname: ident) : $df: path) =>
	{
		#[derive(Debug)] pub struct $name($vkname, Rc<Device>);
		impl Drop for $name { fn drop(&mut self) { unsafe { $df(***self.parent(), **self, std::ptr::null()) }; } }
		impl Deref for $name { type Target = $vkname; fn deref(&self) -> &Self::Target { &self.0 } }
		impl HasParent for $name { type Parent = Rc<Device>; fn parent(&self) -> &Rc<Device> { &self.1 } }
	}
}
macro_rules! DeviceChildDefaultNewMethod
{
	($name: ident : $st: ty > $f: path) =>
	{
		impl $name
		{
			pub fn new(device: &Rc<Device>, info: &$st) -> VkWrapResult<Self>
			{
				let mut o = empty_handle();
				unsafe { $f(***device, info, std::ptr::null(), &mut o) }.map(|| $name(o, device.clone()))
			}
		}
	}
}

// Synchronize around GPUs //
DeviceChildObject!(Fence(VkFence): vkDestroyFence);
DeviceChildObject!(Semaphore(VkSemaphore): vkDestroySemaphore);
impl Fence
{
	pub fn new(device: &Rc<Device>) -> VkWrapResult<Self>
	{
		let info = VkFenceCreateInfo { sType: VkStructureType::FenceCreateInfo, pNext: std::ptr::null(), flags: 0 };
		let mut fence = empty_handle();
		unsafe { vkCreateFence(***device, &info, std::ptr::null(), &mut fence) }.map(|| Fence(fence, device.clone()))
	}
	pub fn wait(&self) -> VkWrapResult<()> { unsafe { vkWaitForFences(***self.parent(), 1, &**self, true as VkBool32, std::u64::MAX) }.to_result() }
	pub fn reset(&self) -> VkWrapResult<()> { unsafe { vkResetFences(***self.parent(), 1, &**self) }.to_result() }
}
impl Semaphore
{
	pub fn new(device: &Rc<Device>) -> VkWrapResult<Self>
	{
		let info = VkSemaphoreCreateInfo { sType: VkStructureType::SemaphoreCreateInfo, pNext: std::ptr::null(), flags: 0 };
		let mut o = empty_handle();
		unsafe { vkCreateSemaphore(***device, &info, std::ptr::null(), &mut o) }.map(|| Semaphore(o, device.clone()))
	}
}

// Device Resources //
DeviceChildObject!(DeviceMemory(VkDeviceMemory): vkFreeMemory);
DeviceChildObject!(Buffer(VkBuffer): vkDestroyBuffer);
DeviceChildObject!(Image(VkImage): vkDestroyImage);
DeviceChildObject!(ImageView(VkImageView): vkDestroyImageView);
impl DeviceMemory
{
	pub fn alloc(device: &Rc<Device>, info: &VkMemoryAllocateInfo) -> VkWrapResult<Self>
	{
		let mut mem = empty_handle();
		unsafe { vkAllocateMemory(***device, info, std::ptr::null(), &mut mem) }.map(|| DeviceMemory(mem, device.clone()))
	}
	pub fn map<T>(&self, range: std::ops::Range<VkDeviceSize>) -> VkWrapResult<*mut T>
	{
		let mut data_ptr = std::ptr::null_mut();
		unsafe { vkMapMemory(***self.parent(), **self, range.start, range.end - range.start, 0, std::mem::transmute(&mut data_ptr)) }.map(|| data_ptr as *mut T)
	}
	pub fn unmap(&self) { unsafe { vkUnmapMemory(***self.parent(), **self) } }
	pub fn bind_buffer(&self, res: &Buffer, offset: VkDeviceSize) -> VkWrapResult<()>
	{
		unsafe { vkBindBufferMemory(***self.parent(), **res, **self, offset) }.to_result()
	}
	pub fn bind_image(&self, res: &Image, offset: VkDeviceSize) -> VkWrapResult<()>
	{
		unsafe { vkBindImageMemory(***self.parent(), **res, **self, offset) }.to_result()
	}
}
DeviceChildDefaultNewMethod!(Buffer: VkBufferCreateInfo > vkCreateBuffer);
DeviceChildDefaultNewMethod!(Image: VkImageCreateInfo > vkCreateImage);
DeviceChildDefaultNewMethod!(ImageView: VkImageViewCreateInfo > vkCreateImageView);
impl MemoryAllocationRequired for Buffer
{
	fn get_memory_requirements(&self) -> VkMemoryRequirements { DeviceDataGetter!(***self.parent(), **self => vkGetBufferMemoryRequirements) }
}
impl MemoryAllocationRequired for Image
{
	fn get_memory_requirements(&self) -> VkMemoryRequirements { DeviceDataGetter!(***self.parent(), **self => vkGetImageMemoryRequirements) }
}

// RenderPass and Framebuffer //
DeviceChildObject!(RenderPass(VkRenderPass): vkDestroyRenderPass);
DeviceChildObject!(Framebuffer(VkFramebuffer): vkDestroyFramebuffer);
DeviceChildDefaultNewMethod!(RenderPass: VkRenderPassCreateInfo > vkCreateRenderPass);
DeviceChildDefaultNewMethod!(Framebuffer: VkFramebufferCreateInfo > vkCreateFramebuffer);

// Commands //
DeviceChildObject!(CommandPool(VkCommandPool): vkDestroyCommandPool);
impl CommandPool
{
	pub fn new(device: &Rc<Device>, queue: &Queue, transient: bool) -> VkWrapResult<Self>
	{
		let flags = if transient { VK_COMMAND_POOL_CREATE_TRANSIENT_BIT } else { 0 };
		let info = VkCommandPoolCreateInfo
		{
			sType: VkStructureType::CommandPoolCreateInfo, pNext: std::ptr::null(),
			flags: flags, queueFamilyIndex: queue.1
		};
		let mut o = empty_handle();
		unsafe { vkCreateCommandPool(***device, &info, std::ptr::null(), &mut o) }.map(|| CommandPool(o, device.clone()))
	}
	pub fn allocate(&self, buffer_level: VkCommandBufferLevel, count: usize) -> VkWrapResult<Vec<VkCommandBuffer>>
	{
		let info = VkCommandBufferAllocateInfo
		{
			sType: VkStructureType::CommandBufferAllocateInfo, pNext: std::ptr::null(),
			commandPool: **self, level: buffer_level, commandBufferCount: count as u32
		};
		let mut bufs = vec![empty_handle(); count];
		unsafe { vkAllocateCommandBuffers(***self.parent(), &info, bufs.as_mut_ptr()) }.map(|| bufs)
	}
}

// ShaderModule and Pipelines //
DeviceChildObject!(ShaderModule(VkShaderModule): vkDestroyShaderModule);
DeviceChildObject!(PipelineLayout(VkPipelineLayout): vkDestroyPipelineLayout);
DeviceChildObject!(PipelineCache(VkPipelineCache): vkDestroyPipelineCache);
DeviceChildObject!(Pipeline(VkPipeline): vkDestroyPipeline);
impl ShaderModule
{
	pub fn new(device: &Rc<Device>, binary: &[u8]) -> VkWrapResult<Self>
	{
		let info = VkShaderModuleCreateInfo
		{
			sType: VkStructureType::ShaderModuleCreateInfo, pNext: std::ptr::null(), flags: 0,
			codeSize: binary.len() as size_t, pCode: unsafe { std::mem::transmute(binary.as_ptr()) }
		};
		let mut m = empty_handle();
		unsafe { vkCreateShaderModule(***device, &info, std::ptr::null(), &mut m) }.map(|| ShaderModule(m, device.clone()))
	}
}
impl PipelineLayout
{
	pub fn new(device: &Rc<Device>, ds_layouts: &[VkDescriptorSetLayout], push_constants: &[VkPushConstantRange]) -> VkWrapResult<Self>
	{
		let info = VkPipelineLayoutCreateInfo
		{
			sType: VkStructureType::PipelineLayoutCreateInfo, pNext: std::ptr::null(), flags: 0,
			setLayoutCount: ds_layouts.len() as u32, pSetLayouts: ds_layouts.as_ptr(),
			pushConstantRangeCount: push_constants.len() as u32, pPushConstantRanges: push_constants.as_ptr()
		};
		let mut l = empty_handle();
		unsafe { vkCreatePipelineLayout(***device, &info, std::ptr::null(), &mut l) }.map(|| PipelineLayout(l, device.clone()))
	}

}
impl PipelineCache
{
	#[allow(dead_code)]
	pub fn new_empty(device: &Rc<Device>, initial_data: &[u8]) -> VkWrapResult<Self>
	{
		let info = VkPipelineCacheCreateInfo
		{
			sType: VkStructureType::PipelineCacheCreateInfo, pNext: std::ptr::null(), flags: 0,
			pInitialData: unsafe { std::mem::transmute(initial_data.as_ptr()) }, initialDataSize: initial_data.len() as size_t
		};
		let mut c = empty_handle();
		unsafe { vkCreatePipelineCache(***device, &info, std::ptr::null(), &mut c) }.map(|| PipelineCache(c, device.clone()))
	}
}
impl Pipeline
{
	pub fn new_graphics(device: &Rc<Device>, cache: Option<&PipelineCache>, infos: &[VkGraphicsPipelineCreateInfo]) -> VkWrapResult<Vec<Self>>
	{
		let mut objs = vec![empty_handle(); infos.len()];
		unsafe { vkCreateGraphicsPipelines(***device, cache.map(|&PipelineCache(a, _)| a).unwrap_or(empty_handle()), infos.len() as u32, infos.as_ptr(),
			std::ptr::null(), objs.as_mut_ptr()) }.map(|| objs.into_iter().map(|x| Pipeline(x, device.clone())).collect())
	}
}

// Descriptors //
DeviceChildObject!(DescriptorSetLayout(VkDescriptorSetLayout): vkDestroyDescriptorSetLayout);
DeviceChildObject!(DescriptorPool(VkDescriptorPool): vkDestroyDescriptorPool);
impl DescriptorSetLayout
{
	pub fn new(device: &Rc<Device>, bindings: &[VkDescriptorSetLayoutBinding]) -> VkWrapResult<Self>
	{
		let info = VkDescriptorSetLayoutCreateInfo
		{
			sType: VkStructureType::DescriptorSetLayoutCreateInfo, pNext: std::ptr::null(), flags: 0,
			bindingCount: bindings.len() as u32, pBindings: bindings.as_ptr()
		};
		let mut dsl = empty_handle();
		unsafe { vkCreateDescriptorSetLayout(***device, &info, std::ptr::null(), &mut dsl) }.map(|| DescriptorSetLayout(dsl, device.clone()))
	}
}
impl DescriptorPool
{
	pub fn new(device: &Rc<Device>, max_sets: usize, pool_sizes: &[VkDescriptorPoolSize]) -> VkWrapResult<Self>
	{
		let info = VkDescriptorPoolCreateInfo
		{
			sType: VkStructureType::DescriptorPoolCreateInfo, pNext: std::ptr::null(), flags: 0,
			maxSets: max_sets as u32, poolSizeCount: pool_sizes.len() as u32, pPoolSizes: pool_sizes.as_ptr()
		};
		let mut pool = empty_handle();
		unsafe { vkCreateDescriptorPool(***device, &info, std::ptr::null(), &mut pool) }.map(|| DescriptorPool(pool, device.clone()))
	}
	pub fn allocate(&self, layouts: &[VkDescriptorSetLayout]) -> VkWrapResult<Vec<VkDescriptorSet>>
	{
		let info = VkDescriptorSetAllocateInfo
		{
			sType: VkStructureType::DescriptorSetAllocateInfo, pNext: std::ptr::null(),
			descriptorPool: **self, descriptorSetCount: layouts.len() as u32, pSetLayouts: layouts.as_ptr()
		};
		let mut objs = vec![empty_handle(); layouts.len()];
		unsafe { vkAllocateDescriptorSets(***self.parent(), &info, objs.as_mut_ptr()) }.map(|| objs)
	}
}

// Sampler //
DeviceChildObject!(Sampler(VkSampler): vkDestroySampler);
DeviceChildDefaultNewMethod!(Sampler: VkSamplerCreateInfo > vkCreateSampler);

// --- WindowSystemIntegration Extensions --- ///

pub struct Swapchain<Surface>(VkSwapchainKHR, Rc<Surface>, Rc<Device>);
impl<Surface> Drop for Swapchain<Surface> { fn drop(&mut self) { unsafe { vkDestroySwapchainKHR(**self.parent(), **self, std::ptr::null()) }; } }
impl<Surface> Deref for Swapchain<Surface> { type Target = VkSwapchainKHR; fn deref(&self) -> &VkSwapchainKHR { &self.0 } }
impl<Surface> HasParent for Swapchain<Surface> { type Parent = Device; fn parent(&self) -> &Device { &self.2 } }
impl<Surface> Swapchain<Surface>
{
	pub fn new(device: &Rc<Device>, surface: &Rc<Surface>, info: &VkSwapchainCreateInfoKHR) -> VkWrapResult<Self>
	{
		let mut sc = empty_handle();
		unsafe { vkCreateSwapchainKHR(***device, info, std::ptr::null(), &mut sc) }.map(|| Swapchain(sc, surface.clone(), device.clone()))
	}
	pub fn images(&self) -> VkWrapResult<Vec<VkImage>> { DeviceDataEnumerator!(**self.parent(), **self => vkGetSwapchainImagesKHR) }
	pub fn acquire_next(&self, device_synchronizer: VkSemaphore) -> VkWrapResult<u32>
	{
		let mut index = 0u32;
		unsafe { vkAcquireNextImageKHR(**self.parent(), **self, std::u64::MAX, device_synchronizer, std::ptr::null_mut(), &mut index) }
			.map(|| index)
	}
	pub fn present(&self, queue: &Queue, index: u32, device_synchronizer: &[VkSemaphore]) -> VkWrapResult<()>
	{
		let info = VkPresentInfoKHR
		{
			sType: VkStructureType::PresentInfoKHR, pNext: std::ptr::null(),
			swapchainCount: 1, pSwapchains: &**self, pImageIndices: &index,
			waitSemaphoreCount: device_synchronizer.len() as u32, pWaitSemaphores: device_synchronizer.as_ptr(),
			pResults: std::ptr::null_mut()
		};
		unsafe { vkQueuePresentKHR(**queue, &info) }.to_result()
	}
}

pub enum ImageSubresourceRange {}
impl ImageSubresourceRange
{
	pub fn default_color() -> VkImageSubresourceRange
	{
		VkImageSubresourceRange
		{
			aspectMask: VK_IMAGE_ASPECT_COLOR_BIT, baseMipLevel: 0, baseArrayLayer: 0, levelCount: 1, layerCount: 1
		}
	}
}

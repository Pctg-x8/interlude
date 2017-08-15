//! Interlude Graphics Interface

use interlude_vk_defs::*;
use interlude_vk_funport::*;
use {EngineResult, EngineError, DescriptorSetWriteInfo, UnrecoverableExt};
use libc::{c_void, c_char, size_t};
use std::ffi::{CString, CStr};
use std::ptr::{null, null_mut};
use std::mem::{uninitialized as reserved, transmute};
use subsystem_layer::{NativeInstance, NativeHandleProvider, NativeResultValueHandler};
use command::CommandPool;
use device::Device;
use descriptor::IntoWriteDescriptorSetNativeStruct;
use std::rc::Rc;

#[cfg(windows)]
static PLATFORM_SURFACE_EXTENSION_NAME: &&'static str = &VK_KHR_WIN32_SURFACE_EXTENSION_NAME;
#[cfg(feature = "target_xlib")]
static PLATFORM_SURFACE_EXTENSION_NAME: &&'static str = &VK_KHR_XLIB_SURFACE_EXTENSION_NAME;

fn bool_to_str(v: VkBool32) -> &'static str { if v == true as VkBool32 { "true" } else { "false" } }

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
	pub fn enable_multidraw_indirect(&mut self) -> &mut Self
	{
		self.0.multiDrawIndirect = true as VkBool32;
		self
	}
	pub fn enable_draw_indirect_first_instance(&mut self) -> &mut Self
	{
		self.0.drawIndirectFirstInstance = true as VkBool32;
		self
	}
	pub fn enable_block_texture_compression(&mut self) -> &mut Self
	{
		self.0.textureCompressionBC = true as VkBool32;
		self
	}
	pub fn enable_nonsolid_fillmode(&mut self) -> &mut Self
	{
		self.0.fillModeNonSolid = true as VkBool32;
		self
	}
}

// QueryTypes
pub enum MemoryIndexType { DeviceLocal, HostVisible }
pub struct MemoryTypeIndices { device_local: u32, host_visible: u32 }

pub struct DebugReportCallback
{
	obj: VkDebugReportCallbackEXT, parent: Rc<NativeInstance>,
	destroy_fn: PFN_vkDestroyDebugReportCallbackEXT
}
impl DebugReportCallback
{
	fn new(instance: &Rc<NativeInstance>) -> EngineResult<Self>
	{
		let create_fn: PFN_vkCreateDebugReportCallbackEXT = unsafe { transmute(vkGetInstanceProcAddr(instance.native(), "vkCreateDebugReportCallbackEXT\x00".as_ptr() as _)) };
		let destroy_fn = unsafe { transmute(vkGetInstanceProcAddr(instance.native(), "vkDestroyDebugReportCallbackEXT\x00".as_ptr() as _)) };
		let mut obj = unsafe { reserved() };
		create_fn(instance.native(), &VkDebugReportCallbackCreateInfoEXT
		{
			flags: VK_DEBUG_REPORT_ERROR_BIT_EXT | VK_DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT | VK_DEBUG_REPORT_WARNING_BIT_EXT,
			pfnCallback: debug_callback, .. Default::default()
		}, null(), &mut obj).make_result_with(|| DebugReportCallback { obj, destroy_fn, parent: instance.clone() })
	}
}
impl Drop for DebugReportCallback
{
	fn drop(&mut self) { (self.destroy_fn)(self.parent.native(), self.obj, null()); }
}

macro_rules! LogAdapterFeature
{
	($f: expr => $($x: ident),*) => { $(info!(target: "Interlude::DiagAdapter", "-- {}: {}", stringify!($x), bool_to_str($f.$x));)* }
}

pub struct GraphicsInterface
{
	instance: (Rc<NativeInstance>, DebugReportCallback), device: Rc<Device>, pools: CommandPool,
	pub device_limits: VkPhysicalDeviceLimits, memory_types: MemoryTypeIndices
}
impl GraphicsInterface
{
	pub fn new(app_name: &str, app_version: u32, device_features: &DeviceFeatures) -> EngineResult<Self>
	{
		let engine_name_c = CString::new("Interlude Multimedia Framework")?;
		let app_name_c = CString::new(app_name)?;
		let appinfo = VkApplicationInfo
		{
			pApplicationName: app_name_c.as_ptr(), applicationVersion: app_version,
			pEngineName: engine_name_c.as_ptr(), engineVersion: VK_MAKE_VERSION!(0, 1, 0),
			.. Default::default()
		};
		let enabled_layers = ["VK_LAYER_LUNARG_standard_validation\x00"];
		let enabled_extensions = [VK_KHR_SURFACE_EXTENSION_NAME, PLATFORM_SURFACE_EXTENSION_NAME, VK_EXT_DEBUG_REPORT_EXTENSION_NAME];
		let instance = NativeInstance::new(&VkInstanceCreateInfo
		{
			pApplicationInfo: &appinfo, enabledLayerCount: enabled_layers.len() as _, ppEnabledLayerNames: enabled_layers.as_ptr() as _,
			enabledExtensionCount: enabled_extensions.len() as _, ppEnabledExtensionNames: enabled_extensions.as_ptr() as _,
			.. Default::default()
		}).map(Rc::new)?;
		let debug_report_callback = DebugReportCallback::new(&instance)?;

		let mut devcount = 0;
		unsafe { vkEnumeratePhysicalDevices(instance.native(), &mut devcount, null_mut()) }.into_result()?;
		let mut pdevs = vec![unsafe { reserved() }; devcount as _];
		unsafe { vkEnumeratePhysicalDevices(instance.native(), &mut devcount, pdevs.as_mut_ptr()) }.into_result()?;
		if pdevs.is_empty() { return Err(EngineError::GenericError("PhysicalDevices are not found")); }
		let pdev = pdevs.swap_remove(0);
		Self::diagnose_pdev(pdev);
		let mut qfcount = 0;
		unsafe { vkGetPhysicalDeviceQueueFamilyProperties(pdev, &mut qfcount, null_mut()) };
		let mut queue_family_properties = vec![unsafe { reserved() }; qfcount as _];
		unsafe { vkGetPhysicalDeviceQueueFamilyProperties(pdev, &mut qfcount, queue_family_properties.as_mut_ptr()) };
		let gqf = queue_family_properties.iter().enumerate().find(|&(_, fp)| (fp.queueFlags & VK_QUEUE_GRAPHICS_BIT) != 0).map(|(i, _)| i as u32)
			.ok_or(EngineError::GenericError("Unable to find graphics queue"))?;
		let tqf = queue_family_properties.iter().enumerate().find(|&(_, fp)| (fp.queueFlags & VK_QUEUE_TRANSFER_BIT) != 0).map(|(i, _)| i as u32);
		let device = Device::new(pdev, &device_features.0, gqf, tqf, &queue_family_properties[gqf as usize]).map(Rc::new)?;
		let pools = CommandPool::new(&device)?;

		let mut memory_types = unsafe { reserved() };
		unsafe { vkGetPhysicalDeviceMemoryProperties(pdev, &mut memory_types) };
		let mut device_properties = unsafe { reserved() };
		unsafe { vkGetPhysicalDeviceProperties(pdev, &mut device_properties) };
		let mt_indices = MemoryTypeIndices::find_from(&memory_types)?;

		Ok(GraphicsInterface
		{
			instance: (instance, debug_report_callback), device, pools,
			device_limits: device_properties.limits, memory_types: mt_indices
		})
	}
	pub fn apicontext(&self) -> &Rc<NativeInstance> { &self.instance.0 }
	pub fn device(&self) -> &Rc<Device> { &self.device }
	pub fn pools(&self) -> &CommandPool { &self.pools }
	pub fn update_descriptors(&self, write_infos: &[DescriptorSetWriteInfo])
	{
		let write_infos_native_interp = write_infos.into_iter().map(Into::into).collect::<Vec<IntoWriteDescriptorSetNativeStruct>>();
		let write_infos_native = write_infos_native_interp.iter().map(Into::into).collect::<Vec<_>>();
		unsafe { vkUpdateDescriptorSets(self.device.native(), write_infos_native.len() as u32, write_infos_native.as_ptr(), 0, null()) };
	}
	pub fn wait_device(&self) -> EngineResult<()> { self.device.wait_for_idle() }

	fn diagnose_pdev(pdev: VkPhysicalDevice)
	{
		// Feature Check //
		let mut features = unsafe { reserved() };
		unsafe { vkGetPhysicalDeviceFeatures(pdev, &mut features) };
		info!(target: "Interlude::DiagAdapter", "adapter features");
		LogAdapterFeature!(features =>
			independentBlend, geometryShader, multiDrawIndirect, drawIndirectFirstInstance, shaderTessellationAndGeometryPointSize,
			depthClamp, depthBiasClamp, wideLines, alphaToOne, multiViewport, shaderCullDistance, shaderClipDistance, shaderResourceResidency);
		// if features.depthClamp == false as VkBool32 { panic!("DepthClamp Feature is required in device"); }
	}
	pub fn surface_support(&self, s: VkSurfaceKHR) -> EngineResult<bool>
	{
		let mut ret = 0;
		unsafe { vkGetPhysicalDeviceSurfaceSupportKHR(self.device.adapter(), self.device.graphics_qf_index, s, &mut ret) }
			.make_result_with(|| ret == true as VkBool32)
	}
	pub fn memindex(&self, t: MemoryIndexType) -> u32
	{
		match t
		{
			MemoryIndexType::DeviceLocal => self.memory_types.device_local,
			MemoryIndexType::HostVisible => self.memory_types.host_visible
		}
	}
}
impl Drop for GraphicsInterface
{
	fn drop(&mut self) { self.device.wait_for_idle().or_crash(); }
}
impl MemoryTypeIndices
{
	fn find_from(p: &VkPhysicalDeviceMemoryProperties) -> EngineResult<Self>
	{
		let mtdx_find_by_flag = |bits| p.memoryTypes[..p.memoryTypeCount as usize].iter().enumerate().find(|&(_, mt)| (mt.propertyFlags & bits) != 0);
		let (d, _) = mtdx_find_by_flag(VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT).ok_or(EngineError::GenericError("Device Local Memory could not be found"))?;
		let (h, _) = mtdx_find_by_flag(VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT).ok_or(EngineError::GenericError("Host Visible Memory could not be found"))?;
		info!(target: "Interlude", "MemoryType Indices: DeviceLocal={} HostVisible={}", d, h);
		Ok(MemoryTypeIndices { device_local: d as _, host_visible: h as _ })
	}
}

extern "system" fn debug_callback(flags: VkDebugReportFlagsEXT, object_type: VkDebugReportObjectTypeEXT, _: u64,
	_: size_t, message_code: i32, _: *const c_char, message: *const c_char, _: *mut c_void) -> VkBool32
{
	if (flags & VK_DEBUG_REPORT_ERROR_BIT_EXT) != 0
	{
		error!(target: format!("Vulkan DebugCall [{:?}]", object_type).as_str(), "({}){}", message_code, unsafe { CStr::from_ptr(message).to_str().unwrap() });
	}
	else if (flags & VK_DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT) != 0
	{
		warn!(target: format!("Vulkan PerformanceDebug [{:?}]", object_type).as_str(), "({}){}", message_code, unsafe { CStr::from_ptr(message).to_str().unwrap() });
	}
	else if (flags & VK_DEBUG_REPORT_WARNING_BIT_EXT) != 0
	{
		warn!(target: format!("Vulkan DebugCall [{:?}]", object_type).as_str(), "({}){}", message_code, unsafe { CStr::from_ptr(message).to_str().unwrap() });
	}
	else
	{
		info!(target: format!("Vulkan DebugCall [{:?}]", object_type).as_str(), "({}){}", message_code, unsafe { CStr::from_ptr(message).to_str().unwrap() });
	}
	false as VkBool32
}

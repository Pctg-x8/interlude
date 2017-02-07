//! Interlude Graphics Interface

use {UnrecoverableExt, DescriptorSetWriteInfo, EngineResult};
use descriptor::IntoWriteDescriptorSetNativeStruct;
use {vk, std};
use vkdefs::*;
use vk::vkUpdateDescriptorSets;
use std::rc::Rc;
use device::Device;
use command::CommandPool;
use error::EngineError;
use libc::{c_void, size_t, c_char};
use std::ffi::CStr;
use tuple_tools::*;
use std::borrow::Cow;

#[cfg(windows)] const PSURFACE_EXNAME: &'static str = "VK_KHR_win32_surface";
#[cfg(unix)] const PSURFACE_EXNAME: &'static str = "VK_KHR_xcb_surface";

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
pub struct GraphicsInterface
{
	instance: (Rc<vk::Instance>, vk::DebugReportCallback), device: Device, pools: CommandPool,
	pub device_limits: VkPhysicalDeviceLimits, memory_types: MemoryTypeIndices
}
impl GraphicsInterface
{
	pub fn new(app_name: Cow<'static, str>, app_version: u32, device_features: &DeviceFeatures) -> EngineResult<Self>
	{
		let instance = try!(vk::Instance::new(app_name, app_version, "Interlude Multimedia Framework".into(), VK_MAKE_VERSION!(0, 0, 1),
			&["VK_LAYER_LUNARG_standard_validation"], &["VK_KHR_surface", PSURFACE_EXNAME, "VK_EXT_debug_report"]).map(Rc::new));
		let debug_report_callback = try!(vk::DebugReportCallback::new(&instance, debug_callback));
		let pdev = try!
		{
			instance.adapters().map_err(EngineError::from)
				.and_then(|aa| aa.into_iter().next().ok_or(EngineError::GenericError("PhysicalDevices are not found")))
				.map(|a| Rc::new(vk::PhysicalDevice::from(a, &instance)))
		};
		let device = try!(
		{
			let queue_family_properties = pdev.queue_family_properties();
			let qf_enumerated = || queue_family_properties.iter().enumerate();
			let qf_filtered = |v| qf_enumerated().filter(move |&(i, _)| i != v);
			qf_enumerated().find(|&(_, fp)| (fp.queueFlags & VK_QUEUE_GRAPHICS_BIT) != 0).map(|(i, _)| i as u32).ok_or(EngineError::GenericError("Unable to find Graphics Queue")).map(|gqf|
				(gqf, qf_filtered(gqf as usize).find(|&(_, fp)| (fp.queueFlags & VK_QUEUE_TRANSFER_BIT) != 0).map(|(i, _)| i as u32))).and_then(|(gqf, tqf)|
			{
				Self::diagnose_pdev(&pdev);
				Device::new(&pdev, &device_features.0, gqf, tqf, &queue_family_properties[gqf as usize])
			})
		});
		let pools = try!(CommandPool::new(&device));

		let memory_types = pdev.memory_properties();
		let mt_indices = try!(MemoryTypeIndices::find_from(&memory_types));

		Ok(GraphicsInterface
		{
			instance: (instance, debug_report_callback), device: device, pools: pools,
			device_limits: pdev.properties().limits, memory_types: mt_indices
		})
	}
	pub fn apicontext(&self) -> &Rc<vk::Instance> { &self.instance.0 }
	pub fn device(&self) -> &Device { &self.device }
	pub fn pools(&self) -> &CommandPool { &self.pools }
	pub fn update_descriptors(&self, write_infos: &[DescriptorSetWriteInfo])
	{
		let write_infos_native_interp = write_infos.into_iter().map(Into::<IntoWriteDescriptorSetNativeStruct>::into).collect::<Vec<_>>();
		let write_infos_native = write_infos_native_interp.iter().map(Into::<VkWriteDescriptorSet>::into).collect::<Vec<_>>();
		unsafe { vkUpdateDescriptorSets(***self.device, write_infos_native.len() as u32, write_infos_native.as_ptr(),
			0, std::ptr::null()) };
	}
	pub fn wait_device(&self) -> EngineResult<()> { self.device.wait_for_idle() }

	fn diagnose_pdev(pdev: &vk::PhysicalDevice)
	{
		// Feature Check //
		let features = pdev.features();
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
	}
	pub fn ensure_surface_support(&self, s: &VkSurfaceKHR) -> EngineResult<()>
	{
		if self.device.is_surface_support(s) { Ok(()) } else { Err(EngineError::GenericError("Unsupported Surface")) }
	}
	pub fn surface_caps(&self, s: &VkSurfaceKHR) -> VkSurfaceCapabilitiesKHR
	{
		self.device.adapter().surface_caps(s)
	}
	pub fn surface_formats(&self, s: &VkSurfaceKHR) -> EngineResult<Vec<VkSurfaceFormatKHR>>
	{
		self.device.adapter().surface_formats(s).map_err(EngineError::from)
	}
	pub fn surface_present_modes(&self, s: &VkSurfaceKHR) -> EngineResult<Vec<VkPresentModeKHR>>
	{
		self.device.adapter().present_modes(s).map_err(EngineError::from)
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
	fn find_from(p: &VkPhysicalDeviceMemoryProperties) -> Result<Self, EngineError>
	{
		let mtdx_find_by_flag = |bits| p.memoryTypes[..p.memoryTypeCount as usize].iter().enumerate().find(|&(_, &VkMemoryType(f, _))| (f & bits) != 0);
		let d = mtdx_find_by_flag(VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT).ok_or(EngineError::GenericError("Device Local Memory could not be found"));
		let h = mtdx_find_by_flag(VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT).ok_or(EngineError::GenericError("Host Visible Memory could not be found"));

		(d, h).flatten().map(|((d, _), (h, _))|
		{
			info!(target: "Interlude", "MemoryType Indices: DeviceLocal={}, HostVisible={}", d, h);
			MemoryTypeIndices { device_local: d as u32, host_visible: h as u32 }
		})
	}
}

unsafe extern "system" fn debug_callback(flags: VkDebugReportFlagsEXT, object_type: VkDebugReportObjectTypeEXT, _: u64,
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

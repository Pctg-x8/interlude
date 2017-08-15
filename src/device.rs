// Prelude: Device Structure

use EngineResult;
use interlude_vk_defs::*;
use interlude_vk_funport::*;
use subsystem_layer::{NativeResultValueHandler, NativeHandleProvider};
use std::ptr::null;
use std::mem::uninitialized as reserved;
use std::cmp::min;

pub struct Device
{
	adapter: VkPhysicalDevice, internal: VkDevice,
	pub graphics_queue: VkQueue, pub transfer_queue: VkQueue,
	pub graphics_qf_index: u32, pub transfer_qf_index: u32
}
impl Device
{
	pub fn new(adapter: VkPhysicalDevice, features: &VkPhysicalDeviceFeatures,
		graphics_qf: u32, transfer_qf: Option<u32>, qf_props: &VkQueueFamilyProperties) -> EngineResult<Self>
	{
		// Ready Parameters //
		static QUEUE_PRIORITIES: [f32; 2] = [0.0f32; 2];
		match transfer_qf
		{
			Some(t) => info!(target: "Interlude", "Individual queue family: g={}, t={}", graphics_qf, t),
			None => info!(target: "Interlude", "Sharing queue family: {}", graphics_qf)
		};
		let queue_info = match transfer_qf
		{
			Some(transfer_qf) => vec![
				VkDeviceQueueCreateInfo { queueFamilyIndex: graphics_qf, queueCount: 1, pQueuePriorities: &QUEUE_PRIORITIES[0], .. Default::default() },
				VkDeviceQueueCreateInfo { queueFamilyIndex: transfer_qf, queueCount: 1, pQueuePriorities: &QUEUE_PRIORITIES[1], .. Default::default() },
			],
			None => vec![VkDeviceQueueCreateInfo
			{
				queueFamilyIndex: graphics_qf, queueCount: min(qf_props.queueCount, 2),
				pQueuePriorities: QUEUE_PRIORITIES.as_ptr(), .. Default::default()
			}]
		};
		let transfer_qf = transfer_qf.unwrap_or(graphics_qf);
		let enabled_layers = ["VK_LAYER_LUNARG_standard_validation\x00"];
		let enabled_extensions = [VK_KHR_SWAPCHAIN_EXTENSION_NAME];
		let mut dev = unsafe { reserved() };
		let mut dev = unsafe { vkCreateDevice(adapter, &VkDeviceCreateInfo
		{
			queueCreateInfoCount: queue_info.len() as _, pQueueCreateInfos: queue_info.as_ptr(),
			enabledLayerCount: enabled_layers.len() as _, ppEnabledLayerNames: enabled_layers.as_ptr() as _,
			enabledExtensionCount: enabled_extensions.len() as _, ppEnabledExtensionNames: enabled_extensions.as_ptr() as _,
			pEnabledFeatures: features, .. Default::default()
		}, null(), &mut dev) }.make_result_with(|| Device
		{
			internal: dev, adapter: adapter.clone(), graphics_qf_index: graphics_qf, transfer_qf_index: transfer_qf, .. unsafe { reserved() }
		})?;
		unsafe { vkGetDeviceQueue(dev.internal, graphics_qf, 0, &mut dev.graphics_queue) };
		unsafe { vkGetDeviceQueue(dev.internal, transfer_qf, queue_info[0].queueCount - 1, &mut dev.transfer_queue) };
		Ok(dev)
	}
	pub fn wait_for_idle(&self) -> EngineResult<()>
	{
		unsafe { vkDeviceWaitIdle(self.internal) }.into_result()
	}

	pub fn has_surface_support(&self, surface: VkSurfaceKHR) -> EngineResult<bool>
	{
		let mut supported = 0;
		unsafe { vkGetPhysicalDeviceSurfaceSupportKHR(self.adapter, self.graphics_qf_index, surface, &mut supported) }
			.make_result_with(|| supported == true as VkBool32)
	}
	pub fn adapter(&self) -> VkPhysicalDevice { self.adapter }

	pub fn new_object<F: Fn(VkDevice, *const C, *const VkAllocationCallbacks, *mut T) -> VkResult, C, T>(&self, func: F, cinfo: &C) -> EngineResult<T>
	{
		let mut t = unsafe { reserved() };
		func(self.internal, cinfo, null(), &mut t).make_result_with(|| t)
	}
	pub fn destroy_object<F: Fn(VkDevice, T, *const VkAllocationCallbacks), T>(&self, func: F, object: T)
	{
		func(self.internal, object, null());
	}
}
impl Drop for Device
{
	fn drop(&mut self) { unsafe { vkDestroyDevice(self.internal, null()) }; }
}
impl NativeHandleProvider for Device
{
	type NativeT = VkDevice;
	fn native(&self) -> VkDevice { self.internal }
}

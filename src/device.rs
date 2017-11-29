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
		let enabled_layers = ["VK_LAYER_LUNARG_standard_validation\x00".as_ptr()];
		let enabled_extensions = ["VK_KHR_swapchain\x00".as_ptr()];
		let mut dev = unsafe { reserved() };
		unsafe { vkCreateDevice(adapter, &VkDeviceCreateInfo
		{
			queueCreateInfoCount: queue_info.len() as _, pQueueCreateInfos: queue_info.as_ptr(),
			enabledLayerCount: enabled_layers.len() as _, ppEnabledLayerNames: enabled_layers.as_ptr() as _,
			enabledExtensionCount: enabled_extensions.len() as _, ppEnabledExtensionNames: enabled_extensions.as_ptr() as _,
			pEnabledFeatures: features, .. Default::default()
		}, null(), &mut dev) }.into_result()?;
		let (mut graphics_queue, mut transfer_queue) = unsafe { reserved() };
		unsafe { vkGetDeviceQueue(dev, graphics_qf, 0, &mut graphics_queue) };
		unsafe { vkGetDeviceQueue(dev, transfer_qf, queue_info[0].queueCount - 1, &mut transfer_queue) };
		Ok(Device { internal: dev, adapter, graphics_qf_index: graphics_qf, transfer_qf_index: transfer_qf, graphics_queue, transfer_queue })
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
}
impl Drop for Device
{
	fn drop(&mut self) { info!(target: "Interlude", "Destroying Device..."); unsafe { vkDestroyDevice(self.internal, null()) }; }
}
impl NativeHandleProvider for Device
{
	type NativeT = VkDevice;
	fn native(&self) -> VkDevice { self.internal }
}

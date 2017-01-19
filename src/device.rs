// Prelude: Device Structure

use super::internals::*;
use {std, vk};
use vk::ffi::*;
use std::rc::Rc;

pub struct Device
{
	adapter: Rc<vk::PhysicalDevice>, internal: Rc<vk::Device>,
	pub graphics_qf_index: u32,
	pub graphics_queue: vk::Queue, pub transfer_queue: vk::Queue
}
impl std::ops::Deref for Device { type Target = Rc<vk::Device>; fn deref(&self) -> &Self::Target { &self.internal } }
impl Device
{
	pub fn new(adapter: &Rc<vk::PhysicalDevice>, features: &VkPhysicalDeviceFeatures,
		graphics_qf: u32, transfer_qf: Option<u32>, qf_props: &VkQueueFamilyProperties) -> Result<Self, EngineError>
	{
		fn device_queue_create_info(family_index: u32, count: u32, priorities: &[f32]) -> VkDeviceQueueCreateInfo
		{
			VkDeviceQueueCreateInfo
			{
				sType: VkStructureType::DeviceQueueCreateInfo, pNext: std::ptr::null(), flags: 0,
				queueFamilyIndex: family_index, queueCount: count, pQueuePriorities: priorities.as_ptr()
			}
		}
		// Ready Parameters //
		static QUEUE_PRIORITIES: [f32; 2] = [0.0f32; 2];
		match transfer_qf
		{
			Some(t) => info!(target: "Prelude", "Not sharing queue family: g={}, t={}", graphics_qf, t),
			None => info!(target: "Prelude", "Sharing queue family: {}", graphics_qf)
		};
		let queue_info = match transfer_qf
		{
			Some(transfer_qf) => vec![
				device_queue_create_info(graphics_qf, 1, &QUEUE_PRIORITIES[0..1]),
				device_queue_create_info(transfer_qf, 1, &QUEUE_PRIORITIES[1..2])
			],
			None => vec![device_queue_create_info(graphics_qf, std::cmp::min(qf_props.queueCount, 2), &QUEUE_PRIORITIES)]
		};
		vk::Device::new(adapter, &queue_info, &["VK_LAYER_LUNARG_standard_validation"], &["VK_KHR_swapchain"], features).map(|device| Device
		{
			graphics_qf_index: graphics_qf,
			graphics_queue: device.queue_at(graphics_qf, 0),
			transfer_queue: device.queue_at(transfer_qf.unwrap_or(graphics_qf), queue_info[0].queueCount - 1),
			internal: Rc::new(device), adapter: adapter.clone()
		}).map_err(|e| EngineError::from(e))
	}
	pub fn wait_for_idle(&self) -> Result<(), EngineError>
	{
		self.internal.wait_for_idle().map_err(EngineError::from)
	}

	pub fn is_surface_support(&self, surface: &vk::Surface) -> bool
	{
		self.adapter.is_surface_support(self.graphics_queue.family_index(), surface)
	}
	pub fn adapter(&self) -> &Rc<vk::PhysicalDevice> { &self.adapter }
}

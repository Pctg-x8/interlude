#![allow(non_camel_case_types, non_snake_case, dead_code)]

// Vulkan C to Rust FFI [Window System Integration]

use xcb::ffi::*;
use vk::*;
use libc::c_void;
use EngineResult;
use std;
use std::rc::Rc;
use vk::traits::*;

#[repr(C)] pub struct VkXcbSurfaceCreateInfoKHR
{
	pub sType: VkStructureType, pub pNext: *const c_void, pub flags: VkXcbSurfaceCreateFlagsKHR,
	pub connection: *mut xcb_connection_t, pub window: xcb_window_t
}

#[link(name = "vulkan")]
extern "system"
{
	pub fn vkCreateXcbSurfaceKHR(instance: VkInstance, pCreateInfo: *const VkXcbSurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
	pub fn vkGetPhysicalDeviceXcbPresentationSupportKHR(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, con: *mut xcb_connection_t, window: xcb_window_t) -> VkBool32;
}

pub struct Surface(VkSurfaceKHR, Rc<Instance>);
impl Surface
{
	pub fn new(instance: &Rc<Instance>, info: &VkXcbSurfaceCreateInfoKHR) -> EngineResult<Self>
	{
		let mut surf = empty_handle();
		unsafe { vkCreateXcbSurfaceKHR(***instance, info, std::ptr::null(), &mut surf) }
			.map(|| Surface(surf, instance.clone())).map_err(From::from)
	}
}
impl Drop for Surface { fn drop(&mut self) { unsafe { vkDestroySurfaceKHR(**self.1, **self, std::ptr::null()) }; } }
impl std::ops::Deref for Surface { type Target = VkSurfaceKHR; fn deref(&self) -> &VkSurfaceKHR { &self.0 } }

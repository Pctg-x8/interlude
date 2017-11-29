#![allow(non_snake_case)]

use {std, libc};
use vk::*;
use winapi::*;
use std::rc::Rc;
use std::ops::Deref;
use EngineResult;

pub type VkWin32SurfaceCreateFlagsKHR = VkFlags;
#[repr(C)]
pub struct VkWin32SurfaceCreateInfoKHR
{
	pub sType: VkStructureType, pub pNext: *const libc::c_void, pub flags: VkWin32SurfaceCreateFlagsKHR,
	pub hinstance: HINSTANCE, pub hwnd: HWND
}

#[link(name="vulkan-1")]
extern "system"
{
	fn vkCreateWin32SurfaceKHR(instance: VkInstance, pCreateInfo: *const VkWin32SurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
	pub fn vkGetPhysicalDeviceWin32PresentationSupportKHR(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32) -> VkBool32;
}

pub struct Surface(VkSurfaceKHR, Rc<Instance>);
impl Surface
{
	pub fn new(instance: &Rc<Instance>, info: &VkWin32SurfaceCreateInfoKHR) -> EngineResult<Self>
	{
		let mut surf = empty_handle();
		unsafe { vkCreateWin32SurfaceKHR(***instance, info, std::ptr::null(), &mut surf) }
			.map(|| Surface(surf, instance.clone())).map_err(From::from)
	}
}
impl Drop for Surface { fn drop(&mut self) { unsafe { vkDestroySurfaceKHR(**self.1, self.0, std::ptr::null()) }; } }
impl Deref for Surface { type Target = VkSurfaceKHR; fn deref(&self) -> &Self::Target { &self.0 } }

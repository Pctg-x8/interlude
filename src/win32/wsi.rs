use vk::*;
use libc::*;
use winapi::*;
use std::rc::Rc;
use std::ops::Deref;
use {EngineResult, EngineError};

pub type VkWin32SurfaceCreateFlagsKHR = VkFlags;
pub struct VkWin32SurfaceCreateInfoKHR
{
	pub sType: VkStructureType, pNext: *const c_void, flags: VkWin32SurfaceCreateFlagsKHR,
	pub hinstance: HINSTANCE, pub hwnd: HWND
}

extern "system"
{
	fn vkCreateWin32SurfaceKHR(instance: VkInstance, pCreateInfo: *const VkWin32SurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurface) -> VkResult;
	fn vkGetPhysicalDeviceWin32PresentationSupportKHR(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32) -> VkBool32;
}

pub struct Surface(VkSurfaceKHR, Rc<Instance>);
impl Surface
{
	pub fn new(instance: &Rc<Instance>, info: &VkWin32SurfaceCreateInfoKHR) -> EngineResult<Self>
	{
		let mut surf = empty_handle();
		unsafe { vkCreateWin32SurfaceKHR(*&instance, info, std::ptr::null(), &mut surf) }
			.map(|| Surface(surf, instance.clone())).map_err(From::from)
	}
}
impl Drop for Surface { fn drop(&mut self) { unsafe { vkDestroySurfaceKHR(self.1, self.0, std::ptr::null()) }; } }
impl Deref for Surface { type Target = VkSurfaceKHR; fn deref(&self) -> &Self::Target { &self.0 } }

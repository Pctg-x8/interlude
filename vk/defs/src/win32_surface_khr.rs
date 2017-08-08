//! VK_KHR_win32_surface extensions

use super::*;
use winapi::*;
use libc::c_void;

pub const VK_KHR_WIN32_SURFACE_SPEC_VERSION: usize = 6;
pub static VK_KHR_WIN32_SURFACE_EXTENSION_NAME: &'static str = "VK_KHR_win32_surface";

pub type VkWin32SurfaceCreateFlagsKHR = VkFlags;

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkWin32SurfaceCreateInfoKHR
{
    pub sType: VkStructureType, pub pNext: *const c_void,
    pub flags: VkWin32SurfaceCreateFlagsKHR, pub hinstance: HINSTANCE, pub hwnd: HWND
}
impl Default for VkWin32SurfaceCreateInfoKHR
{
    fn default() -> Self
    {
        VkWin32SurfaceCreateInfoKHR { sType: VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR, .. unsafe { std::mem::zeroed() } }
    }
}

pub type PFN_vkCreateWin32SurfaceKHR = extern "system" fn(instance: VkInstance, pCreateInfo: *const VkWin32SurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
pub type PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR = extern "system" fn(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32) -> VkBool32;

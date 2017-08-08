//! VK_KHR_android_surface extensions

use libc::*;
use super::*;
use android_ffi::*;

pub const VK_KHR_ANDROID_SURFACE_SPEC_VERSION: usize = 6;
pub static VK_KHR_ANDROID_SURFACE_EXTENSION_NAME: &'static str = "VK_KHR_android_surface";

pub type VkAndroidSurfaceCreateFlagsKHR = VkFlags;

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkAndroidSurfaceCreateInfoKHR
{
    pub sType: VkStructureType, pub pNext: *const c_void, pub flags: VkAndroidSurfaceCreateFlagsKHR,
    pub window: *mut ANativeWindow
}
impl Default for VkAndroidSurfaceCreateInfoKHR
{
    fn default() -> Self
    {
        VkAndroidSurfaceCreateInfoKHR { sType: VK_STRUCTURE_TYPE_ANDROID_SURFACE_CREATE_INFO_KHR, .. unsafe { std::mem::zeroed() } }
    }
}

pub type PFN_vkCreateAndroidSurfaceKHR = extern "system" fn(instance: VkInstance, pCreateInfo: *const VkAndroidSurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;

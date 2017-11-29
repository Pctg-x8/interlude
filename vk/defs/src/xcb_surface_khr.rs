//! VK_KHR_xcb_surface extensions

use libc::*;
use super::*;
use xcb::*;

pub const VK_KHR_XCB_SURFACE_SPEC_VERSION: usize = 6;
pub static VK_KHR_XCB_SURFACE_EXTENSION_NAME: &'static str = "VK_KHR_xcb_surface";

pub type VkXcbSurfaceCreateFlagsKHR = VkFlags;

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkXcbSurfaceCreateInfoKHR
{
    pub sType: VkStructureType, pub pNext: *const c_void,
    pub flags: VkXcbSurfaceCreateFlagsKHR, pub connection: *mut xcb_connection_t, pub window: *mut xcb_window_t
}
impl Default for VkXcbSurfaceCreateInfoKHR
{
    fn default() -> Self
    {
        VkXcbSurfaceCreateInfoKHR { sType: VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR, .. unsafe { std::mem::zeroed() } }
    }
}

pub type PFN_vkCreateXcbSurfaceKHR = extern "system" fn(instance: VkInstance, pCreateInfo: *const VkXcbSurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
pub type PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR = extern "system" fn(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, connection: *mut xcb_connection_t, visual_id: xcb_visualid_t) -> VkBool32;

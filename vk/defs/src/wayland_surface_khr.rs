//! VK_KHR_wayland_surface extensions

use libc::*;
use super::*;
use wayland_client::sys::*;

pub const VK_KHR_WAYLAND_SURFACE_SPEC_VERSION: usize = 6;
pub static VK_KHR_WAYLAND_SURFACE_EXTENSION_NAME: &'static str = "VK_KHR_wayland_surface";

pub type VkWaylandSurfaceCreateFlagsKHR = VkFlags;

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkWaylandSurfaceCreateInfoKHR
{
    pub sType: VkStructureType, pub pNext: *const c_void, pub flags: VkWaylandSurfaceCreateFlagsKHR,
    pub display: *mut wl_display, pub surface: *mut wl_proxy/*wl_surface*/
}
impl Default for VkWaylandSurfaceCreateInfoKHR
{
    fn default() -> Self
    {
        VkWaylandSurfaceCreateInfoKHR { sType: VK_STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR, .. unsafe { std::mem::zeroed() } }
    }
}

pub type PFN_vkCreateWaylandSurfaceKHR = extern "system" fn(instance: VkInstance, pCreateInfo: *const VkWaylandSurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
pub type PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR = extern "system" fn(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, display: *mut wl_display) -> VkBool32;

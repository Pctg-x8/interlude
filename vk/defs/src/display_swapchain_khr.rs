//! VK_KHR_display_swapchain extensions

use libc::*;
use super::*;

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDisplayPresentInfoKHR
{
    pub sType: VkStructureType, pub pNext: *const c_void,
    pub srcRect: VkRect2D, pub dstRect: VkRect2D, pub persistent: VkBool32
}
impl Default for VkDisplayPresentInfoKHR
{
    fn default() -> Self
    {
        VkDisplayPresentInfoKHR { sType: VK_STRUCTURE_TYPE_DISPLAY_PRESENT_INFO_KHR, .. unsafe { std::mem::zeroed() } }
    }
}

pub type PFN_vkCreateSharedSwapchainsKHR = extern "system" fn(device: VkDevice, swapchainCount: u32, pCreateInfos: *const VkSwapchainCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSwapchains: *mut VkSwapchainKHR) -> VkResult;

//! VK_KHR_swapchain extension

use libc::*;
use super::*;

mod nd_handle_base_ts { pub enum VkSwapchainKHR {} }
pub type VkSwapchainKHR = VK_NON_DISPATCHABLE_HANDLE!(VkSwapchainKHR);

pub const VK_KHR_SWAPCHAIN_SPEC_VERSION: usize = 68;
pub static VK_KHR_SWAPCHAIN_EXTENSION_NAME: &'static str = "VK_KHR_swapchain";

pub type VkSwapchainCreateFlagsKHR = VkFlags;
pub const VK_SWAPCHAIN_CREATE_BIND_SFR_BIT_KHX: VkSwapchainCreateFlagsKHR = 0x01;

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkSwapchainCreateInfoKHR
{
    pub sType: VkStructureType, pub pNext: *const c_void, pub flags: VkSwapchainCreateFlagsKHR,
    pub surface: VkSurfaceKHR, pub minImageCount: u32, pub imageFormat: VkFormat, pub imageColorSpace: VkColorSpaceKHR,
    pub imageExtent: VkExtent2D, pub imageArrayLayers: u32, pub imageUsage: VkImageUsageFlags, pub imageSharingMode: VkSharingMode,
    pub queueFamilyIndexCount: u32, pub pQueueFamilyIndices: *const u32, pub preTransform: VkSurfaceTransformFlagsKHR,
    pub compositeAlpha: VkCompositeAlphaFlagsKHR, pub presentMode: VkPresentModeKHR, pub clipped: VkBool32, pub oldSwapchain: VkSwapchainKHR
}
impl Default for VkSwapchainCreateInfoKHR
{
    fn default() -> Self
    {
        VkSwapchainCreateInfoKHR { sType: VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR, .. unsafe { std::mem::zeroed() } }
    }
}

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPresentInfoKHR
{
    pub sType: VkStructureType, pub pNext: *const c_void,
    pub waitSemaphoreCount: u32, pub pWaitSemaphores: *const VkSemaphore,
    pub swapchainCount: u32, pub pSwapchains: *const VkSwapchainKHR,
    pub pImageIndices: *const u32, pub pResults: *mut VkResult
}
impl Default for VkPresentInfoKHR
{
    fn default() -> Self
    {
        VkPresentInfoKHR { sType: VK_STRUCTURE_TYPE_PRESENT_INFO_KHR, .. unsafe { std::mem::zeroed() } }
    }
}

pub type PFN_vkCreateSwapchainKHR = extern "system" fn(device: VkDevice, pCreateInfo: *const VkSwapchainCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSwapchain: *mut VkSwapchainKHR) -> VkResult;
pub type PFN_vkDestroySwapchainKHR = extern "system" fn(device: VkDevice, swapchain: VkSwapchainKHR, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkGetSwapchainImagesKHR = extern "system" fn(device: VkDevice, swapchain: VkSwapchainKHR, pSwapchainImageCount: *mut u32, pSwapchainImages: *mut VkImage) -> VkResult;
pub type PFN_vkAcquireNextImageKHR = extern "system" fn(device: VkDevice, swapchain: VkSwapchainKHR, timeout: u64, semaphore: VkSemaphore, fence: VkFence, pImageIndex: *mut u32) -> VkResult;
pub type PFN_vkQueuePresentKHR = extern "system" fn(queue: VkQueue, pPresentInfo: *const VkPresentInfoKHR) -> VkResult;

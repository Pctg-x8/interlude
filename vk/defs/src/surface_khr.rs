//! VK_KHR_surface extension

use super::*;

#[cfg(target_pointer_width = "64")]
mod nd_handle_base_ts { pub enum VkSurfaceKHR {} }
pub type VkSurfaceKHR = VK_NON_DISPATCHABLE_HANDLE!(VkSurfaceKHR);

pub const VK_KHR_SURFACE_SPEC_VERSION: usize = 25;
pub static VK_KHR_SURFACE_EXTENSION_NAME: &'static str = "VK_KHR_surface";
pub const VK_COLORSPACE_SRGB_NONLINEAR_KHR: VkColorSpaceKHR = VK_COLOR_SPACE_SRGB_NONLINEAR_KHR;

pub type VkColorSpaceKHR = isize;
pub const VK_COLOR_SPACE_SRGB_NONLINEAR_KHR: VkColorSpaceKHR = 0;
pub const VK_COLOR_SPACE_DISPLAY_P3_NONLINEAR_EXT: VkColorSpaceKHR = 100_0104_001;
pub const VK_COLOR_SPACE_EXTENDED_SRGB_LINEAR_EXT: VkColorSpaceKHR = 100_0104_002;
pub const VK_COLOR_SPACE_DCI_P3_LINEAR_EXT: VkColorSpaceKHR = 100_0104_003;
pub const VK_COLOR_SPACE_DCI_P3_NONLINEAR_EXT: VkColorSpaceKHR = 100_0104_004;
pub const VK_COLOR_SPACE_BT709_LINEAR_EXT: VkColorSpaceKHR = 100_0104_005;
pub const VK_COLOR_SPACE_BT709_NONLINEAR_EXT: VkColorSpaceKHR = 100_0104_006;
pub const VK_COLOR_SPACE_BT2020_LINEAR_EXT: VkColorSpaceKHR = 100_0104_007;
pub const VK_COLOR_SPACE_HDR10_ST2084_EXT: VkColorSpaceKHR = 100_0104_008;
pub const VK_COLOR_SPACE_DOLBYVISION_EXT: VkColorSpaceKHR = 100_0104_009;
pub const VK_COLOR_SPACE_KDR10_HLG_EXT: VkColorSpaceKHR = 100_0104_010;
pub const VK_COLOR_SPACE_ADOBERGB_LINEAR_EXT: VkColorSpaceKHR = 100_0104_011;
pub const VK_COLOR_SPACE_ADOBERGB_NONLINEAR_EXT: VkColorSpaceKHR = 100_0104_012;
pub const VK_COLOR_SPACE_PASS_THROUGH_EXT: VkColorSpaceKHR = 100_0104_013;

pub type VkPresentModeKHR = isize;
pub const VK_PRESENT_MODE_IMMEDIATE_KHR: VkPresentModeKHR = 0;
pub const VK_PRESENT_MODE_MAILBOX_KHR: VkPresentModeKHR = 1;
pub const VK_PRESENT_MODE_FIFO_KHR: VkPresentModeKHR = 2;
pub const VK_PRESENT_MODE_FIFO_RELAXED_KHR: VkPresentModeKHR = 3;
pub const VK_PRESENT_MODE_SHARED_DEMAND_REFRESH_KHR: VkPresentModeKHR = 100_0111_000;
pub const VK_PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR: VkPresentModeKHR = 100_0111_001;

pub type VkSurfaceTransformFlagsKHR = VkFlags;
pub const VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR: VkSurfaceTransformFlagsKHR = 0x0001;
pub const VK_SURFACE_TRANSFORM_ROTATE_90_BIT_KHR: VkSurfaceTransformFlagsKHR = 0x0002;
pub const VK_SURFACE_TRANSFORM_ROTATE_180_BIT_KHR: VkSurfaceTransformFlagsKHR = 0x0004;
pub const VK_SURFACE_TRANSFORM_ROTATE_270_BIT_KHR: VkSurfaceTransformFlagsKHR = 0x0008;
pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_BIT_KHR: VkSurfaceTransformFlagsKHR = 0x0010;
pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR: VkSurfaceTransformFlagsKHR = 0x0020;
pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR: VkSurfaceTransformFlagsKHR = 0x0040;
pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR: VkSurfaceTransformFlagsKHR = 0x0080;
pub const VK_SURFACE_TRANSFORM_INHERIT_BIT_KHR: VkSurfaceTransformFlagsKHR = 0x0100;

pub type VkCompositeAlphaFlagsKHR = VkFlags;
pub const VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR: VkCompositeAlphaFlagsKHR = 0x01;
pub const VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR: VkCompositeAlphaFlagsKHR = 0x02;
pub const VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR: VkCompositeAlphaFlagsKHR = 0x04;
pub const VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR: VkCompositeAlphaFlagsKHR = 0x08;

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkSurfaceCapabilitiesKHR
{
    pub minImageCount: u32, pub maxImageCount: u32,
    pub currentExtent: VkExtent2D, pub minImageExtent: VkExtent2D, pub maxImageExtent: VkExtent2D,
    pub maxImageArrayLayers: u32, pub supportedTransforms: VkSurfaceTransformFlagsKHR,
    pub currentTransform: VkSurfaceTransformFlagsKHR, pub supportedCompositeAlpha: VkCompositeAlphaFlagsKHR,
    pub supportedUsageFlags: VkImageUsageFlags
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkSurfaceFormatKHR { pub format: VkFormat, pub colorSpace: VkColorSpaceKHR }
impl Default for VkSurfaceFormatKHR
{
    fn default() -> Self
    {
        VkSurfaceFormatKHR { format: VK_FORMAT_UNDEFINED, colorSpace: VK_COLOR_SPACE_SRGB_NONLINEAR_KHR }
    }
}

pub type PFN_vkDestroySurfaceKHR = extern "system" fn(instance: VkInstance, surface: VkSurfaceKHR, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkGetPhysicalDeviceSurfaceSupportKHR = extern "system" fn(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, surface: VkSurfaceKHR, pSupported: *mut VkBool32) -> VkResult;
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR = extern "system" fn(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pSurfaceCapabilities: *mut VkSurfaceCapabilitiesKHR) -> VkResult;
pub type PFN_vkGetPhysicalDeviceSurfaceFormatsKHR = extern "system" fn(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pSurfaceFormatCount: *mut u32, pSurfaceFormats: *mut VkSurfaceFormatKHR) -> VkResult;
pub type PFN_vkGetPhysicalDeviceSurfacePresentModesKHR = extern "system" fn(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pPresentModeCount: *mut u32, pPresentModes: *mut VkPresentModeKHR) -> VkResult;

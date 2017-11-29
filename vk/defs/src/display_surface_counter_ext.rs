//! VK_EXT_display_surface_counter extensions

pub const VK_EXT_DISPLAY_SURFACE_COUNTER_SPEC_VERSION: usize = 1;
pub static VK_EXT_DISPLAY_SURFACE_COUNTER_EXTENSION_NAME: &'static str = "VK_EXT_display_surface_counter";

use libc::*;
use super::*;

pub type VkSurfaceCounterFlagsEXT = VkFlags;
pub const VK_SURFACE_COUNTER_VBLANK_EXT: VkSurfaceCounterFlagsEXT = 0x01;

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkSurfaceCapabilities2EXT
{
	pub sType: VkStructureType, pub pNext: *mut c_void,
	pub minImageCount: u32, pub maxImageCount: u32,
	pub currentExtent: VkExtent2D, pub minImageExtent: VkExtent2D,
	pub maxImageExtent: VkExtent2D, pub maxImageArrayLayers: u32,
	pub supportedTransforms: VkSurfaceTransformFlagsKHR,
	pub currentTransform: VkSurfaceTransformFlagsKHR,
	pub supportedCompositeAlpha: VkCompositeAlphaFlagsKHR,
	pub supportedUsageFlags: VkImageUsageFlags,
	pub supportedSurfaceCounters: VkSurfaceCounterFlagsEXT
}
impl Default for VkSurfaceCapabilities2EXT
{
	fn default() -> Self
	{
		VkSurfaceCapabilities2EXT
		{
			sType: VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES2_EXT,
			.. unsafe { std::mem::zeroed() }
		}
	}
}

pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT = extern "system" fn(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pSurfaceCapabilities: *mut VkSurfaceCapabilities2EXT) -> VkResult;

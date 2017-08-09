//! VK_EXT_direct_mode_display extensions

pub const VK_EXT_DIRECT_MODE_DISPLAY_SPEC_VERSION: usize = 1;
pub static VK_EXT_DIRECT_MODE_DISPLAY_EXTENSION_NAME: &'static str = "VK_EXT_direct_mode_display";

use super::*;

pub type PFN_vkReleaseDisplayEXT = extern "system" fn(physicalDevice: VkPhysicalDevice, display: VkDisplayKHR) -> VkResult;

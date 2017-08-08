//! VK_KHR_maintenance1 extensions

pub const VK_KHR_MAINTENANCE1_SPEC_VERSION: usize = 1;
pub static VK_KHR_MAINTENANCE1_EXTENSION_NAME: &'static str = "VK_KHR_maintenance1";

use super::*;

pub type VkCommandPoolTrimFlagsKHR = VkFlags;
pub type PFN_vkTrimCommandPoolKHR = extern "system" fn(device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolTrimFlagsKHR);

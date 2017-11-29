//! VK_AMD_draw_indirect_count extensions

pub const VK_AMD_DRAW_INDIRECT_COUNT_SPEC_VERSION: usize = 1;
pub static VK_AMD_DRAW_INDIRECT_COUNT_EXTENSION_NAME: &'static str = "VK_AMD_draw_indirect_count";

use super::*;

pub type PFN_vkCmdDrawIndirectCountAMD = extern "system" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, countBuffer: VkBuffer, countBufferOffset: VkDeviceSize, maxDrawCount: u32, stride: u32);
pub type PFN_vkCmdDrawIndexedIndirectCountAMD = extern "system" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, countBuffer: VkBuffer, countBufferOffset: VkDeviceSize, maxDrawCount: u32, stride: u32);

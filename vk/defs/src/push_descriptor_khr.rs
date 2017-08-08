//! VK_KHR_push_descriptor extensions

pub const VK_KHR_PUSH_DESCRIPTOR_SPEC_VERSION: usize = 1;
pub static VK_KHR_PUSH_DESCRIPTOR_EXTENSION_NAME: &'static str = "VK_KHR_push_descriptor";

use libc::*;
use super::*;

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPhysicalDevicePushDescriptorPropertiesKHR
{
    pub sType: VkStructureType, pub pNext: *mut c_void, pub maxPushDescriptors: u32
}
impl Default for VkPhysicalDevicePushDescriptorPropertiesKHR
{
    fn default() -> Self
    {
        VkPhysicalDevicePushDescriptorPropertiesKHR
        {
            sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR,
            .. unsafe { std::mem::zeroed() }
        }
    }
}

pub type PFN_vkCmdPushDescriptorSetKHR = extern "system" fn(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, layout: VkPipelineLayout, set: u32, descriptorWriteCount: u32, pDescriptorWrites: *const VkWriteDescriptorSet);

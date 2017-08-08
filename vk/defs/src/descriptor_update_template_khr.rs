//! VK_KHR_descriptor_update_template extensions

pub const VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE_SPEC_VERSION: usize = 1;
pub static VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE_EXTENSION_NAME: &'static str = "VK_KHR_descriptor_update_template";

use super::*;
use libc::*;

mod nd_handle_base_ts { pub enum VkDescriptorUpdateTemplateKHR {} }
pub type VkDescriptorUpdateTemplateKHR = VK_NON_DISPATCHABLE_HANDLE!(VkDescriptorUpdateTemplateKHR);

pub type VkDescriptorUpdateTemplateTypeKHR = isize;
pub const VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET_KHR: VkDescriptorUpdateTemplateTypeKHR = 0;
pub const VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR: VkDescriptorUpdateTemplateTypeKHR = 1;

pub type VkDescriptorUpdateTemplateCreateFlagsKHR = VkFlags;

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDescriptorUpdateTemplateEntryKHR
{
    pub dstBinding: u32, pub dstArrayElement: u32, pub descriptorCount: u32,
    pub descriptorType: VkDescriptorType, pub offset: size_t, pub stride: size_t
}
impl Default for VkDescriptorUpdateTemplateEntryKHR
{
    fn default() -> Self { unsafe { std::mem::zeroed() } }
}

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDescriptorUpdateTemplateCreateInfoKHR
{
    pub sType: VkStructureType, pub pNext: *mut c_void,
    pub flags: VkDescriptorUpdateTemplateCreateFlagsKHR,
    pub descriptorUpdateEntryCount: u32,
    pub pDescrptorUpdateEntries: *const VkDescriptorUpdateTemplateEntryKHR,
    pub templateType: VkDescriptorUpdateTemplateTypeKHR,
    pub descriptorSetLayout: VkDescriptorSetLayout, pub pipelineBindPoint: VkPipelineBindPoint,
    pub pipelineLayout: VkPipelineLayout, pub set: u32
}
impl Default for VkDescriptorUpdateTEmplateCreateInfoKHR
{
    fn default() -> Self
    {
        VkDescriptorUpdateTemplateCreateInfoKHR
        {
            sType: VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO_KHR,
            .. unsafe { std::mem::zeroed() }
        }
    }
}

pub type PFN_vkCreateDescriptorUpdateTemplateKHR = extern "system" fn(device: VkDevice, pCreateInfo: *const VkDescriptorUpdateTemplateCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pDescriptorUpdateTemplate: *mut VkDescriptorUpdateTemplateKHR) -> VkResult;
pub type PFN_vkDestroyDescriptorUpdateTemplateKHR = extern "system" fn(device: VkDevice, descriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkUpdateDescriptorSetWithTemplateKHR = extern "system" fn(device: VkDevice, descriptorSet: VkDescriptorSet, descriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR, pData: *const c_void);
pub type PFN_vkCmdPushDescriptorSetWithTemplateKHR = extern "system" fn(commandBuffer: VkCommandBuffer, descriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR, layout: VkPipelineLayout, set: u32, pData: *const c_void);

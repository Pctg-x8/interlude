//! VK_KHR_external_memory extensions

pub const VK_KHR_EXTERNAL_MEMORY_SPEC_VERSION: usize = 1;
pub static VK_KHR_EXTERNAL_MEMORY_EXTENSION_NAME: &'static str = "VK_KHR_external_memory";
pub const VK_QUEUE_FAMILY_EXTERNAL_KHR: u32 = 0xfffffffe;

use libc::*;
use super::*;

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkExternalMemoryImageCreateInfoKHR
{
    pub sType: VkStructureType, pub pNext: *const c_void,
    pub handleTypes: VkExternalMemoryHandleTypeFlagsKHR
}
impl Default for VkExternalMemoryImageCreateInfoKHR
{
    fn default() -> Self
    {
        VkExternalMemoryImageCreateInfoKHR
        {
            sType: VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO_KHR,
            .. unsafe { std::mem::zeroed() }
        }
    }
}

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkExternalMemoryBufferCreateInfoKHR
{
    pub sType: VkStructureType, pub pNext: *const c_void,
    pub handleTypes: VkExternalMemoryHandleTypeFlagsKHR
}
impl Default for VkExternalMemoryBufferCreateInfoKHR
{
    fn default() -> Self
    {
        VkExternalMemoryBufferCreateInfoKHR
        {
            sType: VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_BUFFER_CREATE_INFO_KHR,
            .. unsafe { std::mem::zeroed() }
        }
    }
}

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkExportMemoryAllocateInfoKHR
{
    pub sType: VkStructureType, pub pNext: *const c_void,
    pub handleTypes: VkExternalMEmoryHandleTypeFlagsKHR
}
impl Default for VkExportMemoryAllocateInfoKHR
{
    fn default() -> Self
    {
        VkExportMemoryAllocateInfoKHR
        {
            sType: VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO_KHR,
            .. unsafe { std::mem::zeroed() }
        }
    }
}

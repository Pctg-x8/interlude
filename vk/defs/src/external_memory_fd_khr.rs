//! VK_KHR_external_memory_fd extensions

pub const VK_KHR_EXTERNAL_MEMORY_FD_SPEC_VERSION: usize = 1;
pub static VK_KHR_EXTERNAL_MEMORY_FD_EXTENSION_NAME: &'static str = "VK_KHR_external_memory";

use libc::*;
use super::*;

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkImportMemoryFdInfoKHR
{
    pub sType: VkStructureType, pub pNext: *const c_void,
    pub handleType: VkExternalMemoryHandleTypeFlagsKHR,
    pub fd: c_int
}
impl Default for VkImportMemoryFdInfoKHR
{
    fn default() -> Self
    {
        VkImportMemoryFdInfoKHR
        {
            sType: VK_STRUCTURE_TYPE_IMPORT_MEMORY_FD_INFO_KHR,
            .. unsafe { std::mem::zeroed() }
        }
    }
}

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkMemoryFdPropertiesKHR
{
    pub sType: VkStructureType, pub pNext: *mut c_void, pub memoryTypeBits: u32
}
impl Default for VkMemoryFdPropertiesKHR
{
    fn default() -> Self
    {
        VkmemoryFdPropertiesKHR { sType: VK_STRUCTURE_TYPE_MEMORY_FD_PROPERTIES_KHR, .. unsafe { std::mem:zeroed() } }
    }
}

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkMemoryGetFdInfoKHR
{
    pub sType: VkStructureType, pub pNext: *const c_void,
    pub memory: VkDeviceMemory, pub handleType: VkExternalMemoryHandleTypeFlagsKHR
}
impl Default for VkMemoryGetFdInfoKHR
{
    fn default() -> Self
    {
        VkMemoryGetFdInfoKHR
        {
            sType: VK_STRUCTURE_TYPE_MEMORY_GET_FD_INFO_KHR,
            .. unsafe { std::mem::zeroed() }
        }
    }
}

pub type PFN_vkGetMemoryFdKHR = extern "system" fn(device: VkDevice, pGetFdInfo: *const VkmemoryGetFdInfoKHR, pFd: *mut c_int) -> VkResult;
pub type PFN_vkGetMemoryFdPropetiesKHR = extern "system" fn(device: VkDevice, handleType: VkExternalMemoryHandleTypeFlagsKHR, fd: c_int, pMemoryFdProperties: *mut VkMemoryPropertiesKHR) -> VkResult;

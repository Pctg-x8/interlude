//! VK_KHR_external_memory_win32 extensions

pub const VK_KHR_EXTERNAL_MEMORY_WIN32_SPEC_VERSION: usize = 1;
pub static VK_KHR_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME: &'static str = "VK_KHR_external_memory_win32";

use winapi::*;
use super::*;

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkImportMemoryWin32HandleInfoKHR
{
    pub sType: VkStructureType, pub pNext: *const c_void,
    pub handleType: VkExternalMemoryHandleTypeFlagsKHR,
    pub handle: HANDLE, pub name: LPCWSTR
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkExportMemoryWin32HandleInfoKHR
{
    pub sType: VkStructureType, pub pNext: *const c_void,
    pub pAttributes: *const SECURITY_ATTRIBUTES, pub dwAccess: DWORD,
    pub name: LPCWSTR
}
impl Default for VkImportMemoryWin32HandleInfoKHR
{
    fn default() -> VkImportMemoryWin32HandleInfoKHR
    {
        VkImportMemoryWin32HandleInfoKHR
        {
            sType: VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR,
            .. unsafe { std::mem::zeroed() }
        }
    }
}
impl Default for VkExportMemoryWin32HandleInfoKHR
{
    fn default() -> VkExportMemoryWin32HandleInfoKHR
    {
        VkExportMemoryWin32HandleInfoKHR
        {
            sType: VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR,
            .. unsafe { std::mem::zeroed() }
        }
    }
}

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkMemoryWin32HandlePropertiesKHR
{
    pub sType: VkStructureType, pub pNext: *mut c_void, pub memoryTypeBits: u32
}
impl Default for VkMemoryWin32HandlePropertiesKHR
{
    fn default() -> Self
    {
        VkMemoryWin32HandlePropertiesKHR
        {
            sType: VK_STRUCTURE_TYPE_MEMORY_WIN32_HANDLE_PROPERTIES_KHR,
            .. unsafe { std::mem::zeroed() }
        }
    }
}

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkMemoryGetWin32HandleInfoKHR
{
    pub sType: VkStructureType, pub pNext: *const c_void,
    pub memory: VkDeviceMemory, pub handleType: VkExternalMemoryHandleTypeFlagsKHR
}
impl Default for VkMemoryGetWin32HandleInfoKHR
{
    fn default() -> Self
    {
        VkMemoryGetWin32HandleInfoKHR
        {
            sType: VK_STRUCTURE_TYPE_MEMORY_GET_WIN32_HANDLE_INFO_KHR,
            .. unsafe { std::mem::zeroed() }
        }
    }
}

pub type PFN_vkGetMemoryWin32HandleKHR = extern "system" fn(device: VkDevice, pGetWin32HandleInfo: *const VkMemoryGetWin32HandleInfoKHR, pHandle: *mut HANDLE) -> VkResult;
pub type PFN_vkGetMemoryWin32HandlePropertiesKHR = extern "system" fn(device: VkDevice, handleType: VkExternalMemoryHandleTypeFlagsKHR, handle: HANDLE, pMemoryWin32HandleProperties: *mut VkMemoryWin32HandlePropertiesKHR) -> VkResult;

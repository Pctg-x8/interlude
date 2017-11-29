//! VK_KHR_external_semaphore_win32 extensions

pub const VK_KHR_EXTERNAL_SEMAPHORE_WIN32_SPEC_VERSION: usize = 1;
pub static VK_KHR_EXTERNAL_SEMAPHORE_WIN32_EXTENSION_NAME: &'static str = "VK_KHR_external_semaphore_win32";

use winapi::*;
use super::*;

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkImportSemaphoreWin32HandleInfoKHR
{
    pub sType: VkStructureType, pub pNext: *const c_void,
    pub semaphore: VkSemaphore, pub flags: VkSemaphoreImportFlagsKHR,
    pub handleType: VkExternalSemaphoreHandleTypeFlagBitsKHR,
    pub handle: HANDLE, pub name: LPCWSTR
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkExportSemaphoreWin32HandleInfoKHR
{
    pub sType: VkStructureType, pub pNext: *const c_void,
    pub pAttributes: *const SECURITY_ATTRIBUTES, pub dwAccess: DWORD, pub name: LPCWSTR
}
impl Default for VkImportSemaphoreWin32HandleInfoKHR
{
    fn default() -> Self
    {
        VkImportSemaphoreWin32HandleInfoKHR
        {
            sType: VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR,
            .. unsafe { std::mem::zeroed() }
        }
    }
}
impl Default for VkExportSemaphoreWin32HandleInfoKHR
{
    fn default() -> Self
    {
        VkExportSemaphoreWin32HandleInfoKHR
        {
            sType: VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR,
            .. unsafe { std::mem::zeroed() }
        }
    }
}

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkD3D12FenceSubmitInfoKHR
{
    pub sType: VkStructureType, pub pNext: *const c_void,
    pub waitSemaphoreValuesCount: u32, pub pWaitSemaphoreValues: *const u64,
    pub signalSemaphoreValuesCount: u32, pub pSignalSemaphoreValues: *const u64
}
impl Default for VkD3D12FenceSubmitInfoKHR
{
    fn default() -> Self
    {
        VkD3D12FenceSubmitInfoKHR
        {
            sType: VK_STRUCTURE_TYPE_D3D12_FENCE_SUBMIT_INFO_KHR,
            .. unsafe { std::mem::zeroed() }
        }
    }
}

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkSemaphoreGetWin32HandleInfoKHR
{
    pub sType: VkStructureType, pub pNext: *const c_void,
    pub semaphore: VkSemaphore, pub handleType: VkExternalSemaphoreHandleTypeFlagsKHR
}
impl Default for VkSemaphoreGetWin32HandleInfoKHR
{
    fn default() -> Self
    {
        VkSemaphoreGetWin32HandleInfoKHR
        {
            sType: VK_STRUCTURE_TYPE_SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR,
            .. unsafe { std::mem::zeroed() }
        }
    }
}

pub type PFN_vkImportSemaphoreWin32HandleKHR = extern "system" fn(device: VkDevice, pImportSemaphoreWin32HandleInfo: *const VkImportSemaphoreWin32HandleInfoKHR) -> VkResult;
pub type PFN_vkGetSemaphoreWin32HandleKHR = extern "system" fn(device: VkDevice, pGetWin32HandleInfo: *const VkSemaphoreGetWin32HandleInfoKHR, pHandle: *mut HANDLE) -> VkResult;

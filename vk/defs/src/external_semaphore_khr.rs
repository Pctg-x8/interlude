//! VK_KHR_external_semaphore extensions

pub const VK_KHR_EXTERNAL_SEMAPHORE_SPEC_VERSION: usize = 1;
pub static VK_KHR_EXTERNAL_SEMAPHORE_EXTENSION_NAME: &'static str = "VK_KHR_external_semaphore";

use libc::*;
use super::*;

pub type VkSemaphoreImportFlagsKHR = VkFlags;
pub const VK_SEMAPHORE_IMPORT_TEMPORARY_BIT_KHR: VkSemaphoreImportFlagsKHR = 0x01;

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkExportSemaphoreCreateInfoKHR
{
    pub sType: VkStructureType, pub pNext: *const c_void,
    pub handleTypes: VkExternalSemaphoreHandleTypeFlagsKHR
}
impl Default for VkExportSemaphoreCreateInfoKHR
{
    fn default() -> Self
    {
        VkExportSemaphoreCreateInfoKHR
        {
            sType: VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_CREATE_INFO_KHR,
            .. unsafe { std::mem::zeroed() }
        }
    }
}

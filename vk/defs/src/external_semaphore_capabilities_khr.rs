//! VK_KHR_external_semaphore_capabilities extensions

pub const VK_KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_SPEC_VERSION: usize = 1;
pub static VK_KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_EXTENSION_NAME: &'static str = "VK_KHR_external_semaphore_capabilities";

use libc::*;
use super::*;

pub type VkExternalSemaphoreHandleTypeFlagsKHR = VkFlags;
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR: VkExternalSemaphoreHandleTypeFlagsKHR = 0x01;
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR: VkExternalSemaphoreHandleTypeFlagsKHR = 0x02;
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR: VkExternalSemaphoreHandleTypeFlagsKHR = 0x04;
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT_KHR: VkExternalSemaphoreHandleTypeFlagsKHR = 0x08;
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT_KHR: VkExternalSemaphoreHandleTypeFlagsKHR = 0x10;

pub type VkExternalSemaphoreFeatureFlagsKHR = VkFlags;
pub const VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT_KHR: VkExternalSemaphoreFeatureFlagsKHR = 0x01;
pub const VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT_KHR: VkExternalSemaphoreFeatureFlagsKHR = 0x02;

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPhysicalDeviceExternalSemaphoreInfoKHR
{
    pub sType: VkStructureType, pub pNext: *const c_void, pub handleType: VkExternalSemaphoreHandleTypeFlagsKHR
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkExternalSemaphorePropertiesKHR
{
    pub sType: VkStructureType, pub pNext: *mut c_void,
    pub exportFromImportedHandleType: VkExternalSemaphoreHandleTypeFlagsKHR,
    pub compatibleHandleTypes: VkExternalSemaphoreHandleTypeFlagsKHR,
    pub externalSemaphoreFeatures: VkExternalSemaphoreFeatureFlagsKHR
}
impl Default for VkPhysicalDeviceExternalSemaphoreInfoKHR
{
    fn default() -> Self
    {
        VkPhysicalDeviceExternalSemaphoreInfoKHR
        {
            sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO_KHR,
            .. unsafe { std::mem::zeroed() }
        }
    }
}
impl Default for VkExternalSemaphorePropertiesKHR
{
    fn default() -> Self
    {
        VkExternalSemaphorePropertiesKHR
        {
            sType: VK_STRUCTURE_TYPE_EXTERNAL_SEMAPHORE_PROPERTIES_KHR,
            .. unsafe { std::mem::zeroed() }
        }
    }
}

pub type PFN_vkGetPhysicalDeviceExternalSemaphorePropertiesKHR = extern "system" fn(physicalDevice: VkPhysicalDevice, pExternalSemaphoreInfoKHR: *const VkPhysicalDeviceExternalSemaphoreInfoKHR, pExternalSemaphoreProperties: *mut VkExternalSemaphorePropertiesKHR);

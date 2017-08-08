//! VK_KHR_16bit_storage extensions

pub const VK_KHR_16BIT_STORAGE_SPEC_VERSION: usize = 1;
pub static VK_KHR_16BIT_STORAGE_EXTENSION_NAME: &'static str = "VK_KHR_16bit_storage";

use libc::*;
use super::*;

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPhysicalDevice16BitStorageFeaturesKHR
{
    pub sType: VkStructureType, pub pNext: *mut c_void,
    pub storageBuffer16BitAccess: VkBool32,
    pub uniformAndStorageBuffer16BitAccess: VkBool32,
    pub storagePushConstant16: VkBool32,
    pub storageInputOutput16: VkBool32
}
impl Default for VkPhysicalDevice16BitStorageFeaturesKHR
{
    fn default() -> Self
    {
        VkPhysicalDevice16BitStorageFeaturesKHR
        {
            sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES_KHR,
            .. unsafe { std::mem::zeroed() }
        }
    }
}

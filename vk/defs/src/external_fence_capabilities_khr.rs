//! VK_KHR_external_fence_capabilities extensions

pub const VK_KHR_EXTERNAL_FENCE_CAPABILITIES_SPEC_VERSION: usize = 1;
pub static VK_KHR_EXTERNAL_FENCE_CAPABILITIES_EXTENSION_NAME: &'static str = "VK_KHR_external_fence_capabilities";

use libc::*;
use super::*;

pub type VkExternalFenceHandleTypeFlagsKHR = VkFlags;
pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR: VkExternalFenceHandleTypeFlagsKHR = 0x01;
pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR: VkExternalFenceHandleTypeFlagsKHR = 0x02;
pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR: VkExternalFenceHandleTypeFlagsKHR = 0x04;
pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT_KHR: VkExternalFenceHandleTypeFlagsKHR = 0x08;

pub type VkExternalFenceFeatureFlagsKHR = VkFlags;
pub const VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT_KHR: VkExternalFenceFeatureFlagsKHR = 0x01;
pub const VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT_KHR: VkExternalFenceFeatureFlagsKHR = 0x02;

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPhysicalDeviceExternalFenceInfoKHR
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub handleType: VkExternalFenceHandleTYpeFlagsKHR
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkExternalFencePropertiesKHR
{
	pub sType: VkStructureType, pub pNext: *mut c_void,
	pub exportFromImportedHandleTypes: VkExternalFenceTypeFlagsKHR,
	pub compatibleHandleTypes: VkExternalFenceHandleTypeFlagsKHR,
	pub externalFenceFeatures: VkExternalFenceFeatureFlagsKHR
}
impl Default for VkPhysicalDeviceExternalFenceInfoKHR
{
	fn default() -> Self
	{
		VkPhysicalDeviceExternalFenceInfoKHR
		{
			sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO_KHR,
			.. unsafe { std::mem::zeroed() }
		}
	}
}
impl Default for VkExternalFencePropertiesKHR
{
	fn default() -> Self
	{
		VkExternalFencePropertiesKHR
		{
			sType: VK_STRUCTURE_TYPE_EXTERNAL_FENCE_PROPERTIES_KHR,
			.. unsafe { std::mem::zeroed() }
		}
	}
}

pub type PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR = extern "system" fn(physicalDevice: VkPhysicalDevice, pExternalFenceInfo: *const VkPhysicalDeviceExternalFenceInfoKHR, pExternalFenceProperties: *mut VkExternalFencePropertiesKHR);

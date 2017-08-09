//! VK_NV_external_memory_capabilities extensions

pub const VK_NV_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION: usize = 1;
pub static VK_NV_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME: &'static str = "VK_NV_external_memory_capabilities";

use super::*;

pub type VkExternalMemoryHandleTypeFlagsNV = VkFlags;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_NV: VkExternalMemoryHandleTypeFlagsNV = 0x01;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_NV: VkExternalMemoryHandleTypeFlagsNV = 0x02;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_BIT_NV: VkExternalMemoryHandleTypeFlagsNV = 0x04;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_KMT_BIT_NV: VkExternalMemoryHandleTypeFlagsNV = 0x08;

pub type VkExternalMemoryFeatureFlagsNV = VkFlags;
pub const VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_NV: VkExternalMemoryFeatureFlagsNV = 0x01;
pub const VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_NV: VkExternalMemoryFeatureFlagsNV = 0x02;
pub const VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_NV: VkExternalMemoryFeatureFlagsNV = 0x04;

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkExternalImageFormatPropertiesNV
{
	pub imageFormatProperties: VkImageFormatProperties,
	pub externalMemoryFeatures: VkExternalMemoryFeatureFlagsNV,
	pub exportFromImportedHandleTypes: VkExternalMemoryHandleTypeFlagsNV,
	pub compatibleHandleTypes: VkExternalMemoryHandleTypeFlagsNV
}

pub type PFN_vkGetPhysicalDeviceExternalImageFormatPropertiesNV = extern "system" fn(physicalDevice: VkPhysicalDevice, format: VkFormat, itype: VkImageType, tiling: VkImageTiling, usage: VkImageUsageFlags, flags: VkImageCreateFlags, externalHandleType: VkExternalMemoryHandleTypeFlagsNV, pExternalImageFormatProperties: *mut VkExternalImageFormatPropertiesNV) -> VkResult;

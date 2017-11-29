//! VK_EXT_blend_operation_advanced extensions

pub const VK_EXT_BLEND_OPERATION_ADVANCED_SPEC_VERSION: usize = 2;
pub static VK_EXT_BLEND_OPERATION_ADVANCED_EXTENSION_NAME: &'static str = "VK_EXT_blend_operation_advanced";

use libc::*;
use super::*;

pub type VkBlendOverlapEXT = i32;
pub const VK_BLEND_OVERLAP_UNCORRELATED_EXT: VkBlendOverlapEXT = 0;
pub const VK_BLEND_OVERLAP_DISJOINT_EXT: VkBlendOverlapEXT = 1;
pub const VK_BLEND_OVERLAP_CONJOINT_EXT: VkBlendOverlapEXT = 2;

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT
{
	pub sType: VkStructureType, pub pNext: *mut c_void,
	pub advancedBlendCoherentOperations: VkBool32
}
impl Default for VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT
{
	fn default() -> Self
	{
		VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT
		{
			sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT,
			.. unsafe { std::mem::zeroed() }
		}
	}
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT
{
	pub sType: VkStructureType, pub pNext: *mut c_void,
	pub advancedBlendMaxColorAttachments: u32,
	pub advancedBlendIndependentBlend: VkBool32,
	pub advancedBlendNonPremultipliedSrcColor: VkBool32,
	pub advancedBlendNonPremultipliedDstColor: VkBool32,
	pub advancedBlendCorrelatedOverlap: VkBool32,
	pub advancedBlendAllOperations: VkBool32
}
impl Default for VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT
{
	fn default() -> Self
	{
		VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT
		{
			sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT,
			.. unsafe { std::mem::zeroed() }
		}
	}
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPipelineColorBlendAdvancedStateCreateInfoEXT
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub srcPremultiplied: VkBool32,
	pub dstPremultiplied: VkBool32,
	pub blendOverlap: VkBlendOverlapEXT
}
impl Default for VkPipelineColorBlendAdvancedStateCreateInfoEXT
{
	fn default() -> Self
	{
		VkPipelineColorBlendAdvancedStateCreateInfoEXT
		{
			sType: VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT,
			.. unsafe { std::mem::zeroed() }
		}
	}
}

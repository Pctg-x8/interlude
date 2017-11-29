//! VK_EXT_sampler_filter_minmax extensions

pub const VK_EXT_SAMPLER_FILTER_MINMAX_SPEC_VERSION: usize = 1;
pub static VK_EXT_SAMPLER_FILTER_MINMAX_EXTENSION_NAME: &'static str = "VK_EXT_sampler_filter_minmax";

use libc::*;
use super::*;

pub type VkSamplerReductionModeEXT = i32;
pub const VK_SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE_EXT: VkSamplerReductionModeEXT = 0;
pub const VK_SAMPLER_REDUCTION_MODE_MIN_EXT: VkSamplerReductionModeEXT = 1;
pub const VK_SAMPLER_REDUCTION_MODE_MAX_EXT: VkSamplerReductionModeEXT = 2;

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkSamplerReductionModeCreateInfoEXT
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub reductionMode: VkSamplerReductionModeEXT
}
impl Default for VkSamplerReductionModeCreateInfoEXT
{
	fn default() -> Self
	{
		VkSamplerReductionModeCreateInfoEXT
		{
			sType: VK_STRUCTURE_TYPE_SAMPLER_REDUCTION_MODE_CREATE_INFO_EXT,
			.. unsafe { std::mem::zeroed() }
		}
	}
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT
{
	pub sType: VkStructureType, pub pNext: *mut c_void,
	pub filterMinmaxSingleComponentFormats: VkBool32,
	pub filterMinmaxImageComponentMapping: VkBool32
}
impl Default for VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT
{
	fn default() -> Self
	{
		VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT
		{
			sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES_EXT,
			.. unsafe { std::mem::zeroed() }
		}
	}
}

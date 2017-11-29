//! VK_KHX_multiview extensions

pub const VK_KHX_MULTIVIEW_SPEC_VERSION: usize = 1;
pub static VK_KHX_MULTIVIEW_EXTENSION_NAME: &'static str = "VK_KHX_multiview";

use libc::*;
use super::*;

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkRenderPassMultiviewCreateInfoKHX
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub subpassCount: u32, pub pViewMasks: *const u32,
	pub dependencyCount: u32, pub pViewOffsets: *const i32,
	pub correlationMaskCount: u32, pub pCorrelationMasks: *const u32
}
impl Default for VkRenderPassMultiviewCreateInfoKHX
{
	fn default() -> Self
	{
		VkRenderPassMultiviewCreateInfoKHX
		{
			sType: VK_STRUCTURE_TYPE_RENDER_PASS_MULTIVIEW_CREATE_INFO_KHX,
			.. unsafe { std::mem::zeroed() }
		}
	}
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPhysicalDeviceMultiviewFeaturesKHX
{
	pub sType: VkStructureType, pub pNext: *mut c_void,
	pub multiview: VkBool32, pub multiviewGeometryShader: VkBool32,
	pub multiviewTessellationShader: VkBool32
}
impl Default for VkPhysicalDeviceMultiviewFeaturesKHX
{
	fn default() -> Self
	{
		VkPhysicalDeviceMultiviewFeaturesKHX
		{
			sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_FEATURES_KHX,
			.. unsafe { std::mem::zeroed() }
		}
	}
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPhysicalDeviceMultiviewPropertiesKHX
{
	pub sType: VkStructureType, pub pNext: *mut c_void,
	pub maxMultiviewViewCount: u32, pub maxMultiviewInstanceIndex: u32
}
impl Default for VkPhysicalDeviceMultiviewPropertiesKHX
{
	fn default() -> Self
	{
		VkPhysicalDeviceMultiviewPropertiesKHX
		{
			sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES_KHX,
			.. unsafe { std::mem::zeroed() }
		}
	}
}

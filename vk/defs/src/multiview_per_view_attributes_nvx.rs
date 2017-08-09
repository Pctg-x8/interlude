//! VK_NVX_multiview_per_view_attributes extensions

pub const VK_NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_SPEC_VERSION: usize = 1;
pub static VK_NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_EXTENSION_NAME: &'static str = "VK_NVX_multiview_per_view_attributes";

use libc::*;
use super::*;

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX
{
	pub sType: VkStructureType, pub pNext: *mut c_void,
	pub perViewPositionAllComponents: VkBool32
}
impl Default for VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX
{
	fn default() -> Self
	{
		VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX
		{
			sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX,
			.. unsafe { std::mem::zeroed() }
		}
	}
}

//! VK_KHR_variable_pointers extensions

pub const VK_KHR_VARIABLE_POINTERS_SPEC_VERSION: usize = 1;
pub static VK_KHR_VARIABLE_POINTERS_EXTENSION_NAME: &'static str = "VK_KHR_variable_pointers";

use libc::*;
use super::*;

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPhysicalDeviceVariablePointerFeaturesKHR
{
	pub sType: VkStructureType, pub pNext: *mut c_void,
	pub variablePointersStorageBuffer: VkBool32,
	pub variablePointers: VkBool32
}
impl Default for VkPhysicalDeviceVariablePointerFeaturesKHR
{
	fn default() -> Self
	{
		VkPhysicalDeviceVariablePointerFeaturesKHR
		{
			sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES_KHR,
			.. unsafe { std::mem::zeroed() }
		}
	}
}

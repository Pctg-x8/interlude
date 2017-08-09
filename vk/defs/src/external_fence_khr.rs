//! VK_KHR_external_fence extensions

pub const VK_KHR_EXTERNAL_FENCE_SPEC_VERSION: usize = 1;
pub static VK_KHR_EXTERNAL_FENCE_EXTENSION_NAME: &'static str = "VK_KHR_external_fence";

use libc::*;
use super::*;

pub type VkFenceImportFlagsKHR = VkFlags;
pub const VK_FENCE_IMPORT_TEMPORARY_BIT_KHR: VkFenceImportFlagsKHR = 0x01;

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkExportFenceCreateInfoKHR
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub handleTypes: VkExternalFenceHandleTypeFlagsKHR
}
impl Default for VkExportFenceCreateInfoKHR
{
	fn default() -> Self
	{
		VkExportFenceCreateInfoKHR
		{
			sType: VK_STRUCTURE_TYPE_EXPORT_FENCE_CREATE_INFO_KHR,
			.. unsafe { std::mem::zeroed() }
		}
	}
}

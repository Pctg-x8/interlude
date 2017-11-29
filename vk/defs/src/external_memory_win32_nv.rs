//! VK_NV_external_memory_win32 extensions

pub const VK_NV_EXTERNAL_MEMORY_WIN32_SPEC_VERSION: usize = 1;
pub static VK_NV_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME: &'static str = "VK_NV_external_memory_win32";

use winapi::*;
use super::*;

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkImportMemoryWin32HandleInfoNV
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub handleType: VkExternalMemoryHandleTypeFlagsNV,
	pub handle: HANDLE
}
impl Default for VkImportMemoryWin32HandleInfoNV
{
	fn default() -> Self
	{
		VkImportMemoryWin32HandleInfoNV
		{
			sType: VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_NV,
			.. unsafe { std::mem::zeroed() }
		}
	}
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkExportMemoryWin32HandleInfoNV
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub pAttributes: *const SECURITY_ATTRIBUTES,
	pub dwAccess: DWORD
}
impl Default for VkExportMemoryWin32HandleInfoNV
{
	fn default() -> Self
	{
		VkExportMemoryWin32HandleInfoNV
		{
			sType: VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_NV,
			.. unsafe { std::mem::zeroed() }
		}
	}
}

pub type PFN_vkGetMemoryWin32HandleNV = extern "system" fn(device: VkDevice, memory: VkDeviceMemory, handleType: VkExternalMemoryHandleTypeFlagsNV, pHandle: *mut HANDLE) -> VkResult;

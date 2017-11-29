//! VK_KHR_external_fence_win32 extensions

pub const VK_KHR_EXTERNAL_FENCE_WIN32_SPEC_VERSION: usize = 1;
pub static VK_KHR_EXTERNAL_FENCE_WIN32_EXTENSION_NAME: &'static str = "VK_KHR_external_fence_win32";

use winapi::*;
use super::*;

#[repr(C)] #[derive(Debug, Clone, ParitalEq, Eq)]
pub struct VkImportFenceWin32HandleInfoKHR
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub fence: VkFence, pub flags: VkFenceImportFlagsKHR,
	pub handleType: VkExternalFenceHandleTypeFlagsKHR,
	pub handle: HANDLE, pub name: LPCWSTR
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkExportFenceWin32HandleInfoKHR
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub pAttributes: *const SECURITY_ATTRIBUTES, pub dwAccess: DWORD,
	pub name: LPCWSTR
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkFenceGetWin32HandleInfoKHR
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub fence: VkFence, pub handleType: VkExternalFenceHandleTypeFlagsKHR
}
impl Default for VkImportFenceWin32HandleInfoKHR
{
	fn default() -> Self
	{
		VkImportFenceWin32HandleInfoKHR
		{
			sType: VK_STRUCTURE_TYPE_IMPORT_FENCE_WIN32_HANDLE_INFO_KHR,
			.. unsafe { std::mem::zeroed() }
		}
	}
}
impl Default for VkExportFenceWin32HandleInfoKHR
{
	fn default() -> Self
	{
		VkExportFenceWin32HandleInfoKHR
		{
			sType: VK_STRUCTURE_TYPE_EXPORT_FENCE_WIN32_HANDLE_INFO_KHR,
			.. unsafe { std::mem::zeroed() }
		}
	}
}
impl Default for VkFenceGetWin32HandleInfoKHR
{
	fn default() -> Self
	{
		VkFenceGetWin32HandleInfoKHR
		{
			sType: VK_STRUCTURE_TYPE_FENCE_GET_WIN32_HANDLE_INFO_KHR,
			.. unsafe { std::mem::zeroed() }
		}
	}
}

pub type PFN_vkImportFenceWin32HandleKHR = extern "system" fn(device: VkDevice, pImportFenceWin32HandleInfo: *const VkImportFenceWin32HandleInfoKHR) -> VkResult;
pub type PFN_vkGetFenceWin32HandleKHR = extern "system" fn(device: VkDevice, pGetWin32HandleInfo: *const VkFenceGetWin32HandleInfoKHR, pHandle: *mut HANDLE) -> VkResult;

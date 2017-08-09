//! VK_MVK_macos_surface extensions

pub const VK_MVK_MACOS_SURFACE_SPEC_VERSION: usize = 2;
pub static VK_MVK_MACOS_SURFACE_EXTENSION_NAME: &'static str = "VK_MVK_macos_surface";

use libc::*;
use super::*;

pub type VkMacOSSurfaceCreateFlagsMVK = VkFlags;

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkMacOSSurfaceCreateInfoMVK
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub flags: VkMacOSSurfaceCreateFlagsMVK, pub pView: *const c_void
}
impl Default for VkMacOSSurfaceCreateInfoMVK
{
	fn default() -> Self
	{
		VkMacOSSurfaceCreateInfoMVK
		{
			sType: VK_STRUCTURE_TYPE_MACOS_SURFACE_CREATE_INFO_MVK,
			.. unsafe { std::mem::zeroed() }
		}
	}
}

pub type PFN_vkCreateMacOSSurfaceMVK = extern "system" fn(instance: VkInstance, pCreateInfo: *const VkMacOSSurfaceCreateInfoMVK, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;

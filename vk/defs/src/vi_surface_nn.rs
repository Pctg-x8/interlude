//! VK_NN_vi_surface extensions

pub const VK_NN_VI_SURFACE_SPEC_VERSION: usize = 1;
pub static VK_NN_VI_SURFACE_EXTENSION_NAME: &'static str = "VK_NN_vi_surface";

use libc::*;
use super::*;

pub type VkViSurfaceCreateFlagsNN = VkFlags;

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkViSurfaceCreateInfoNN
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub flags: VkViSurfaceCreateFlagsNN, pub window: *mut c_void
}
impl Default for VkViSurfaceCreateInfoNN
{
	fn default() -> Self
	{
		VkViSurfaceCreateInfoNN
		{
			sType: VK_STRUCTURE_TYPE_VI_SURFACE_CREATE_INFO_NN,
			.. unsafe { std::mem::zeroed() }
		}
	}
}

pub type PFN_vkCreateViSurfaceNN = extern "system" fn(instance: VkInstance, pCreateInfo: *const VkViSurfaceCreateInfoNN, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;

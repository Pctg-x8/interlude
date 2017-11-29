//! VK_KHR_get_surface_capabilities2 extensions

pub const VK_KHR_GET_SURFACE_CAPABILITIES_2_SPEC_VERSION: usize = 1;
pub static VK_KHR_GET_SURFACE_CAPABILITIES_2_EXTENSION_NAME: &'static str = "VK_KHR_get_surface_capabilities2";

use libc::*;
use super::*;

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPhysicalDeviceSurfaceInfo2KHR
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub surface: VkSurfaceKHR
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkSurfaceCapabilities2KHR
{
	pub sType: VkStructureType, pub pNext: *mut c_void,
	pub surfaceCapabilities: VkSurfaceCapabilitiesKHR
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkSurfaceFormat2KHR
{
	pub sType: VkStructureType, pub pNext: *mut c_void,
	pub surfaceFormat: VkSurfaceFormatKHR
}
impl Default for VkPhysicalDeviceSurfaceInfo2KHR
{
	fn default() -> Self
	{
		VkPhysicalDeviceSurfaceInfo2KHR
		{
			sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SURFACE_INFO_2_KHR,
			.. unsafe { std::mem::zeroed() }
		}
	}
}
impl Default for VkSurfaceCapabilities2KHR
{
	fn default() -> Self
	{
		VkSurfaceCapabilities2KHR
		{
			sType: VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_KHR,
			.. unsafe { std::mem::zeroed() }
		}
	}
}
impl Default for VkSurfaceFormat2KHR
{
	fn default() -> Self
	{
		VkSurfaceFormat2KHR
		{
			sType: VK_STRUCTURE_TYPE_SURFACE_FORMAT_2_KHR,
			surfaceFormat: Default::default(),
			.. unsafe { std::mem::zeroed() }
		}
	}
}

pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR = extern "system" fn(physicalDevice: VkPhysicalDevice, pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR, pSurfaceCapabilities: *mut VkSurfaceCapabilities2KHR) -> VkResult;
pub type PFN_vkGetPhysicalDeviceSurfaceFormats2KHR = extern "system" fn(physicalDevice: VkPhysicalDevice, pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR, pSurfaceFormatCount: *mut u32, pSurfaceFormats: *mut VkSurfaceFormat2KHR) -> VkResult;

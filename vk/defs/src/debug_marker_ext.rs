//! VK_EXT_debug_marker extensions

pub const VK_EXT_DEBUG_MARKER_SPEC_VERSION: usize = 4;
pub static VK_EXT_DEBUG_MARKER_EXTENSION_NAME: &'static str = "VK_EXT_debug_marker";

use libc::*;
use super::*;

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDebugMarkerObjectNameInfoEXT
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub objectType: VkDebugReportObjectTypeEXT, pub object: u64, pub pObjectName: *const c_char
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDebugMarkerObjectTagInfoEXT
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub objectType: VkDebugReportObjectTypeEXT, pub object: u64, pub tagName: u64,
	pub tagSize: size_t, pub pTag: *const c_void
}
#[repr(C)] #[derive(Debug, Clone, PartialEq)]
pub struct VkDebugMarkerMarkerInfoEXT
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub pMarkerName: *const c_char, pub color: [c_float; 4]
}
impl Default for VkDebugMarkerObjectNameInfoEXT
{
	fn default() -> Self
	{
		VkDebugMarkerObjectNameInfoEXT
		{
			sType: VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_NAME_INFO_EXT,
			.. unsafe { std::mem::zeroed() }
		}
	}
}
impl Default for VkDebugMarkerObjectTagInfoEXT
{
	fn default() -> Self
	{
		VkDebugMarkerObjectTagInfoEXT
		{
			sType: VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_TAG_INFO_EXT,
			.. unsafe { std::mem::zeroed() }
		}
	}
}
impl Default for VkDebugMarkerMarkerInfoEXT
{
	fn default() -> Self
	{
		VkDebugMarkerMarkerInfoEXT
		{
			sType: VK_STRUCTURE_TYPE_DEBUG_MARKER_MARKER_INFO_EXT,
			.. unsafe { std::mem::zeroed() }
		}
	}
}

pub type PFN_vkDebugMarkerSetObjectTagEXT = extern "system" fn(device: VkDevice, pTagInfo: *const VkDebugMarkerObjectTagInfoEXT) -> VkResult;
pub type PFN_vkDebugMarkerSetObjectNameEXT = extern "system" fn(device: VkDevice, pNameInfo: *const VkDebugMarkerObjectNameInfoEXT) -> VkResult;
pub type PFN_vkCmdDebugMarkerBeginEXT = extern "system" fn(commandBuffer: VkCommandBuffer, pMarkerInfo: *const VkDebugMarkerMarkerInfoEXT);
pub type PFN_vkCmdDebugMarkerEndEXT = extern "system" fn(commandBuffer: VkCommandBuffer);
pub type PFN_vkCmdDebugMarkerInsertEXT = extern "system" fn(commandBuffer: VkCommandBuffer, pMarkerInfo: *const VkDebugMarkerMarkerInfoEXT);

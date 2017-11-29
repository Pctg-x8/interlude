//! VK_NV_viewport_swizzle extensions

pub const VK_NV_VIEWPORT_SWIZZLE_SPEC_VERSION: usize = 1;
pub static VK_NV_VIEWPORT_SWIZZLE_EXTENSION_NAME: &'static str = "VK_NV_viewport_swizzle";

use libc::*;
use super::*;

pub type VkViewportCoordinateSwizzleNV = i32;
pub const VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_X_NV: VkViewportCoordinateSwizzleNV = 0;
pub const VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_X_NV: VkViewportCoordinateSwizzleNV = 1;
pub const VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Y_NV: VkViewportCoordinateSwizzleNV = 2;
pub const VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Y_NV: VkViewportCoordinateSwizzleNV = 3;
pub const VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Z_NV: VkViewportCoordinateSwizzleNV = 4;
pub const VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Z_NV: VkViewportCoordinateSwizzleNV = 5;
pub const VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_W_NV: VkViewportCoordinateSwizzleNV = 6;
pub const VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_W_NV: VkViewportCoordinateSwizzleNV = 7;

pub type VkPipelineViewportSwizzleStateCreateFlagsNV = VkFlags;

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkViewportSwizzleNV
{
	pub x: VkViewportCoordinateSwizzleNV,
	pub y: VkViewportCoordinateSwizzleNV,
	pub z: VkViewportCoordinateSwizzleNV,
	pub w: VkViewportCoordinateSwizzleNV
}

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPipelineViewportSwizzleStateCreateInfoNV
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub flags: VkPipelineViewportSwizzleStateCreateFlagsNV,
	pub viewportCount: u32, pub pViewportSwizzles: *const VkViewportSwizzleNV
}
impl Default for VkPipelineViewportSwizzleStateCreateInfoNV
{
	fn default() -> Self
	{
		VkPipelineViewportSwizzleStateCreateInfoNV
		{
			sType: VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV,
			.. unsafe { std::mem::zeroed() }
		}
	}
}

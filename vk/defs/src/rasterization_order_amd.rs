//! VK_AMD_rasterization_order extensions

pub const VK_AMD_RASTERIZATION_ORDER_SPEC_VERSION: usize = 1;
pub static VK_AMD_RASTERIZATION_ORDER_EXTENSION_NAME: &'static str = "VK_AMD_rasterization_order";

use libc::*;
use super::*;

pub type VkRasterizationOrderAMD = i32;
pub const VK_RASTERIZATION_ORDER_STRICT_AMD: VkRasterizationOrderAMD = 0;
pub const VK_RASTERIZATION_ORDER_RELAXED_AMD: VkRasterizationOrderAMD = 1;

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPipelineRasterizationStateRasterizationOrderAMD
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub rasterizationOrder: VkRasterizationOrderAMD
}
impl Default for VkPipelineRasterizationStateRasterizationOrderAMD
{
	fn default() -> Self
	{
		VkPipelineRasterizationStateRasterizationOrderAMD
		{
			sType: VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD,
			.. unsafe { std::mem::zeroed() }
		}
	}
}

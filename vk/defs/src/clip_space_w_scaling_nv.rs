//! VK_NV_clip_space_w_scaling extensions

pub const VK_NV_CLIP_SPACE_W_SCALING_SPEC_VERSION: usize = 1;
pub static VK_NV_CLIP_SPACE_W_SCALING_EXTENSION_NAME: &'static str = "VK_NV_clip_space_w_scaling";

use libc::*;
use super::*;

#[repr(C)] #[derive(Debug, Clone, PartialEq)]
pub struct VkViewportWScalingNV { pub xcoeff: c_float, pub ycoeff: c_float }

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPipelineViewportWScalingStateCreateInfoNV
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub viewportWScalingEnable: VkBool32,
	pub viewportCount: u32, pub pViewportWScalings: *const VkViewportWScalingNV
}
impl Default for VkPipelineViewportWScalingStateCreateInfoNV
{
	fn default() -> Self
	{
		VkPipelineViewportWScalingStateCreateInfoNV
		{
			sType: VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV,
			.. unsafe { std::mem::zeroed() }
		}
	}
}

pub type PFN_vkCmdSetViewportWScalingNV = extern "system" fn(commandBuffer: VkCommandBuffer, firstViewprt: u32, viewportCount: u32, pViewportWScalings: *const VkViewportWScalingNV);

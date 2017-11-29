//! VK_NV_framebuffer_mixed_samples extensions

pub const VK_NV_FRAMEBUFFER_MIXED_SAMPLES_SPEC_VERSION: usize = 1;
pub static VK_NV_FRAMEBUFFER_MIXED_SAMPLES_EXTENSION_NAME: &'static str = "VK_NV_framebuffer_mixed_samples";

use libc::*;
use super::*;

pub type VkCoverageModulationModeNV = i32;
pub const VK_COVERAGE_MODULATION_MODE_NONE_NV: VkCoverageModulationModeNV = 0;
pub const VK_COVERAGE_MODULATION_MODE_RGB_NV: VkCoverageModulationModeNV = 1;
pub const VK_COVERAGE_MODULATION_MODE_ALPHA_NV: VkCoverageModulationModeNV = 2;
pub const VK_COVERAGE_MODULATION_MODE_RGBA_NV: VkCoverageModulationModeNV = 3;

pub type VkPipelineCoverageModulationStateCreateFlagsNV = VkFlags;

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPipelineCoverageModulationStateCreateInfoNV
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub flags: VkPipelineCoverageModulationStateCreateFlagsNV,
	pub coverageModulationMode: VkCoverageModulationModeNV,
	pub coverageModulationTableEnable: VkBool32,
	pub coverageModulationTableCount: u32,
	pub pCoverageModulationTable: *const c_float
}
impl Default for VkPipelineCoverageModulationStateCreateInfoNV
{
	fn default() -> Self
	{
		VkPipelineCoverageModulationStateCreateInfoNV
		{
			sType: VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV,
			.. unsafe { std::mem::zeroed() }
		}
	}
}

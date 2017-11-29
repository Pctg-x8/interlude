//! VK_AMD_texture_gather_bias_lod extensions

pub const VK_AMD_TEXTURE_GATHER_BIAS_LOD_SPEC_VERSION: usize = 1;
pub static VK_AMD_TEXTURE_GATHER_BIAS_LOD_EXTENSION_NAME: &'static str = "VK_AMD_texture_gather_bias_lod";

use libc::*;
use super::*;

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkTextureLODGatherFormatPropertiesAMD
{
	pub sType: VkStructureType, pub pNext: *mut c_void,
	pub supportsTextureGatherLODBiasAMD: VkBool32
}
impl Default for VkTextureLODGatherFormatPropertiesAMD
{
	fn default() -> Self
	{
		VkTextureLODGatherFormatPropertiesAMD
		{
			sType: VK_STRUCTURE_TYPE_TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD,
			.. unsafe { std::mem::zeroed() }
		}
	}
}

//! VK_KHR_get_memory_requirements2 extensions

pub const VK_KHR_GET_MEMORY_REQUIREMENTS2_SPEC_VERSION: usize = 1;
pub static VK_KHR_GET_MEMORY_REQUIREMENTS2_EXTENSION_NAME: &'static str = "VK_KHR_get_memory_requirements2";

use libc::*;
use super::*;

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkBufferMemoryRequirementsInfo2KHR
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub buffer: VkBuffer
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkImageMemoryRequirementsInfo2KHR
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub image: VkImage
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkImageSparseMemoryRequirementsInfo2KHR
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub image: VkImage
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkMemoryRequirements2KHR
{
	pub sType: VkStructureType, pub pNext: *mut c_void,
	pub memoryRequirements: VkMemoryRequirements
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkSparseImageMemoryRequirements2KHR
{
	pub sType: VkStructureType, pub pNext: *mut c_void,
	pub memoryRequirements: VkSparseImageMemoryRequirements
}
impl Default for VkBufferMemoryRequirementsInfo2KHR
{
	fn default() -> Self
	{
		VkBufferMemoryRequirementsInfo2KHR
		{
			sType: VK_STRUCTURE_TYPE_BUFFER_MEMORY_REQUIREMENTS_INFO_2_KHR,
			.. unsafe { std::mem::zeroed() }
		}
	}
}
impl Default for VkImageMemoryRequirementsInfo2KHR
{
	fn default() -> Self
	{
		VkImageMemoryRequirementsInfo2KHR
		{
			sType: VK_STRUCTURE_TYPE_IMAGE_MEMORY_REQUIREMENTS_INFO_2_KHR,
			.. unsafe { std::mem::zeroed() }
		}
	}
}
impl Default for VkImageSparseMemoryRequirementsInfo2KHR
{
	fn default() -> Self
	{
		VkImageSparseMemoryRequirementsInfo2KHR
		{
			sType: VK_STRUCTURE_TYPE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2_KHR,
			.. unsafe { std::mem::zeroed() }
		}
	}
}
impl Default for VkMemoryRequirements2KHR
{
	fn default() -> Self
	{
		VkMemoryRequirements2KHR
		{
			sType: VK_STRUCTURE_TYPE_MEMORY_REQUIREMENTS_2_KHR,
			.. unsafe { std::mem::zeroed() }
		}
	}
}
impl Default for VkSparseImageMemoryRequirements2KHR
{
	fn default() -> Self
	{
		VkSparseImageMemoryRequirements2KHR
		{
			sType: VK_STRUCTURE_TYPE_SPARSE_IMAGE_MEMORY_REQUIREMENTS_2_KHR,
			.. unsafe { std::mem::zeroed() }
		}
	}
}

pub type PFN_vkGetImageMemoryRequirements2KHR = extern "system" fn(device: VkDevice, pInfo: *const VkImageMemoryRequirementsInfo2KHR, pMemoryRequirements: *mut VkMemoryRequirements2KHR);
pub type PFN_vkGetBufferMemoryRequirements2KHR = extern "system" fn(device: VkDevice, pInfo: *const VkBufferMemoryRequirementsInfo2KHR, pMemoryRequirements: *mut VkMemoryRequirements2KHR);
pub type PFN_vkGetImageSparseMemoryRequirements2KHR = extern "system" fn(device: VkDevice, pInfo: *const VkImageSparseMemoryRequirementsInfo2KHR, pSparseMemoryRequirementCount: *mut u32, pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2KHR);

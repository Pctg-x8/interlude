//! VK_NV_dedicated_allocation extensions

pub const VK_NV_DEDICATED_ALLOCATION_SPEC_VERSION: usize = 1;
pub static VK_NV_DEDICATED_ALLOCATION_EXTENSION_NAME: &'static str = "VK_NV_dedicated_allocation";

use libc::*;
use super::*;

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDedicatedAllocationImageCreateInfoNV
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub dedicatedAllocation: VkBool32
}
impl Default for VkDedicatedAllocationImageCreateInfoNV
{
	fn default() -> Self
	{
		VkDedicatedAllocationImageCreateInfoNV
		{
			sType: VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV,
			.. unsafe { std::mem::zeroed() }
		}
	}
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDedicatedAllocationBufferCreateInfoNV
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub dedicatedAllocation: VkBool32
}
impl Default for VkDedicatedAllocationBufferCreateInfoNV
{
	fn default() -> Self
	{
		VkDedicatedAllocationBufferCreateInfoNV
		{
			sType: VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV,
			.. unsafe { std::mem::zeroed() }
		}
	}
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDedicatedAllocationMemoryAllocateInfoNV
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub image: VkImage, pub buffer: VkBuffer
}
impl Default for VkDedicatedAllocationMemoryAllocateInfoNV
{
	fn default() -> Self
	{
		VkDedicatedAllocationMemoryAllocateInfoNV
		{
			sType: VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV,
			.. unsafe { std::mem::zeroed() }
		}
	}
}

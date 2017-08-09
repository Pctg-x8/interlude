//! VK_KHR_dedicated_allocation extensions

pub const VK_KHR_DEDICATED_ALLOCATION_SPEC_VERSION: usize = 1;
pub static VK_KHR_DEDICATED_ALLOCATION_EXTENSION_NAME: &'static str = "VK_KHR_dedicated_allocation";

use libc::*;
use super::*;

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkMemoryDedicatedRequirementsKHR
{
	pub sType: VkStructureType, pub pNext: *mut c_void,
	pub prefersDedicatedAllocation: VkBool32,
	pub requiresDedicatedAllocaion: VkBool32
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkMemoryDedicatedAllocateInfoKHR
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub image: VkImage, pub buffer: VkBuffer
}
impl Default for VkMemoryDedicatedRequirementsKHR
{
	fn default() -> Self
	{
		VkMemoryDedicatedRequirementsKHR
		{
			sType: VK_STRUCTURE_TYPE_MEMORY_DEDICATED_REQUIREMENTS_KHR,
			.. unsafe { std::mem::zeroed() }
		}
	}
}
impl Default for VkMemoryDedicatedAllocateInfoKHR
{
	fn default() -> Self
	{
		VkMemoryDedicatedAllocateInfoKHR
		{
			sType: VK_STRUCTURE_TYPE_MEMORY_DEDICATED_ALLOCATE_INFO_KHR,
			.. unsafe { std::mem::zeroed() }
		}
	}
}

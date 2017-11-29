//! VK_KHX_device_group extensions

pub const VK_KHX_DEVICE_GROUP_SPEC_VERSION: usize = 1;
pub static VK_KHX_DEVICE_GROUP_EXTENSION_NAME: &'static str = "VK_KHX_device_group";
pub const VK_MAX_DEVICE_GROUP_SIZE_KHX: usize = 32;

use libc::*;
use super::*;

pub type VkPeerMemoryFeatureFlagsKHX = VkFlags;
pub const VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT_KHX: VkPeerMemoryFeatureFlagsKHX = 0x01;
pub const VK_PEER_MEMORY_FEATURE_COPY_DST_BIT_KHX: VkPeerMemoryFeatureFlagsKHX = 0x02;
pub const VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT_KHX: VkPeerMemoryFeatureFlagsKHX = 0x04;
pub const VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT_KHX: VkPeerMemoryFeatureFlagsKHX = 0x08;

pub type VkMemoryAllocateFlagsKHX = VkFlags;
pub const VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT_KHX: VkMemoryAllocateFlagsKHX = 0x01;

pub type VkDeviceGroupPresentModeFlagsKHX = VkFlags;
pub const VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHX: VkDeviceGroupPresentModeFlagsKHX = 0x01;
pub const VK_DEVICE_GROUP_PRESENT_MODE_REMOTE_BIT_KHX: VkDeviceGroupPresentModeFlagsKHX = 0x02;
pub const VK_DEVICE_GROUP_PRESENT_MODE_SUM_BIT_KHX: VkDeviceGroupPresentModeFlagsKHX = 0x04;
pub const VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_MULTI_DEVICE_BIT_KHX: VkDeviceGroupPresentModeFlagsKHX = 0x08;


#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkMemoryAllocateFlagsInfoKHX
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub flags: VkMemoryAllocateFlagsKHX, pub deviceMask: u32
}
impl Default for VkMemoryAllocateFlagsInfoKHX
{
	fn default() -> Self
	{
		VkMemoryAllocateFlagsInfoKHX
		{
			sType: VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO_KHX,
			.. unsafe { std::mem::zeroed() }
		}
	}
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkBindBufferMemoryInfoKHX
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub buffer: VkBuffer, pub memory: VkDeviceMemory, pub memoryOffset: VkDeviceSize,
	pub deviceIndexCount: u32, pub pDeviceIndices: *const u32
}
impl Default for VkBindBufferMemoryInfoKHX
{
	fn default() -> Self
	{
		VkBindBufferMemoryInfoKHX
		{
			sType: VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO_KHX,
			.. unsafe { std::mem::zeroed() }
		}
	}
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkBindImageMemoryInfoKHX
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub image: VkImage, pub memory: VkDeviceMemory, pub memoryOffset: VkDeviceSize,
	pub deviceIndexCount: u32, pub pDeviceIndices: *const u32,
	pub SFRRectCount: u32, pub pSFRRects: *const VkRect2D
}
impl Default for VkBindImageMemoryInfoKHX
{
	fn default() -> Self
	{
		VkBindImageMemoryInfoKHX
		{
			sType: VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO_KHX,
			.. unsafe { std::mem::zeroed() }
		}
	}
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDeviceGroupRenderPassBeginInfoKHX
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub deviceMask: u32, pub deviceRenderAreaCount: u32, pub pDeviceRenderAreas: *const VkRect2D
}
impl Default for VkDeviceGroupRenderPassBeginInfoKHX
{
	fn default() -> Self
	{
		VkDeviceGroupRenderPassBeginInfoKHX
		{
			sType: VK_STRUCTURE_TYPE_DEVICE_GROUP_RENDER_PASS_BEGIN_INFO_KHX,
			.. unsafe { std::mem::zeroed() }
		}
	}
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDeviceGroupCommandBufferBeginInfoKHX
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub deviceMask: u32
}
impl Default for VkDeviceGroupCommandBufferBeginInfoKHX
{
	fn default() -> Self
	{
		VkDeviceGroupCommandBufferBeginInfoKHX
		{
			sType: VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO_KHX,
			.. unsafe { std::mem::zeroed() }
		}
	}
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDeviceGroupSubmitInfoKHX
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub waitSemaphoreCount: u32, pub pWaitSemaphoreDeviceIndices: *const u32,
	pub commandBufferCount: u32, pub pCommandBufferDeviceMasks: *const u32,
	pub signalSemaphoreCount: u32, pub pSignalSemaphoreDeviceIndices: *const u32
}
impl Default for VkDeviceGroupSubmitInfoKHX
{
	fn default() -> Self
	{
		VkDeviceGroupSubmitInfoKHX
		{
			sType: VK_STRUCTURE_TYPE_DEVICE_GROUP_SUBMIT_INFO_KHX,
			.. unsafe { std::mem::zeroed() }
		}
	}
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDeviceGroupBindSparseInfoKHX
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub resourceDeviceIndex: u32, pub memoryDeviceIndex: u32
}
impl Default for VkDeviceGroupBindSparseInfoKHX
{
	fn default() -> Self
	{
		VkDeviceGroupBindSparseInfoKHX
		{
			sType: VK_STRUCTURE_TYPE_DEVICE_GROUP_BIND_SPARSE_INFO_KHX,
			.. unsafe { std::mem::zeroed() }
		}
	}
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDeviceGroupPresentCapabilitiesKHX
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub presentMask: [u32; VK_MAX_DEVICE_GROUP_SIZE_KHX],
	pub modes: VkDeviceGroupPresentModeFlagsKHX
}
impl Default for VkDeviceGroupPresentCapabilitiesKHX
{
	fn default() -> Self
	{
		VkDeviceGroupPresentCapabilitiesKHX
		{
			sType: VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_CAPABILITIES_KHX,
			.. unsafe { std::mem::zeroed() }
		}
	}
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkImageSwapchainCreateInfoKHX
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub swapchain: VkSwapchainKHR
}
impl Default for VkImageSwapchainCreateInfoKHX
{
	fn default() -> Self
	{
		VkImageSwapchainCreateInfoKHX
		{
			sType: VK_STRUCTURE_TYPE_IMAGE_SWAPCHAIN_CREATE_INFO_KHX,
			.. unsafe { std::mem::zeroed() }
		}
	}
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkBindImageMemorySwapchainInfoKHX
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub swapchain: VkSwapchainKHR, pub imageIndex: u32
}
impl Default for VkBindImageMemorySwapchainInfoKHX
{
	fn default() -> Self
	{
		VkBindImageMemorySwapchainInfoKHX
		{
			sType: VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHX,
			.. unsafe { std::mem::zeroed() }
		}
	}
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkAcquireNextImageInfoKHX
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub swapchain: VkSwapchainKHR, pub timeout: u64,
	pub semaphore: VkSemaphore, pub fence: VkFence, pub deviceMask: u32
}
impl Default for VkAcquireNextImageInfoKHX
{
	fn default() -> Self
	{
		VkAcquireNextImageInfoKHX
		{
			sType: VK_STRUCTURE_TYPE_ACQUIRE_NEXT_IMAGE_INFO_KHX,
			.. unsafe { std::mem::zeroed() }
		}
	}
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDeviceGroupPresentInfoKHX
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub swapchainCount: u32, pub pDeviceMasks: *const u32,
	pub mode: VkDeviceGroupPresentModeFlagsKHX
}
impl Default for VkDeviceGroupPresentInfoKHX
{
	fn default() -> Self
	{
		VkDeviceGroupPresentInfoKHX
		{
			sType: VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_INFO_KHX,
			.. unsafe { std::mem::zeroed() }
		}
	}
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDeviceGroupSwapchainCreateInfoKHX
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub modes: VkDeviceGroupPresentModeFlagsKHX
}
impl Default for VkDeviceGroupSwapchainCreateInfoKHX
{
	fn default() -> Self
	{
		VkDeviceGroupSwapchainCreateInfoKHX
		{
			sType: VK_STRUCTURE_TYPE_DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHX,
			.. unsafe { std::mem::zeroed() }
		}
	}
}

pub type PFN_vkGetDeviceGroupPeerMemoryFeaturesKHX = extern "system" fn(device: VkDevice, heapIndex: u32, localDeviceIndex: u32, remoteDeviceIndex: u32, pPeerMemoryFeatures: *mut VkPeerMemoryFeatureFlagsKHX);
pub type PFN_vkBindBufferMemory2KHX = extern "system" fn(device: VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindBufferMemoryInfoKHX) -> VkResult;
pub type PFN_vkBindImageMemory2KHX = extern "system" fn(device: VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindImageMemoryInfoKHX) -> VkResult;
pub type PFN_vkCmdSetDeviceMaskKHX = extern "system" fn(commandBuffer: VkCommandBuffer, deviceMask: u32);
pub type PFN_vkGetDeviceGroupPresentCapabilitiesKHX = extern "system" fn(device: VkDevice, pDeviceGroupPresentCapabilities: *mut VkDeviceGroupPresentCapabilitiesKHX) -> VkResult;
pub type PFN_vkGetDeviceGroupSurfacePresentModesKHX = extern "system" fn(device: VkDevice, surface: VkSurfaceKHR, pModes: *mut VkDeviceGroupPresentModeFlagsKHX) -> VkResult;
pub type PFN_vkAcquireNextImage2KHX = extern "system" fn(device: VkDevice, pAcquireInfo: *const VkAcquireNextImageInfoKHX, pImageIndex: *mut u32) -> VkResult;
pub type PFN_vkCmdDispatchBaseKHX = extern "system" fn(commandBuffer: VkCommandBuffer, baseGroupX: u32, baseGroupY: u32, baseGroupZ: u32, groupCountX: u32, groupCountY: u32, groupCountZ: u32);
pub type PNF_vkGetPhysicalDevicePresentRectanglesKHX = extern "system" fn(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pRectCount: *mut u32, pRects: *mut VkRect2D) -> VkResult;

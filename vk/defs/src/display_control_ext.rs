//! VK_EXT_display_control extensions

pub const VK_EXT_DISPLAY_CONTROL_SPEC_VERSION: usize = 1;
pub static VK_EXT_DISPLAY_CONTROL_EXTENSION_NAME: &'static str = "VK_EXT_display_control";

use libc::*;
use super::*;

pub type VkDisplayPowerStateEXT = i32;
pub const VK_DISPLAY_POWER_STATE_OFF_EXT: VkDisplayPowerStateEXT = 0;
pub const VK_DISPLAY_POWER_STATE_SUSPEND_EXT: VkDisplayPowerStateEXT = 1;
pub const VK_DISPLAY_POWER_STATE_ON_EXT: VkDisplayPowerStateEXT = 2;

pub type VkDeviceEventTypeEXT = i32;
pub const VK_DEVICE_EVENT_TYPE_DISPLAY_HOTPLUG_EXT: VkDeviceEventTypeEXT = 0;

pub type VkDisplayEventTypeEXT = i32;
pub const VK_DISPLAY_EVENT_TYPE_FIRST_PIXEL_OUT_EXT: VkDisplayEventTypeEXT = 0;

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDisplayPowerInfoEXT
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub powerState: VkDisplayPowerStateEXT
}
impl Default for VkDisplayPowerInfoEXT
{
	fn default() -> Self
	{
		VkDisplayPowerInfoEXT
		{
			sType: VK_STRUCTURE_TYPE_DISPLAY_POWER_INFO_EXT,
			.. unsafe { std::mem::zeroed() }
		}
	}
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDeviceEventInfoEXT
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub deviceEvent: VkDeviceEventTypeEXT
}
impl Default for VkDeviceEventInfoEXT
{
	fn default() -> Self
	{
		VkDeviceEventInfoEXT
		{
			sType: VK_STRUCTURE_TYPE_DEVICE_EVENT_INFO_EXT,
			.. unsafe { std::mem::zeroed() }
		}
	}
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDisplayEventInfoEXT
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub displayEvent: VkDisplayEventTypeEXT
}
impl Default for VkDisplayEventInfoEXT
{
	fn default() -> Self
	{
		VkDisplayEventInfoEXT
		{
			sType: VK_STRUCTURE_TYPE_DISPLAY_EVENT_INFO_EXT,
			.. unsafe { std::mem::zeroed() }
		}
	}
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkSwapchainCounterCreateInfoEXT
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub surfaceCounters: VkSurfaceCounterFlagsEXT
}
impl Default for VkSwapchainCounterCreateInfoEXT
{
	fn default() -> Self
	{
		VkSwapchainCounterCreateInfoEXT
		{
			sType: VK_STRUCTURE_TYPE_SWAPCHAIN_COUNTER_CREATE_INFO_EXT,
			.. unsafe { std::mem::zeroed() }
		}
	}
}

pub type PFN_vkDisplayPowerControlEXT = extern "system" fn(device: VkDevice, display: VkDisplayKHR, pDisplayPowerInfo: *const VkDisplayPowerInfoEXT) -> VkResult;
pub type PFN_vkRegisterDeviceEventEXT = extern "system" fn(device: VkDevice, pDeviceEventInfo: *const VkDeviceEventInfoEXT, pAllocator: *const VkAllocationCallbacks, pFence: *mut VkFence) -> VkResult;
pub type PFN_vkRegisterDisplayEventEXT = extern "system" fn(device: VkDevice, display: VkDisplayKHR, pDisplayEventInfo: *const VkDisplayEventInfoEXT, pAllocator: *const VkAllocationCallbacks, pFence: *mut VkFence) -> VkResult;
pub type PFN_vkGetSwapchainCounterEXT = extern "system" fn(device: VkDevice, swapchain: VkSwapchainKHR, counter: VkSurfaceCounterFlagsEXT, pCounterValue: *mut u64) -> VkResult;

//! VK_EXT_debug_report extensions

pub const VK_EXT_DEBUG_REPORT_SPEC_VERSION: usize = 8;
pub static VK_EXT_DEBUG_REPORT_EXTENSION_NAME: &'static str = "VK_EXT_debug_report";

use libc::*;
use super::*;

mod nd_handle_base_ts { pub enum VkDebugReportCallbackEXT {} }
pub type VkDebugReportCallbackEXT = VK_NON_DISPATCHABLE_HANDLE!(VkDebugReportCallbackEXT);

pub type VkDebugReportObjectTypeEXT = i32;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT: VkDebugReportObjectTypeEXT = 0;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_INSTANCE_EXT: VkDebugReportObjectTypeEXT = 1;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_PHYSICAL_DEVICE_EXT: VkDebugReportObjectTypeEXT = 2;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_EXT: VkDebugReportObjectTypeEXT = 3;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_QUEUE_EXT: VkDebugReportObjectTypeEXT = 4;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_SEMAPHORE_EXT: VkDebugReportObjectTypeEXT = 5;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_BUFFER_EXT: VkDebugReportObjectTypeEXT = 6;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_FENCE_EXT: VkDebugReportObjectTypeEXT = 7;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_MEMORY_EXT: VkDebugReportObjectTypeEXT = 8;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_EXT: VkDebugReportObjectTypeEXT = 9;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_EXT: VkDebugReportObjectTypeEXT = 10;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_EVENT_EXT: VkDebugReportObjectTypeEXT = 11;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_QUERY_POOL_EXT: VkDebugReportObjectTypeEXT = 12;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_VIEW_EXT: VkDebugReportObjectTypeEXT = 13;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_VIEW_EXT: VkDebugReportObjectTypeEXT = 14;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_SHADER_MODULE_EXT: VkDebugReportObjectTypeEXT = 15;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_CACHE_EXT: VkDebugReportObjectTypeEXT = 16;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_LAYOUT_EXT: VkDebugReportObjectTypeEXT = 17;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_RENDER_PASS_EXT: VkDebugReportObjectTypeEXT = 18;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_EXT: VkDebugReportObjectTypeEXT = 19;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT_EXT: VkDebugReportObjectTypeEXT = 20;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_EXT: VkDebugReportObjectTypeEXT = 21;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_POOL_EXT: VkDebugReportObjectTypeEXT = 22;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_EXT: VkDebugReportObjectTypeEXT = 23;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_FRAMEBUFFER_EXT: VkDebugReportObjectTypeEXT = 24;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_POOL_EXT: VkDebugReportObjectTypeEXT = 25;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_SURFACE_KHR_EXT: VkDebugReportObjectTypeEXT = 26;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_SWAPCHAIN_KHR_EXT: VkDebugReportObjectTypeEXT = 27;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT_EXT: VkDebugReportObjectTypeEXT = 28;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_DISPLAY_KHR_EXT: VkDebugReportObjectTypeEXT = 29;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_DISPLAY_MODE_KHR_EXT: VkDebugReportObjectTypeEXT = 30;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_OBJECT_TABLE_NVX_EXT: VkDebugReportObjectTypeEXT = 31;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_INDIRECT_COMMANDS_LAYOUT_NVX_EXT: VkDebugReportObjectTypeEXT = 32;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_KHR_EXT: VkDebugReportObjectTypeEXT = 100_0085_000;

pub type VkDebugReportFlagsEXT = VkFlags;
pub const VK_DEBUG_REPORT_INFORMATION_BIT_EXT: VkDebugReportFlagsEXT = 0x01;
pub const VK_DEBUG_REPORT_WARNING_BIT_EXT: VkDebugReportFlagsEXT = 0x02;
pub const VK_DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT: VkDebugReportFlagsEXT = 0x04;
pub const VK_DEBUG_REPORT_ERROR_BIT_EXT: VkDebugReportFlagsEXT = 0x08;
pub const VK_DEBUG_REPORT_DEBUG_BIT_EXT: VkDebugReportFlagsEXT = 0x10;

#[repr(C)]
pub struct VkDebugReportCallbackCreateInfoEXT
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub flags: VkDebugReportFlagsEXT, pub pfnCallback: PFN_vkDebugReportCallbackEXT,
	pub pUserData: *mut c_void
}
impl Default for VkDebugReportCallbackCreateInfoEXT
{
	fn default() -> Self
	{
		VkDebugReportCallbackCreateInfoEXT
		{
			sType: VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT,
			.. unsafe { std::mem::zeroed() }
		}
	}
}

pub type PFN_vkDebugReportCallbackEXT = extern "system" fn(flags: VkDebugReportFlagsEXT, objectType: VkDebugReportObjectTypeEXT, object: u64, location: size_t, messageCode: i32, pLayerPrefix: *const c_char, pMessage: *const c_char, pUserData: *mut c_void) -> VkBool32;
pub type PFN_vkCreateDebugReportCallbackEXT = extern "system" fn(instance: VkInstance, pCreateInfo: *const VkDebugReportCallbackCreateInfoEXT, pAllocator: *const VkAllocationCallbacks, pCallback: *mut VkDebugReportCallbackEXT) -> VkResult;
pub type PFN_vkDestroyDebugReportCallbackEXT = extern "system" fn(instance: VkInstance, callback: VkDebugReportCallbackEXT, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkDebugReportMessageEXT = extern "system" fn(instance: VkInstance, flags: VkDebugReportFlagsEXT, objectType: VkDebugReportObjectTypeEXT, object: u64, location: size_t, messageCode: i32, pLayerPrefix: *const c_char, pMessage: *const c_char);

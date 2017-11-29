#![allow(non_camel_case_types)]

// Vulkan C to Rust FFI Type Aliases

use libc::*;
use super::*;

pub type VkFlags = u32;
pub type VkBool32 = u32;
pub type VkDeviceSize = u64;
pub type VkSampleMask = u32;

pub type VkInstanceCreateFlags = VkFlags;
pub type VkSampleCountFlags = VkFlags;
pub type VkQueueFlags = VkFlags;
pub type VkMemoryPropertyFlags = VkFlags;
pub type VkMemoryHeapFlags = VkFlags;
pub type VkDeviceCreateFlags = VkFlags;
pub type VkDeviceQueueCreateFlags = VkFlags;
pub type VkImageUsageFlags = VkFlags;
pub type VkImageAspectFlags = VkFlags;
pub type VkAttachmentDescriptionFlags = VkFlags;
pub type VkDescriptorPoolCreateFlags = VkFlags;
pub type VkDescriptorPoolResetFlags = VkFlags;
pub type VkFramebufferCreateFlags = VkFlags;
pub type VkRenderPassCreateFlags = VkFlags;
pub type VkSubpassDescriptionFlags = VkFlags;
pub type VkAccessFlags = VkFlags;
pub type VkSampleCountFlagBits = VkFlags;
pub type VkPipelineStageFlags = VkFlags;
pub type VkMemoryMapFlags = VkFlags;
pub type VkDependencyFlags = VkFlags;
pub type VkCommandPoolCreateFlags = VkFlags;
pub type VkQueryControlFlags = VkFlags;
pub type VkQueryPipelineStatisticFlags = VkFlags;
pub type VkCommandBufferUsageFlags = VkFlags;
pub type VkBufferUsageFlags = VkFlags;
pub type VkBufferViewCreateFlags = VkFlags;
pub type VkImageViewCreateFlags = VkFlags;
pub type VkShaderModuleCreateFlags = VkFlags;
pub type VkPipelineCacheCreateFlags = VkFlags;
pub type VkPipelineCreateFlags = VkFlags;
pub type VkPipelineShaderStageCreateFlags = VkFlags;
pub type VkPipelineVertexInputStateCreateFlags = VkFlags;
pub type VkPipelineInputAssemblyStateCreateFlags = VkFlags;
pub type VkPipelineTessellationStateCreateFlags = VkFlags;
pub type VkPipelineViewportStateCreateFlags = VkFlags;
pub type VkPipelineRasterizationStateCreateFlags = VkFlags;
pub type VkCullModeFlags = VkFlags;
pub type VkPipelineMultisampleStateCreateFlags = VkFlags;
pub type VkPipelineDepthStencilStateCreateFlags = VkFlags;
pub type VkPipelineColorBlendStateCreateFlags = VkFlags;
pub type VkColorComponentFlags = VkFlags;
pub type VkPipelineDynamicStateCreateFlags = VkFlags;
pub type VkPipelineLayoutCreateFlags = VkFlags;
pub type VkShaderStageFlags = VkFlags;
pub type VkFenceCreateFlags = VkFlags;
pub type VkSemaphoreCreateFlags = VkFlags;
pub type VkBufferCreateFlags = VkFlags;
pub type VkImageCreateFlags = VkFlags;
pub type VkDescriptorSetLayoutCreateFlags = VkFlags;
pub type VkSamplerCreateFlags = VkFlags;

pub type VkShaderStageFlagBits = VkFlags;

pub type VkXlibSurfaceCreateFlagsKHR = VkFlags;
pub type VkXcbSurfaceCreateFlagsKHR = VkFlags;
pub type VkWin32SurfaceCreateFlagsKHR = VkFlags;
pub type VkSurfaceTransformFlagsKHR = VkFlags;
pub type VkCompositeAlphaFlagsKHR = VkFlags;
pub type VkSwapchainCreateFlagsKHR = VkFlags;

pub type VkDebugReportFlagsEXT = VkFlags;

pub fn bool_to_str(val: VkBool32) -> &'static str
{
	if val == true as VkBool32 { "true" } else { "false" }
}

// Function Pointers
pub type PFN_vkAllocationFunction = unsafe extern "system" fn(pUserData: *mut c_void, size: size_t, alignment: size_t, allocationScope: VkSystemAllocationScope) -> *mut c_void;
pub type PFN_vkReallocationFunction = unsafe extern "system" fn(pUserData: *mut c_void, pOriginal: *mut c_void, size: size_t, alignment: size_t, allocationScope: VkSystemAllocationScope) -> *mut c_void;
pub type PFN_vkFreeFunction = unsafe extern "system" fn(pUserData: *mut c_void, pMemory: *mut c_void);
pub type PFN_vkInternalAllocationNotification = unsafe extern "system" fn(pUserData: *mut c_void, size: size_t, allocationType: VkInternalAllocationType, allocationScope: VkSystemAllocationScope);
pub type PFN_vkInternalFreeNotification = unsafe extern "system" fn(pUserData: *mut c_void, size: size_t, allocationType: VkInternalAllocationType, allocationScope: VkSystemAllocationScope);

pub type PFN_vkCreateDebugReportCallbackEXT = unsafe extern "system" fn(instance: VkInstance, pCreateInfo: *const VkDebugReportCallbackCreateInfoEXT, pAllocator: *const VkAllocationCallbacks, pCallback: *mut VkDebugReportCallbackEXT) -> VkResult;
pub type PFN_vkDestroyDebugReportCallbackEXT = unsafe extern "system" fn(instance: VkInstance, callback: VkDebugReportCallbackEXT, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkDebugReportMessageEXT = unsafe extern "system" fn(instance: VkInstance, flags: VkDebugReportFlagsEXT, objectType: VkDebugReportObjectTypeEXT, object: u64, location: size_t, messageCode: i32, pLayerPrefix: *const c_char, pMessage: *const c_char);
pub type PFN_vkDebugReportCallbackEXT = unsafe extern "system" fn(flags: VkDebugReportFlagsEXT, objectType: VkDebugReportObjectTypeEXT, object: u64, location: size_t, messageCode: i32, pLayerPrefix: *const c_char, pMessage: *const c_char, pUserData: *mut c_void) -> VkBool32;

pub type PFN_vkVoidFunction = unsafe extern "system" fn();

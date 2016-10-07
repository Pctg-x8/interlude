// Vulkan C to Rust FFI Type Aliases

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
pub type VkSurfaceTransformFlagsKHR = VkFlags;
pub type VkCompositeAlphaFlagsKHR = VkFlags;
pub type VkSwapchainCreateFlagsKHR = VkFlags;

pub type VkDebugReportFlagsEXT = VkFlags;

pub fn bool_to_str(val: VkBool32) -> &'static str
{
	if val == true as VkBool32 { "true" } else { "false" }
}

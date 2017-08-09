
extern crate libc;
extern crate interlude_vk_defs;
use libc::*;
use interlude_vk_defs::*;

#[link(name = "vulkan")]
extern "system"
{
    pub fn vkCreateInstance(pCreateInfo: *const VkInstanceCreateInfo, pAllocator: *const VkAllocationCallbacks, pInstance: *mut VkInstance) -> VkResult;
    pub fn vkDestroyInstance(instance: VkInstance, pAllocator: *const VkAllocationCallbacks);
    pub fn vkEnumeratePhysicalDevices(instance: VkInstance, pPhysicalDeviceCount: *mut u32, pPhysicalDevices: *mut VkPhysicalDevice) -> VkResult;
    pub fn vkGetPhysicalDeviceFeatures(physicalDevice: VkPhysicalDevice, pFeatures: *mut VkPhysicalDeviceFeatures);
    pub fn vkGetPhysicalDeviceFormatProperties(physicalDevice: VkPhysicalDevice, format: VkFormat, pFormatProperties: *mut VkFormatProperties);
    pub fn vkGetPhysicalDeviceImageFormatProperties(physicalDevice: VkPhysicalDevice, format: VkFormat, itype: VkImageType, tiling: VkImageTiling, usage: VkImageUsageFlags, flags: VkImageCreateFlags, pImageFormatProperties: *mut VkImageFormatProperties) -> VkResult;
    pub fn vkGetPhysicalDeviceProperties(physicalDevice: VkPhysicalDevice, pProperties: *mut VkPhysicalDeviceProperties);
    pub fn vkGetPhysicalDeviceQueueFamilyProperties(physicalDevice: VkPhysicalDevice, pQueueFamilyPropertyCount: *mut u32, pQueueFamilyProperties: *mut VkQueueFamilyProperties);
    pub fn vkGetPhysicalDeviceMemoryProperties(physicalDevice: VkPhysicalDevice, pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties);
    pub fn vkGetInstanceProcAddr(instance: VkInstance, pName: *const c_char) -> PFN_vkVoidFunction;
    pub fn vkGetDeviceProcAddr(device: VkDevice, pName: *const c_char) -> PFN_vkVoidFunction;
    pub fn vkCreateDevice(physicalDevice: VkPhysicalDevice, pCreateInfo: *const VkDeviceCreateInfo, pAllocator: *const VkAllocationCallbacks, pDevice: *mut VkDevice) -> VkResult;
    pub fn vkDestroyDevice(device: VkDevice, pAllocator: *const VkAllocationCallbacks);
    pub fn vkEnumerateInstanceExtensionProperties(pLayerName: *const c_char, pPropertyCount: *mut u32, pProperties: *mut VkExtensionProperties) -> VkResult;
    pub fn vkEnumerateDeviceExtensionProperties(physicalDevice: VkPhysicalDevice, pLayerName: *const c_char, pPropertyCount: *mut u32, pProperties: *mut VkExtensionProperties) -> VkResult;
    pub fn vkEnumerateInstanceLayerProperties(pPropertyCount: *mut u32, pProperties: *mut VkLayerProperties) -> VkResult;
    pub fn vkEnumerateDeviceLayerProperties(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkLayerProperties) -> VkResult;
    pub fn vkGetDeviceQueue(device: VkDevice, queueFamilyIndex: u32, queueIndex: u32, pQueue: *mut VkQueue);
    pub fn vkQueueSubmit(queue: VkQueue, submitCount: u32, pSubmits: *const VkSubmitInfo, fence: VkFence) -> VkResult;
    pub fn vkQueueWaitIdle(queue: VkQueue) -> VkResult;
    pub fn vkDeviceWaitIdle(device: VkDevice) -> VkResult;
    pub fn vkAllocateMemory(device: VkDevice, pAllocateInfo: *const VkMemoryAllocateInfo, pAllocator: *const VkAllocationCallbacks, pMemory: *mut VkDeviceMemory) -> VkResult;
    pub fn vkFreeMemory(device: VkDevice, memory: VkDeviceMemory, pAllocator: *const VkAllocationCallbacks);
    pub fn vkMapMemory(device: VkDevice, memory: VkDeviceMemory, offset: VkDeviceSize, size: VkDeviceSize, flags: VkMemoryMapFlags, ppData: *mut *mut c_void) -> VkResult;
    pub fn vkUnmapMemory(device: VkDevice, memory: VkDeviceMemory);
    pub fn vkFlushMappedMemoryRanges(device: VkDevice, memoryRangeCount: u32, pMemoryRanges: *const VkMappedMemoryRange) -> VkResult;
    pub fn vkInvalidateMappedMemoryRanges(device: VkDevice, memoryRangeCount: u32, pMemoryRanges: *const VkMappedMemoryRange) -> VkResult;
    pub fn vkGetDeviceMemoryCommitment(device: VkDevice, memory: VkDeviceMemory, pCommittedMemoryInBytes: *mut VkDeviceSize);
    pub fn vkBindBufferMemory(device: VkDevice, buffer: VkBuffer, memory: VkDeviceMemory, memoryOffset: VkDeviceSize) -> VkResult;
    pub fn vkBindImageMemory(device: VkDevice, image: VkImage, memory: VkDeviceMemory, memoryOffset: VkDeviceSize) -> VkResult;
    pub fn vkGetBufferMemoryRequirements(device: VkDevice, buffer: VkBuffer, pMemoryRequirements: *mut VkMemoryRequirements);
    pub fn vkGetImageMemoryRequirements(device: VkDevice, image: VkImage, pMemoryRequirements: *mut VkMemoryRequirements);
    pub fn vkGetImageSparseMemoryRequirements(device: VkDevice, image: VkImage, pSparseMemoryRequirementCount: *mut u32, pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements);
    pub fn vkGetPhysicalDeviceSparseImageFormatProperties(physicalDevice: VkPhysicalDevice, format: VkFormat, _type: VkImageType, samples: VkSampleCountFlags, usage: VkImageUsageFlags, tiling: VkImageTiling, pPropertyCount: *mut u32, pProperties: *mut VkSparseImageFormatProperties);
    pub fn vkQueueBindSparse(queue: VkQueue, bindInfoCount: u32, pBindInfp: *const VkBindSparseInfo, fence: VkFence) -> VkResult;
    pub fn vkCreateFence(device: VkDevice, pCreateInfo: *const VkFenceCreateInfo, pAllocator: *const VkAllocationCallbacks, pFence: *mut VkFence) -> VkResult;
    pub fn vkDestroyFence(device: VkDevice, fence: VkFence, pAllocator: *const VkAllocationCallbacks);
    pub fn vkResetFences(device: VkDevice, fenceCount: u32, pFences: *const VkFence) -> VkResult;
    pub fn vkGetFenceStatus(device: VkDevice, fence: VkFence) -> VkResult;
    pub fn vkWaitForFences(device: VkDevice, fenceCount: u32, pFences: *const VkFence, waitAll: VkBool32, timeout: u64) -> VkResult;
    pub fn vkCreateSemaphore(device: VkDevice, pCreateInfo: *const VkSemaphoreCreateInfo, pAllocator: *const VkAllocationCallbacks, pSemaphore: *mut VkSemaphore) -> VkResult;
    pub fn vkDestroySemaphore(device: VkDevice, semaphore: VkSemaphore, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreateEvent(device: VkDevice, pCreateInfo: *const VkEventCreateInfo, pAllocator: *const VkAllocationCallbacks, pEvent: *mut VkEvent) -> VkResult;
    pub fn vkDestroyEvent(device: VkDevice, event: VkEvent, pAllocator: *const VkAllocationCallbacks);
    pub fn vkGetEventStatus(device: VkDevice, event: VkEvent) -> VkResult;
    pub fn vkSetEvent(device: VkDevice, event: VkEvent) -> VkResult;
    pub fn vkResetEvent(device: VkDevice, event: VkEvent) -> VkResult;
    pub fn vkCreateQueryPool(device: VkDevice, pCreateInfo: *const VkQueryPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pQueryPool: *mut VkQueryPool) -> VkResult;
    pub fn vkDestroyQueryPool(device: VkDevice, queryPool: VkQueryPool, pAllocator: *const VkAllocationCallbacks);
    pub fn vkGetQueryPoolResults(device: VkDevice, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32, dataSize: size_t, pData: *mut c_void, stride: VkDeviceSize, flags: VkQueryResultFlags) -> VkResult;
    pub fn vkCreateBuffer(device: VkDevice, pCreateInfo: *const VkBufferCreateInfo, pAllocator: *const VkAllocationCallbacks, pBuffer: *mut VkBuffer) -> VkResult;
    pub fn vkDestroyBuffer(device: VkDevice, buffer: VkBuffer, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreateBufferView(device: VkDevice, pCreateInfo: *const VkBufferViewCreateInfo, pAllocator: *const VkAllocationCallbacks, pView: *mut VkBufferView) -> VkResult;
    pub fn vkDestroyBufferView(device: VkDevice, bufferView: VkBufferView, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreateImage(device: VkDevice, pCreateInfo: *const VkImageCreateInfo, pAllocator: *const VkAllocationCallbacks, pImage: *mut VkImage) -> VkResult;
    pub fn vkDestroyImage(device: VkDevice, image: VkImage, pAllocator: *const VkAllocationCallbacks);
    pub fn vkGetImageSubresourceLayout(device: VkDevice, image: VkImage, pSubresource: *const VkImageSubresource, pLayout: *mut VkSubresourceLayout);
    pub fn vkCreateImageView(device: VkDevice, pCreateInfo: *const VkImageViewCreateInfo, pAllocator: *const VkAllocationCallbacks, pView: *mut VkImageView) -> VkResult;
    pub fn vkDestroyImageView(device: VkDevice, imageView: VkImageView, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreateShaderModule(device: VkDevice, pCreateInfo: *const VkShaderModuleCreateInfo, pAllocator: *const VkAllocationCallbacks, pShaderModule: *mut VkShaderModule) -> VkResult;
    pub fn vkDestroyShaderModule(device: VkDevice, shaderModule: VkShaderModule, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreatePipelineCache(device: VkDevice, pCreateInfo: *const VkPipelineCacheCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelineCache: *mut VkPipelineCache) -> VkResult;
    pub fn vkDestroyPipelineCache(device: VkDevice, pipelineCache: VkPipelineCache, pAllocator: *const VkAllocationCallbacks);
    pub fn vkGetPipelineCacheData(device: VkDevice, pipelineCache: VkPipelineCache, pDataSize: *mut size_t, pData: *mut c_void) -> VkResult;
    pub fn vkMergePipelineCaches(device: VkDevice, dstCache: VkPipelineCache, srcCacheCount: u32, pSrcCaches: *const VkPipelineCache) -> VkResult;
    pub fn vkCreateGraphicsPipelines(device: VkDevice, pipelineCache: VkPipelineCache, createInfoCount: u32, pCreateInfos: *const VkGraphicsPipelineCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelines: *mut VkPipeline) -> VkResult;
    pub fn vkCreateComputePipelines(device: VkDevice, pipelineCache: VkPipelineCache, createInfoCount: u32, pCreateInfos: *const VkComputePipelineCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelines: *mut VkPipeline) -> VkResult;
    pub fn vkDestroyPipeline(device: VkDevice, pipeline: VkPipeline, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreatePipelineLayout(device: VkDevice, pCreateInfo: *const VkPipelineLayoutCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelineLayout: *mut VkPipelineLayout) -> VkResult;
    pub fn vkDestroyPipelineLayout(device: VkDevice, pipelineLayout: VkPipelineLayout, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreateSampler(device: VkDevice, pCreateInfo: *const VkSamplerCreateInfo, pAllocator: *const VkAllocationCallbacks, pSampler: *mut VkSampler) -> VkResult;
    pub fn vkDestroySampler(device: VkDevice, sampler: VkSampler, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreateDescriptorSetLayout(device: VkDevice, pCreateInfo: *const VkDescriptorSetLayoutCreateInfo, pAllocator: *const VkAllocationCallbacks, pSetLayout: *mut VkDescriptorSetLayout) -> VkResult;
    pub fn vkDestroyDescriptorSetLayout(device: VkDevice, descriptorSetLayout: VkDescriptorSetLayout, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreateDescriptorPool(device: VkDevice, pCreateInfo: *const VkDescriptorPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pDescriptorPool: *mut VkDescriptorPool) -> VkResult;
    pub fn vkDestroyDescriptorPool(device: VkDevice, descriptorPool: VkDescriptorPool, pAllocator: *const VkAllocationCallbacks);
    pub fn vkResetDescriptorPool(device: VkDevice, descriptorPool: VkDescriptorPool, flags: VkDescriptorPoolResetFlags) -> VkResult;
    pub fn vkAllocateDescriptorSets(device: VkDevice, pAllocateInfo: *const VkDescriptorSetAllocateInfo, pDescriptorSets: *mut VkDescriptorSet) -> VkResult;
    pub fn vkFreeDescriptorSets(device: VkDevice, descriptorPool: VkDescriptorPool, descriptorSetCount: u32, pDescriptorSets: *const VkDescriptorSet) -> VkResult;
    pub fn vkUpdateDescriptorSets(device: VkDevice, descriptorWriteCount: u32, pDescriptorWrites: *const VkWriteDescriptorSet, descriptorCopyCount: u32, pDescriptorCopies: *const VkCopyDescriptorSet);
    pub fn vkCreateFramebuffer(device: VkDevice, pCreateInfo: *const VkFramebufferCreateInfo, pAllocator: *const VkAllocationCallbacks, pFramebuffer: *mut VkFramebuffer) -> VkResult;
    pub fn vkDestroyFramebuffer(device: VkDevice, framebuffer: VkFramebuffer, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreateRenderPass(device: VkDevice, pCreateInfo: *const VkRenderPassCreateInfo, pAllocator: *const VkAllocationCallbacks, pRenderPass: *mut VkRenderPass) -> VkResult;
    pub fn vkDestroyRenderPass(device: VkDevice, renderPass: VkRenderPass, pAllocator: *const VkAllocationCallbacks);
    pub fn vkGetRenderAreaGranularity(device: VkDevice, renderPass: VkRenderPass, pGranularity: *mut VkExtent2D);
    pub fn vkCreateCommandPool(device: VkDevice, pCreateInfo: *const VkCommandPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pCommandPool: *mut VkCommandPool) -> VkResult;
    pub fn vkDestroyCommandPool(device: VkDevice, commandPool: VkCommandPool, pAllocator: *const VkAllocationCallbacks);
    pub fn vkResetCommandPool(device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolResetFlags) -> VkResult;
    pub fn vkAllocateCommandBuffers(device: VkDevice, pAllocateInfo: *const VkCommandBufferAllocateInfo, pCommandBuffers: *mut VkCommandBuffer) -> VkResult;
    pub fn vkFreeCommandBuffers(device: VkDevice, commandPool: VkCommandPool, commandBufferCount: u32, pCommandBuffers: *const VkCommandBuffer);
    pub fn vkBeginCommandBuffer(commandBuffer: VkCommandBuffer, pBeginInfo: *const VkCommandBufferBeginInfo) -> VkResult;
    pub fn vkEndCommandBuffer(commandBuffer: VkCommandBuffer) -> VkResult;
    pub fn vkResetCommandBuffer(commandBuffer: VkCommandBuffer, flags: VkCommandBufferResetFlags) -> VkResult;

    pub fn vkCmdBindPipeline(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, pipeline: VkPipeline);
    pub fn vkCmdSetViewport(commandBuffer: VkCommandBuffer, firstViewport: u32, viewportCount: u32, pViewports: *const VkViewport);
    pub fn vkCmdSetScissor(commandBuffer: VkCommandBuffer, firstScissor: u32, scissorCount: u32, pScissors: *const VkRect2D);
    pub fn vkCmdSetLineWidth(commandBuffer: VkCommandBuffer, lineWidth: c_float);
    pub fn vkCmdSetDepthBias(commandBuffer: VkCommandBuffer, depthBiasConstantFactor: c_float, depthBiasClamp: c_float, depthBiasSlopeFactor: c_float);
    pub fn vkCmdSetBlendConstants(commandBuffer: VkCommandBuffer, blendConstants: [c_float; 4]);
    pub fn vkCmdSetDepthBounds(commandBuffer: VkCommandBuffer, minDepthBounds: c_float, maxDepthBounds: c_float);
    pub fn vkCmdSetStencilCompareMask(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, compareMask: u32);
    pub fn vkCmdSetStencilWriteMask(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, writeMask: u32);
    pub fn vkCmdSetStencilReference(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, reference: u32);
    pub fn vkCmdBindDescriptorSets(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, layout: VkPipelineLayout, firstSet: u32, descriptorSetCount: u32, pDescriptorSets: *const VkDescriptorSet, dynamicOffsetCount: u32, pDynamicOffsets: *const u32);
    pub fn vkCmdBindIndexBuffer(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, indexType: VkIndexType);
    pub fn vkCmdBindVertexBuffers(commandBuffer: VkCommandBuffer, firstBinding: u32, bindingCount: u32, pBuffers: *const VkBuffer, pOffsets: *const VkDeviceSize);
    pub fn vkCmdDraw(commandBuffer: VkCommandBuffer, vertexCount: u32, instanceCount: u32, firstVertex: u32, firstInstance: u32);
    pub fn vkCmdDrawIndexed(commandBuffer: VkCommandBuffer, indexCount: u32, instanceCount: u32, firstIndex: u32, vertexOffset: i32, firstInstance: u32);
    pub fn vkCmdDrawIndirect(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, drawCount: u32, stride: u32);
    pub fn vkCmdDrawIndexedIndirect(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, drawCount: u32, stride: u32);
    pub fn vkCmdDispatch(commandBuffer: VkCommandBuffer, groupCountX: u32, groupCountY: u32, groupCountZ: u32);
    pub fn vkCmdDispatchIndirect(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize);
    pub fn vkCmdCopyBuffer(commandBuffer: VkCommandBuffer, srcBuffer: VkBuffer, dstBuffer: VkBuffer, regionCount: u32, pRegions: *const VkBufferCopy);
    pub fn vkCmdCopyImage(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkImageCopy);
    pub fn vkCmdBlitImage(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkImageBlit, filter: VkFilter);
    pub fn vkCmdCopyBufferToImage(commandBuffer: VkCommandBuffer, srcBuffer: VkBuffer, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkBufferImageCopy);
    pub fn vkCmdCopyImageToBuffer(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstBuffer: VkBuffer, regionCount: u32, pRegions: *const VkBufferImageCopy);
    pub fn vkCmdUpdateBuffer(commandBuffer: VkCommandBuffer, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, dataSize: VkDeviceSize, pData: *const c_void);
    pub fn vkCmdFillBuffer(commandBuffer: VkCommandBuffer, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, size: VkDeviceSize, data: u32);
    pub fn vkCmdClearColorImage(commandBuffer: VkCommandBuffer, image: VkImage, imageLayout: VkImageLayout, pColor: *const VkClearColorValue, rangeCount: u32, pRanges: *const VkImageSubresourceRange);
    pub fn vkCmdClearDepthStencilImage(commandBuffer: VkCommandBuffer, image: VkImage, imageLayout: VkImageLayout, pDepthStencil: *const VkClearDepthStencilValue, rangeCount: u32, pRanges: *const VkImageSubresourceRange);
    pub fn vkCmdClearAttachments(commandBuffer: VkCommandBuffer, attachmentCount: u32, pAttachments: *const VkClearAttachment, rectCount: u32, pRects: *const VkClearRect);
    pub fn vkCmdResolveImage(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkImageResolve);
    pub fn vkCmdSetEvent(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags);
    pub fn vkCmdResetEvent(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags);
    pub fn vkCmdWaitEvents(commandBuffer: VkCommandBuffer, eventCount: u32, pEvents: *const VkEvent, srcStageMask: VkPipelineStageFlags, dstStageMask: VkPipelineStageFlags,
        memoryBarrierCount: u32, pMemoryBarriers: *const VkMemoryBarrier, bufferMemoryBarrierCount: u32, pBufferMemoryBarriers: *const VkBufferMemoryBarrier, imageMemoryBarrierCount: u32, pImageMemoryBariers: *const VkImageMemoryBarrier);
    pub fn vkCmdPipelineBarrier(commandBuffer: VkCommandBuffer, srcStageMask: VkPipelineStageFlags, dstStageMask: VkPipelineStageFlags, dependencyFlags: VkDependencyFlags,
        memoryBarrierCount: u32, pMemoryBarriers: *const VkMemoryBarrier, bufferMemoryBarrierCount: u32, pBufferMemoryBarriers: *const VkBufferMemoryBarrier, imageMemoryBarrierCount: u32, pImageMemoryBarriers: *const VkImageMemoryBarrier);
    pub fn vkCmdBeginQuery(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32, flags: VkQueryControlFlags);
    pub fn vkCmdEndQuery(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32);
    pub fn vkCmdResetQueryPool(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32);
    pub fn vkCmdWriteTimestamp(commandBuffer: VkCommandBuffer, pipelineStage: VkPipelineStageFlags, queryPool: VkQueryPool, query: u32);
    pub fn vkCmdCopyQueryPoolResults(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, stride: VkDeviceSize, flags: VkQueryResultFlags);
    pub fn vkCmdPushConstants(commandBuffer: VkCommandBuffer, layout: VkPipelineLayout, stageFlags: VkShaderStageFlags, offset: u32, size: u32, pValues: *const c_void);
    pub fn vkCmdBeginRenderPass(commandBuffer: VkCommandBuffer, pRenderPassBegin: *const VkRenderPassBeginInfo, contents: VkSubpassContents);
    pub fn vkCmdNextSubpass(commandBuffer: VkCommandBuffer, contents: VkSubpassContents);
    pub fn vkCmdEndRenderPass(commandBuffer: VkCommandBuffer);
    pub fn vkCmdExecuteCommands(commandBuffer: VkCommandBuffer, commandBufferCount: u32, pCommandBuffers: *const VkCommandBuffer);
    #[cfg(feature = "VK_KHR_push_descriptor")]
    pub fn vkCmdPushDescriptorSetKHR(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, layout: VkPipelineLayout, set: u32, descriptorWriteCount: u32, pDescriptorWrites: *const VkWriteDescriptorSet);
    #[cfg(feature = "VK_KHR_descriptor_update_template")]
    pub fn vkCmdPushDescriptorSetWithTemplateKHR(commandBuffer: VkCommandBuffer, descriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR, layout: VkPipelineLayout, set: u32, pData: *const c_void);
    #[cfg(feature = "VK_EXT_debug_marker")]
    pub fn vkCmdDebugMarkerBeginEXT(commandBuffer: VkCommandBuffer, pMarkerInfo: *const VkDebugMarkerMarkerInfoEXT);
    #[cfg(feature = "VK_EXT_debug_marker")]
    pub fn vkCmdDebugMarkerEndEXT(commandBuffer: VkCommandBuffer);
    #[cfg(feature = "VK_EXT_debug_marker")]
    pub fn vkCmdDebugMarkerInsertEXT(commandBuffer: VkCommandBuffer, pMarkerInfo: *const VkDebugMarkerMarkerInfoEXT);
    #[cfg(feature = "VK_AMD_draw_indirect_count")]
    pub fn vkCmdDrawIndirectCountAMD(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDEviceSize, countBuffer: VkBuffer, countBufferOffset: VkDeviceSize, maxDrawCount: u32, stride: u32);
    #[cfg(feature = "VK_AMD_draw_indirect_count")]
    pub fn vkCmdDrawIndexedIndirectCountAMD(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, countBuffer: VkBuffer, countBufferOffset: VkDeviceSize, maxDrawCount: u32, stride: u32);
    #[cfg(feature = "VK_KHX_device_group")]
    pub fn vkCmdSetDeviceMaskKHX(commandBuffer: VkCommandBuffer, deviceMask: u32);
    #[cfg(feature = "VK_KHX_device_group")]
    pub fn vkCmdDispatchBaseKHX(commandBuffer: VkCommandBuffer, baseGroupX: u32, baseGroupY: u32, baseGroupZ: u32, groupCountX: u32, groupCountY: u32, groupCountZ: u32);
    #[cfg(feature = "VK_NVX_device_generated_commands")]
    pub fn vkCmdProcessCommandsNVX(commandBuffer: VkCommandBuffer, pProcessCommandsInfo: *const VkCmdProcessCommandsInfoNVX);
    #[cfg(feature = "VK_NVX_device_generated_commands")]
    pub fn vkCmdReserveSpaceForCommandsNVX(commandBuffer: VkCommandBuffer, pReserveSpaceInfo: *const VkCmdReserveSpaceForCommandsInfoNVX);
    #[cfg(feature = "VK_NV_clip_space_w_scaling")]
    pub fn vkCmdSetViewportWScalingNV(commandBuffer: VkCommandBuffer, firstViewport: u32, viewportCount: u32, pViewportWScalings: *const VkViewportWScalingNV);
    #[cfg(feature = "VK_EXT_discard_rectangles")]
    pub fn vkCmdDiscardRectangleEXT(commandBuffer: VkCommandBuffer, firstDiscardRectangle: u32, discardRectangleCount: u32, pDiscardRectangles: *const VkRect2D);
}

#[cfg(feature = "VK_KHR_surface")]
extern "system"
{
    pub fn vkDestroySurfaceKHR(instance: VkInstance, surface: VkSurfaceKHR, pAllocator: *const VkAllocationCallbacks);
    pub fn vkGetPhysicalDeviceSurfaceSupportKHR(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, surface: VkSurfaceKHR, pSupported: *mut VkBool32) -> VkResult;
    pub fn vkGetPhysicalDeviceSurfaceCapabilitiesKHR(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pSurfaceCapabilities: *mut VkSurfaceCapabilitiesKHR) -> VkResult;
    pub fn vkGetPhysicalDeviceSurfaceFormatsKHR(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pSurfaceFormatCount: *mut u32, pSurfaceFormats: *mut VkSurfaceFormatKHR) -> VkResult;
    pub fn vkGetPhysicalDeviceSurfacePresentModesKHR(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pPresentModeCount: *mut u32, pPresentModes: *mut VkPresentModeKHR) -> VkResult;
}

#[cfg(feature = "VK_KHR_swapchain")]
extern "system"
{
    pub fn vkCreateSwapchainKHR(device: VkDevice, pCreateInfo: *const VkSwapchainCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSwapchain: *mut VkSwapchainKHR) -> VkResult;
    pub fn vkDestroySwapchainKHr(device: VkDevice, swapchain: VkSwapchainKHR, pAllocator: *const VkAllocationCallbacks);
    pub fn vkGetSwapchainImagesKHR(device: VkDevice, swapchain: VkSwapchainKHR, pSwapchainImageCount: *mut u32, pSwapchainImages: *mut VkImage) -> VkResult;
    pub fn vkAcquireNextImageKHR(device: VkDevice, swapchain: VkSwapchainKHR, timeout: u64, semaphore: VkSemaphore, fence: VkFence, pImageIndex: *mut u32) -> VkResult;
    pub fn vkQueuePresentKHR(queue: VkQueue, pPresentInfo: *const VkPresentInfoKHR) -> VkResult;
}

#[cfg(feature = "VK_KHR_display")]
extern "system"
{
    pub fn vkGetPhysicalDeviceDisplayPropertiesKHR(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkDisplayPropertiesKHR) -> VkResult;
    pub fn vkGetPhysicalDeviceDisplayPlanePropertiesKHR(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkDisplayPlanePropertiesKHR) -> VkResult;
    pub fn vkGetDisplayPlaneSupportedDisplaysKHR(physicalDevice: VkPhysicalDevice, planeIndex: u32, pDisplayCount: *mut u32, pDisplays: *mut VkDisplayKHR) -> VkResult;
    pub fn vkGetDisplayModePropertiesKHR(physicalDevice: VkPhysicalDevice, display: VkdisplayKHR, pPropertyCount: *mut u32, pProperties: *mut VkDisplayModePropertiesKHR) -> VkResult;
    pub fn vkCreateDisplayModeKHR(physicalDevice: VkPhysicalDevice, display: VkDisplayKHR, pCreateInfo: *const VkDisplayModeCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pMode: *mut VkDisplayModeKHR) -> VkResult;
    pub fn vkGetDisplayPlaneCapabilitiesKHR(physicalDevice: VkPhysicalDevice, mode: VkDisplayModeKHR, planeIndex: u32, pCapabilities: *mut VkDisplayPlaneCapabilitiesKHR) -> VkResult;
    pub fn vkCreateDisplayPlaneSurfaceKHR(instance: VkInstance, pCreateInfo: *const VkDisplaySurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
}

#[cfg(feature = "VK_KHR_display_swapchain")]
extern "system"
{
    pub fn vkCreateSharedSwapchainsKHR(device: VkDevice, swapchainCount: u32, pCreateInfos: *const VkSwapchainCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSwapchains: *mut VkSwapchainKHR) -> VkResult;
}

#[cfg(feature = "VK_KHR_xlib_surface")]
extern "system"
{
    pub fn vkCreateXlibSurfaceKHR(instance: VkInstance, pCreateInfo: *const VkXlibSurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
    pub fn vkGetPhysicalDeviceXlibPresentationSupportKHR(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, dpy: *mut Display, visualID: VisualID) -> VkBool32;
}

#[cfg(feature = "VK_KHR_xcb_surface")]
extern "system"
{
    pub fn vkCreateXcbSurfaceKHR(instance: VkInstance, pCreateInfo: *const VkXcbSurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
    pub fn vkGetPhysicalDeviceXcbPresentationSupportKHR(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, connection: *mut xcb_connection_t, visual_id: xcb_visualid_t) -> VkBool32;
}

#[cfg(feature = "VK_KHR_wayland_surface")]
extern "system"
{
    pub fn vkCreateWaylandSurfaceKHR(instance: VkInstance, pCreateInfo: *const VkWaylandSurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
    pub fn vkGetPhysicalDeviceWaylandPresentationSupportKHR(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, display: *mut wl_display) -> VkBool32;
}

// TODO: Mir Support

#[cfg(feature = "VK_KHR_android_surface")]
extern "system"
{
    pub fn vkCreateAndroidSurfaceKHR(instance: VkInstance, pCreateInfo: *const VkAndroidSurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
}

#[cfg(feature = "VK_KHR_win32_surface")]
extern "system"
{
    pub fn vkCreateWin32SurfaceKHR(instance: VkInstance, pCreateInfo: *const VkWin32SurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
    pub fn vkGetPhysicalDeviceWin32PresentationSupportKHR(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32) -> VkBool32;
}

#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
extern "system"
{
    pub fn vkGetPhysicalDeviceFeatures2(physicalDevice: VkPhysicalDevice, pFeatures: *mut VkPhysicalDeviceFeatures2KHR);
    pub fn vkGetPhysicalDeviceProperties2KHR(physicalDevice: VkPhysicalDevice, pProperties: *mut VkPhysicalDeviceProperties2KHR);
    pub fn vkGetPhysicalDeviceFormatProperties2KHR(physicalDevice: VkPhysicalDevice, format: VkFormat, pFormatProperties: *mut VkFormatProperties2KHR);
    pub fn vkGetPhysicalDeviceImageFormatProperties2KHR(physicalDevice: VkPhysicalDevice, pImageFormatInfo: *const VkPhysicalDeviceImageFormatInfo2KHR, pImageFormatProperties: *mut VkImageFormatProperties2KHR) -> VkResult;
    pub fn vkGetPhysicalDeviceQueueFamilyProperties2KHR(physicalDevice: VkPhysicalDevice, pQueueFamilyPropertyCount: *mut u32, pQueueFamilyProperties: *mut VkQueueFamilyProperties2KHR);
    pub fn vkGetPhysicalDeviceMemoryProperties2KHR(physicalDevice: VkPhysicalDevice, pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties2KHR);
    pub fn vkGetPhysicalDeviceSparseImageFormatProperties2KHR(physicalDevice: VkPhysicalDevice, pFormatInfo: *const VkPhysicalDeviceSparseImageFormatInfo2KHR, pPropertyCount: *mut u32, pProperties: *mut VkSparseImageFormatProperties2KHR);
}

#[cfg(feature = "VK_KHR_maintenance1")]
extern "system"
{
    pub fn vkTrimCommandPoolKHR(device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolTrimFlagsKHR);
}

#[cfg(feature = "VK_KHR_external_memory_capabilities")]
extern "system"
{
    pub fn vkGetPhysicalDeviceExternalBufferPropertiesKHR(physicalDevice: VkPhysicalDevice, pExternalBufferInfo: *const VkPhysicalDeviceExternalBufferInfoKHR, pExternalBufferProperties: *mut VkExternalBufferPropertiesKHR);
}

#[cfg(feature = "VK_KHR_external_memory_win32")]
extern "system"
{
    pub fn vkGetMemoryWin32HandleKHR(device: VkDevice, pGetWin32HandleInfo: *const VkMemoryGetWin32HandleInfoKHR, pHandle: *mut HANDLE) -> VkResult;
    pub fn vkGetMemoryWin32HandlePropertiesKHR(device: VkDevice, handleType: VkExternalMemoryHandleTypeFlagsKHR, handle: HANDLE, pMemoryWin32HandleProperties: *mut VkMemoryWin32HandleProperiesKHR) -> VkResult;
}

#[cfg(feature = "VK_KHR_external_memory_fd")]
extern "system"
{
    pub fn vkGetMemoryFdKHR(device: VkDevice, pGetFdInfo: *const VkMemoryGetFdInfoKHR, pFd: *mut c_int) -> VkResult;
    pub fn vkGetMemoryFdPropertiesKHR(device: VkDevice, handleType: VkExternalMemoryHandleTypeFlagsKHR, fd: c_int, pMemoryFdProperties: *mut VkMemoryFdPropertiesKHR) -> VkResult;
}

#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
extern "system"
{
    pub fn vkGetPhysicalDeviceExternalSemaphorePropertiesKHR(physicalDevice: VkPhysicalDevice, pExternalSemaphoreInfo: *const VkPhysicalDeviceExternalSemaphoreInfoKHR, pExternalSemaphoreProperties: *mut VkExternalSemaphorePropertiesKHR);
}

#[cfg(feature = "VK_KHR_external_semaphore_win32")]
extern "system"
{
    pub fn vkImportSemaphoreWin32HandleKHR(device: VkDevice, pImportSemaphoreWin32HandleInfo: *const VkImportSemaphoreWin32HandleInfo) -> VkResult;
    pub fn vkGetSemaphoreWin32HandleKHR(device: VkDevice, pGetWin32HandleInfo: *const VkSemaphoreGetWin32HandleInfoKHR, pHandle: *mut HANDLE) -> VkResult;
}

#[cfg(feature = "VK_KHR_external_semaphore_fd")]
extern "system"
{
    pub fn vkImportSemaphoreFdKHR(device: VkDevice, pImportSemaphoreFdInfo: *const VkImportSemaphoreFdInfoKHR) -> VkResult;
    pub fn vkGetSemaphoreFdKHR(device: VkDevice, pGetFdInfo: *const VkSemaphoreGetFdInfoKHR, pFd: *mut c_int) -> VkResult;
}

#[cfg(feature = "VK_KHR_descriptor_update_template")]
extern "system"
{
    pub fn vkCreateDescriptorUpdateTemplateKHR(device: VkDevice, pCreateInfo: *const VkDescriptorUpdateTemplateCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pDescriptorUpdateTemplate: *mut VkDescriptorUpdateTemplateKHR) -> VkResult;
    pub fn vkDestroyDescriptorUpdateTemplateKHR(device: VkDEvice, descriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR, pAllocator: *const VkAllocationCallbacks);
    pub fn vkUpdateDescriptorSetWithTemplateKHR(device: VkDevice, descriptorSet: VkDescriptorSet, descriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR, pData: *const c_void);
}

#[cfg(feature = "VK_KHR_shared_presentable_image")]
extern "system"
{
    pub fn vkGetSwapchainStatusKHR(device: VkDevice, swapchain: VkSwapchainKHR) -> VkResult;
}

#[cfg(feature = "VK_KHR_external_fence_capabilities")]
extern "system"
{
    pub fn vkGetPhysicalDeviceExternalFencePropertiesKHR(physicalDevice: VkPhysicalDevice, pExternalFenceInfo: *const VkPhysicalDeviceExternalFenceInfoKHR, pExternalFenceProperties: *mut VkExternalFencePropertiesKHR);
}

#[cfg(feature = "VK_KHR_external_fence_win32")]
extern "system"
{
    pub fn vkImportFenceWin32HandleKHR(device: VkDevice, pImportFenceWin32HandleInfo: *const VkImportFenceWin32HandleInfoKHR) -> VkResult;
    pub fn vkGetFenceWin32HandleKHR(device: VkDevice, pGetWin32HandleInfo: *const VkFenceGetWin32HandleInfoKHR, pHandle: *mut HANDLE) -> VkResult;
}

#[cfg(feature = "VK_KHR_external_fence_fd")]
extern "system"
{
    pub fn vkImportFenceFdKHR(device: VkDevice, pImportFenceFdInfo: *const VkImportFenceFdInfoKHR) -> VkResult;
    pub fn vkGetFenceFdKHR(device: VkDevice, pGetFdInfo: *const VkFenceGetFdInfoKHR, pFd: *mut c_int) -> VkResult;
}

#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
extern "system"
{
    pub fn vkGetPhysicalDeviceSurfaceCapabilities2KHR(physicalDevice: VkPhysicalDevice, pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR, pSurfaceCapabilities: *mut VkSurfaceCapabiltiies2KHR) -> VkResult;
    pub fn vkGetPhysicalDEviceSurfaceFormats2KHR(physicalDevice: VkPhysicalDevice, pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR, pSurfaceFormatCount: *mut u32, pSurfaceFormats: *mut VkSurfaceFormat2KHR) -> VkResult;
}

#[cfg(feature = "VK_KHR_get_memory_requirements2")]
extern "system"
{
    pub fn vkGetImageMemoryRequirements2KHR(device: VkDEvice, pInfo: *const VkImageMemoryRequirementsInfo2KHR, pMemoryRequirements: *mut VkMemoryRequirements2KHR);
    pub fn vkGetBufferMemoryRequirements2KHR(device: VkDevice, pInfo: *const VkBufferMemoryRequirementsInfo2KHR, pMemoryRequirements: *mut VkMemoryRequirements2KHR);
    pub fn vkGetImageSparseMemoryRequirements2KHR(device: VkDevice, pInfo: *const VkImageSparseMemoryRequirementsInfo2KHR, pSparseMemoryRequirementCount: *mut u32, pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2KHR);
}

#[cfg(feature = "VK_EXT_debug_report")]
extern "system"
{
    pub fn vkCreateDebugReportCallbackEXT(instance: VkInstance, pCreateInfo: *const VkDEbugReportCallbackCreateInfoEXT, pAllocator: *const VkAllocationCallbacks, pCallback: *mut VkDebugReportCallbackEXT) -> VkResult;
    pub fn vkDestroyDebugReportCallbackEXT(instance: VkInstance, callback: VkDebugReportCallbackEXT, pAllocator: *const VkAllocationCallbacks);
    pub fn vkDebugReportMessageEXT(instance: VkInstance, flags: VkDebugReportFlagsEXT, objectType: VkDebugReportObjectTypeEXT, object: u64, location: size_t, messageCode: i32, pLayerPrefix: *const c_char, pMessage: *const c_char);
}

#[cfg(feature = "VK_EXT_debug_marker")]
extern "system"
{
    pub fn vkDebugMarkerSetObjectTagEXT(device: VkDevice, pTagInfo: *const VkDebugMarkerObjectTagInfoEXT) -> VkResult;
    pub fn vkDebugMarkerSetObjectNameEXT(device: VkDevice, pNameInfo: *const VkDebugMarkerObjectNameInfoEXT) -> VkResult;
}

#[cfg(feature = "VK_NV_external_memory_capabilities")]
extern "system"
{
    pub fn vkGetPhysicalDeviceExternalImageFormatPropertiesNV(physicalDevice: VkPhysicalDevice, format: VkFormat, _type: VkImageType, tiling: VkImageTiling, usage: VkImageUsageFlags, flags: VkImageCreateFlags,
        externalHandleType: VkExternalMemoryHandleTypeFlagsNV, pExternalImageFormatProperties: *mut VkExternalImageFormatPropertiesNV) -> VkResult;
}

#[cfg(feature = "VK_NV_external_memory_win32")]
extern "system"
{
    pub fn vkGetMemoryWin32HandleNV(device: VkDevice, memory: VkDeviceMemory, handleType: VkExternalMemoryHandleTypeFlagsNV, pHandle: *mut HANDLE) -> VkResult;
}

#[cfg(feature = "VK_KHX_device_group")]
extern "system"
{
    pub fn vkGetDeviceGroupPeerMemoryFeaturesKHX(device: VkDevice, heapIndex: u32, localDeviceIndex: u32, remoteDeviceIndex: u32, pPeerMemoryFeatures: *mut VkPeerMemoryFeatureFlagsKHX);
    pub fn vkBindBufferMemory2KHX(device: VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindBufferMemoryInfoKHX) -> VkResult;
    pub fn vkBindImageMemory2KHX(device: VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindImageMemoryInfoKHX) -> VkResult;
    pub fn vkGetDeviceGroupPresentCapabilitiesKHX(device: VkDevice, pDeviceGroupPresentCapabilities: *mut VkDeviceGroupPresentCapabilitiesKHX) -> VkResult;
    pub fn vkGetDeviceGroupSurfacePresentModesKHX(device: VkDEvice, surface: VkSurfaceKHR, pModes: *mut VkDeviceGroupPresentModeFlagsKHX) -> VkResult;
    pub fn vkAcquireNextImage2KHX(device: VkDevice, pAcquireInfo: *const VkAcquireNextImageInfoKHX, pImageIndex: *mut u32) -> VkResult;
    pub fn vkGetPhysicalDevicePresentRectanglesKHX(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pRectCount: *mut u32, pRects: *mut VkRect2D) -> VkResult;
}

#[cfg(feature = "VK_NN_vi_surface")]
extern "system"
{
    pub fn vkCreateViSurfaceNN(instance: VkInstance, pCreateInfo: *const VkViSurfaceCreateInfoNN, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
}

#[cfg(feature = "VK_KHX_device_group_creation")]
extern "system"
{
    pub fn vkEnumeratePhysicalDeviceGroupsKHX(instance: VkInstance, pPhysicalDeviceGroupCount: *mut u32, pPhysicalDeviceGroupProperties: *mut VkPhysicalDeviceGroupPropertiesKHX) -> VkResult;
}

#[cfg(feature = "VK_NVX_device_generated_commands")]
extern "system"
{
    pub fn vkCreateIndirectCommandsLayoutNVX(device: VkDevice, pCreateInfo: *const VkIndirectCommandsLayoutCreateInfoNVX, pAllocator: *const VkAllocationCallbacks, pIndirectCommandsLayout: *mut VkIndirectCommandsLayoutNVX) -> VkResult;
    pub fn vkDestroyIndirectCommandsLayoutNVX(device: VkDevice, indirectCommandsLayout: VkIndirectCommandsLayoutNVX, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreateObjectTableNVX(device: VkDevice, pCreateInfo: *const VkObjectTableCreateInfoNVX, pAllocator: *const VkAllocationCallbacks, pObjectTable: *mut VkObjectTableNVX) -> VkResult;
    pub fn vkDestroyObjectTableNVX(device: VkDevice, objectTable: VkObjectTableNVX, pAllocator: *const VkAllocationCallbacks);
    pub fn vkRegisterObjectsNVX(device: VkDevice, objectTable: VkObjectTableNVX, objectCount: u32, ppObjectTableEntries: *const *const VkObjectTableEntryNVX, pObjectIndices: *const u32) -> VkResult;
    pub fn vkUnregisterObjectsNVX(device: VkDevice, objectTable: VkObjectTableNVX, objectCount: u32, pObjectEntryTypes: *const VkObjectEntryTypeNVX, pObjectIndices: *const u32) -> VkResult;
    pub fn vkGetPhysicalDeviceGeneratedCommandsPropertiesNVX(physicalDevice: VkPhysicalDevice, pFeatures: *mut VkDeviceGeneratedCommandsFeaturesNVX, pLimits: *mut VkDeviceGeneratedCommandsLimitsNVX);
}

#[cfg(feature = "VK_EXT_direct_mode_display")]
extern "system"
{
    pub fn vkReleaseDisplayEXT(physicalDevice: VkPhysicalDevice, display: VkDisplayKHR) -> VkResult;
}

#[cfg(feature = "VK_EXT_acquire_xlib_display")]
extern "system"
{
    pub fn vkAcquireXlibDisplayEXT(physicalDevice: VkPhysicalDevice, dpy: *mut Display, display: VkDisplayKHR) -> VkResult;
    pub fn vkGetRandROutputDisplayEXT(physicalDevice: VkPhysicalDevice, dpy: *mut Display, rrOutput: RROutput, pDisplay: *mut VkDisplayKHR) -> VkResult;
}

#[cfg(feature = "VK_EXT_display_surface_counter")]
extern "system"
{
    pub fn vkGetPhysicalDeviceSurfaceCapabilities2EXT(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pSurfaceCapabilities: *mut VkSurfaceCapabilities2EXT) -> VkResult;
}

#[cfg(feature = "VK_EXT_display_control")]
extern "system"
{
    pub fn vkDisplayPowerControlEXT(device: VkDevice, display: VkDisplayKHR, pDisplayPowerInfo: *const VkDisplayPowerInfoEXT) -> VkResult;
    pub fn vkRegisterDeviceEventEXT(device: VkDevice, pDeviceEventInfo: *const VkDeviceEventInfoEXT, pAllocator: *const VkAllocationCallbacks, pFence: *mut VkFence) -> VkResult;
    pub fn vkRegisterDisplayEventEXT(device: VkDevice, display: VkDisplayKHR, pDisplayEventInfo: *const VkDisplayEventInfoEXT, pAllocator: *const VkAllocationCallbacks, pFence: *mut VkFence) -> VkResult;
    pub fn vkGetSwapchainCounterEXT(device: VkDevice, swapchain: VkSwapchainKHR, counter: VkSurfaceCounterFlagsEXT, pCounterValue: *mut u64) -> VkResult;
}

#[cfg(feature = "VK_GOOGLE_display_timing")]
extern "system"
{
    pub fn vkGetRefreshCycleDurationGOOGLE(device: VkDevice, swapchain: VkSwapchainKHR, pDisplayTimingProperties: *mut VkRefreshCycleDurationGOOGLE) -> VkResult;
    pub fn vkGetPastPresentationTimingGOOGLE(device: VkDevice, swapchain: VkSwapchainKHR, pPresentationTimingCount: *mut u32, pPresentationTimings: *mut VkPastPresentationTimingGOOGLE) -> VkResult;
}

#[cfg(feature = "VK_EXT_hdr_metadata")]
extern "system"
{
    pub fn vkSetHdrMetadataEXT(device: VkDevice, swapchainCount: u32, pSwapchains: *const VkSwapchainKHR, pMetadata: *const VkHdrMetadataEXT);
}

#[cfg(feature = "VK_MVK_ios_surface")]
extern "system"
{
    pub fn vkCreateIOSSurfaceMVK(instance: VkInstance, pCreateInfo: *const VkIOSSurfaceCreateInfoMVK, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
}

#[cfg(feature = "VK_MVK_macos_surface")]
extern "system"
{
    pub fn vkCreateMacOSSurfaceMVK(instance: VkInstance, pCreateInfo: *const VkMacOSSurfaceCreateInfoMVK, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
}

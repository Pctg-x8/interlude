#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// Vulkan C to Rust FFI Structs and Handles

use std::os::raw::*;
use libc::size_t;
use super::*;
use xcb;

// Basic Types(Copyable) //
#[repr(C)] #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)] pub struct VkOffset2D(pub i32, pub i32);
#[repr(C)] #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)] pub struct VkExtent2D(pub u32, pub u32);
#[repr(C)] #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)] pub struct VkRect2D(pub VkOffset2D, pub VkExtent2D);
#[repr(C)] #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)] pub struct VkOffset3D(pub i32, pub i32, pub i32);
#[repr(C)] #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)] pub struct VkExtent3D(pub u32, pub u32, pub u32);
#[repr(C)] #[derive(Clone, Copy, Debug, PartialEq)] pub struct VkViewport(pub f32, pub f32, pub f32, pub f32, pub f32, pub f32);

#[repr(C)]
pub struct VkInstanceCreateInfo
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub flags: VkInstanceCreateFlags, pub pApplicationInfo: *const VkApplicationInfo,
	pub enabledLayerCount: u32, pub ppEnabledLayerNames: *const *const c_char,
	pub enabledExtensionCount: u32, pub ppEnabledExtensionNames: *const *const c_char
}
#[repr(C)]
pub struct VkApplicationInfo
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub pApplicationName: *const c_char, pub applicationVersion: u32,
	pub pEngineName: *const c_char, pub engineVersion: u32,
	pub apiVersion: u32
}
#[repr(C)]
pub struct VkAllocationCallbacks
{
	pub pUserData: *mut c_void,
	pub pfnAllocation: PFN_vkAllocationFunction,
	pub pfnReallocation: PFN_vkReallocationFunction,
	pub fnFree: PFN_vkFreeFunction,
	pub pfnInternalAllocation: PFN_vkInternalAllocationNotification,
	pub pfnInternalFree: PFN_vkInternalFreeNotification
}
#[repr(C)]
pub struct VkPhysicalDeviceFeatures
{
	pub robustBufferAccess: VkBool32,
	pub fullDrawIndexUint32: VkBool32,
	pub imageCubeArray: VkBool32,
	pub independentBlend: VkBool32,
	pub geometryShader: VkBool32,
	pub tessellationShader: VkBool32,
	pub sampleRateShading: VkBool32,
	pub dualSrcBlend: VkBool32,
	pub logicOp: VkBool32,
	pub multiDrawIndirect: VkBool32,
	pub drawIndirectFirstInstance: VkBool32,
	pub depthClamp: VkBool32,
	pub depthBiasClamp: VkBool32,
	pub fillModeNonSolid: VkBool32,
	pub depthBounds: VkBool32,
	pub wideLines: VkBool32,
	pub largePoints: VkBool32,
	pub alphaToOne: VkBool32,
	pub multiViewport: VkBool32,
	pub samplerAnisotropy: VkBool32,
	pub textureCompressionETC2: VkBool32,
	pub textureCompressionASTC_LDR: VkBool32,
	pub textureCompressionBC: VkBool32,
	pub occlusionQueryPrecise: VkBool32,
	pub pipelineStatisticsQuery: VkBool32,
	pub vertexPipelineStoresAndAtomics: VkBool32,
	pub fragmentStoresAndAtomics: VkBool32,
	pub shaderTessellationAndGeometryPointSize: VkBool32,
	pub shaderImageGatherExtended: VkBool32,
	pub shaderStorageImageExtendedFormats: VkBool32,
	pub shaderStorageImageMultisample: VkBool32,
	pub shaderStorageImageReadWithoutFormat: VkBool32,
	pub shaderStorageImageWriteWithoutFormat: VkBool32,
	pub shaderUniformBufferArrayDynamicIndexing: VkBool32,
	pub shaderSampledImageArrayDynamicIndexing: VkBool32,
	pub shaderStorageBufferArrayDynamicIndexing: VkBool32,
	pub shaderStorageImageArrayDynamicIndexing: VkBool32,
	pub shaderClipDistance: VkBool32,
	pub shaderCullDistance: VkBool32,
	pub shaderFloat64: VkBool32,
	pub shaderInt64: VkBool32,
	pub shaderInt16: VkBool32,
	pub shaderResourceResidency: VkBool32,
	pub shaderResoruceMinLod: VkBool32,
	pub sparseBinding: VkBool32,
	pub sparseResidencyBuffer: VkBool32,
	pub sparseResidencyImage2D: VkBool32,
	pub sparseResidencyImage3D: VkBool32,
	pub sparseResidency2Samples: VkBool32,
	pub sparseResidency4SAmples: VkBool32,
	pub sparseResidency8Samples: VkBool32,
	pub sparseResidency16Samples: VkBool32,
	pub sparseResidencyAliased: VkBool32,
	pub variableMultisampleRate: VkBool32,
	pub inheritedQueries: VkBool32
}
#[repr(C)] pub struct VkPhysicalDeviceLimits
{
    pub maxImageDimension1D: u32,
    pub maxImageDimension2D: u32,
    pub maxImageDimension3D: u32,
    pub maxImageDimensionCube: u32,
    pub maxImageArrayLayers: u32,
    pub maxTexelBufferElements: u32,
    pub maxUniformBufferRange: u32,
    pub maxStorageBufferRange: u32,
    pub maxPushConstantsSize: u32,
    pub maxMemoryAllocationCount: u32,
    pub maxSamplerAllocationCount: u32,
    pub bufferImageGranularity: VkDeviceSize,
    pub sparseAddressSpaceSize: VkDeviceSize,
    pub maxBoundDescriptorSets: u32,
    pub maxPerStageDescriptorSamplers: u32,
    pub maxPerStageDescriptorUniformBuffers: u32,
    pub maxPerStageDescriptorStorageBuffers: u32,
    pub maxPerStageDescriptorSampledImages: u32,
    pub maxPerStageDescriptorStorageImages: u32,
    pub maxPerStageDescriptorInputAttachments: u32,
    pub maxPerStageResources: u32,
    pub maxDescriptorSetSamplers: u32,
    pub maxDescriptorSetUniformBuffers: u32,
    pub maxDescriptorSetUniformBuffersDynamic: u32,
    pub maxDescriptorSetStorageBuffers: u32,
    pub maxDescriptorSetStorageBuffersDynamic: u32,
    pub maxDescriptorSetSampledImages: u32,
    pub maxDescriptorSetStorageImages: u32,
    pub maxDescriptorSetInputAttachments: u32,
    pub maxVertexInputAttributes: u32,
    pub maxVertexInputBindings: u32,
    pub maxVertexInputAttributeOffset: u32,
    pub maxVertexInputBindingStride: u32,
    pub maxVertexOutputComponents: u32,
    pub maxTessellationGenerationLevel: u32,
    pub maxTessellationPatchSize: u32,
    pub maxTessellationControlPerVertexInputComponents: u32,
    pub maxTessellationControlPerVertexOutputComponents: u32,
    pub maxTessellationControlPerPatchOutputComponents: u32,
    pub maxTessellationControlTotalOutputComponents: u32,
    pub maxTessellationEvaluationInputComponents: u32,
    pub maxTessellationEvaluationOutputComponents: u32,
    pub maxGeometryShaderInvocations: u32,
    pub maxGeometryInputComponents: u32,
    pub maxGeometryOutputComponents: u32,
    pub maxGeometryOutputVertices: u32,
    pub maxGeometryTotalOutputComponents: u32,
    pub maxFragmentInputComponents: u32,
    pub maxFragmentOutputAttachments: u32,
    pub maxFragmentDualSrcAttachments: u32,
    pub maxFragmentCombinedOutputResources: u32,
    pub maxComputeSharedMemorySize: u32,
    pub maxComputeWorkGroupCount: [u32; 3],
    pub maxComputeWorkGroupInvocations: u32,
    pub maxComputeWorkGroupSize: [u32; 3],
    pub subPixelPrecisionBits: u32,
    pub subTexelPrecisionBits: u32,
    pub mipmapPrecisionBits: u32,
    pub maxDrawIndexedIndexValue: u32,
    pub maxDrawIndirectCount: u32,
    pub maxSamplerLodBias: f32,
    pub maxSamplerAnisotropy: f32,
    pub maxViewports: u32,
    pub maxViewportDimensions: [u32; 2],
    pub viewportBoundsRange: [f32; 2],
    pub viewportSubPixelBits: u32,
    pub minMemoryMapAlignment: size_t,
    pub minTexelBufferOffsetAlignment: VkDeviceSize,
    pub minUniformBufferOffsetAlignment: VkDeviceSize,
    pub minStorageBufferOffsetAlignment: VkDeviceSize,
    pub minTexelOffset: i32,
    pub maxTexelOffset: u32,
    pub minTexelGatherOffset: i32,
    pub maxTexelGatherOffset: u32,
    pub minInterpolationOffset: f32,
    pub maxInterpolationOffset: f32,
    pub subPixelInterpolationOffsetBits: u32,
    pub maxFramebufferWidth: u32,
    pub maxFramebufferHeight: u32,
    pub maxFramebufferLayers: u32,
    pub framebufferColorSampleCounts: VkSampleCountFlags,
    pub framebufferDepthSampleCounts: VkSampleCountFlags,
    pub framebufferStencilSampleCounts: VkSampleCountFlags,
    pub framebufferNoAttachmentsSampleCounts: VkSampleCountFlags,
    pub maxColorAttachments: u32,
    pub sampledImageColorSampleCounts: VkSampleCountFlags,
    pub sampledImageIntegerSampleCounts: VkSampleCountFlags,
    pub sampledImageDepthSampleCounts: VkSampleCountFlags,
    pub sampledImageStencilSampleCounts: VkSampleCountFlags,
    pub storageImageSampleCounts: VkSampleCountFlags,
    pub maxSampleMaskWords: u32,
    pub timestampComputeAndGraphics: VkBool32,
    pub timestampPeriod: f32,
    pub maxClipDistances: u32,
    pub maxCullDistances: u32,
    pub maxCombinedClipAndCullDistances: u32,
    pub discreteQueuePriorities: u32,
    pub pointSizeRange: [f32; 2],
    pub lineWidthRange: [f32; 2],
    pub pointSizeGranularity: f32,
    pub lineWidthGranularity: f32,
    pub strictLines: VkBool32,
    pub standardSampleLocations: VkBool32,
    pub optimalBufferCopyOffsetAlignment: VkDeviceSize,
    pub optimalBufferCopyRowPitchAlignment: VkDeviceSize,
    pub nonCoherentAtomSize: VkDeviceSize
}
#[repr(C)] pub struct VkPhysicalDeviceSparseProperties
{
	pub residencyStandard2DBlockShape: VkBool32,
	pub residencyStandard2DMultisampleBlockShape: VkBool32,
	pub residencyStandard3DBlockShape: VkBool32,
	pub residencyAlignedMipSize: VkBool32,
	pub residencyNonResidentStrict: VkBool32
}
#[repr(C)] pub struct VkPhysicalDeviceProperties
{
	pub apiVersion: u32, pub driverVersion: u32, pub vendorID: u32, pub deviceID: u32,
	pub deviceType: VkPhysicalDeviceType, pub deviceName: [c_char; VK_MAX_PHYSICAL_DEVICE_NAME_SIZE],
	pub pipelineCacheUUID: [u8; VK_UUID_SIZE], pub limits: VkPhysicalDeviceLimits,
	pub sparseProperties: VkPhysicalDeviceSparseProperties
}
#[repr(C)] #[derive(Clone)] pub struct VkQueueFamilyProperties
{
	pub queueFlags: VkQueueFlags, pub queueCount: u32, pub timestampValidBits: u32,
	pub minImageTransferGranularity: VkExtent3D
}
#[repr(C)] pub struct VkPhysicalDeviceMemoryProperties
{
	pub memoryTypeCount: u32, pub memoryTypes: [VkMemoryType; VK_MAX_MEMORY_TYPES],
	pub memoryHeapCount: u32, pub memoryHeaps: [VkMemoryHeap; VK_MAX_MEMORY_HEAPS]
}
#[repr(C)] pub struct VkMemoryType(pub VkMemoryPropertyFlags, pub u32);
#[repr(C)] pub struct VkMemoryHeap(pub VkDeviceSize, pub VkMemoryHeapFlags);
#[repr(C)]
pub struct VkDeviceQueueCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkDeviceQueueCreateFlags,
	pub queueFamilyIndex: u32,
	pub queueCount: u32,pub pQueuePriorities: *const f32
}
#[repr(C)]
pub struct VkDeviceCreateInfo
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkDeviceCreateFlags,
	pub queueCreateInfoCount: u32,
	pub pQueueCreateInfos: *const VkDeviceQueueCreateInfo,
	pub enabledLayerCount: u32,
	pub ppEnabledLayerNames: *const *const c_char,
	pub enabledExtensionCount: u32,
	pub ppEnabledExtensionNames: *const *const c_char,
	pub pEnabledFeatures: *const VkPhysicalDeviceFeatures
}
#[repr(C)] pub struct VkImageViewCreateInfo
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub flags: VkImageViewCreateFlags, pub image: VkImage,
	pub viewType: VkImageViewType, pub format: VkFormat,
	pub components: VkComponentMapping, pub subresourceRange: VkImageSubresourceRange
}
#[repr(C)] pub struct VkComponentMapping
{
	pub r: VkComponentSwizzle, pub g: VkComponentSwizzle, pub b: VkComponentSwizzle, pub a: VkComponentSwizzle
}
impl VkComponentMapping
{
	pub fn default() -> VkComponentMapping
	{
		VkComponentMapping { r: VkComponentSwizzle::R, g: VkComponentSwizzle::G, b: VkComponentSwizzle::B, a: VkComponentSwizzle::A }
	}
}
#[repr(C)] #[derive(Clone, Copy)] pub struct VkImageSubresourceRange
{
	pub aspectMask: VkImageAspectFlags,
	pub baseMipLevel: u32, pub levelCount: u32,
	pub baseArrayLayer: u32, pub layerCount: u32
}
#[repr(C)] pub struct VkAttachmentDescription
{
	pub flags: VkAttachmentDescriptionFlags, pub format: VkFormat,
	pub samples: VkSampleCountFlagBits,
	pub loadOp: VkAttachmentLoadOp, pub storeOp: VkAttachmentStoreOp,
	pub stencilLoadOp: VkAttachmentLoadOp, pub stencilStoreOp: VkAttachmentStoreOp,
	pub initialLayout: VkImageLayout, pub finalLayout: VkImageLayout
}
#[repr(C)] #[derive(Clone, Copy)] pub struct VkAttachmentReference(pub u32, pub VkImageLayout);		// attachment, layout
#[repr(C)] pub struct VkSubpassDescription
{
	pub flags: VkSubpassDescriptionFlags, pub pipelineBindPoint: VkPipelineBindPoint,
	pub inputAttachmentCount: u32, pub pInputAttachments: *const VkAttachmentReference,
	pub colorAttachmentCount: u32, pub pColorAttachments: *const VkAttachmentReference,
	pub pResolveAttachments: *const VkAttachmentReference, pub pDepthStencilAttachment: *const VkAttachmentReference,
	pub preserveAttachmentCount: u32, pub pPreserveAttachments: *const u32
}
#[repr(C)] pub struct VkSubpassDependency
{
	pub srcSubpass: u32, pub dstSubpass: u32,
	pub srcStageMask: VkPipelineStageFlags, pub dstStageMask: VkPipelineStageFlags,
	pub srcAccessMask: VkAccessFlags, pub dstAccessMask: VkAccessFlags,
	pub dependencyFlags: VkDependencyFlags
}
#[repr(C)] pub struct VkRenderPassCreateInfo
{
	pub sType: VkStructureType, pub pNext: *const c_void, pub flags: VkRenderPassCreateFlags,
	pub attachmentCount: u32, pub pAttachments: *const VkAttachmentDescription,
	pub subpassCount: u32, pub pSubpasses: *const VkSubpassDescription,
	pub dependencyCount: u32, pub pDependencies: *const VkSubpassDependency
}
#[repr(C)] pub struct VkFramebufferCreateInfo
{
	pub sType: VkStructureType, pub pNext: *const c_void, pub flags: VkFramebufferCreateFlags,
	pub renderPass: VkRenderPass, pub attachmentCount: u32, pub pAttachments: *const VkImageView,
	pub width: u32, pub height: u32, pub layers: u32
}
#[repr(C)] pub struct VkCommandPoolCreateInfo
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub flags: VkCommandPoolCreateFlags, pub queueFamilyIndex: u32
}
#[repr(C)] pub struct VkCommandBufferAllocateInfo
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub commandPool: VkCommandPool, pub level: VkCommandBufferLevel,
	pub commandBufferCount: u32
}
#[repr(C)] pub struct VkCommandBufferInheritanceInfo
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub renderPass: VkRenderPass, pub subpass: u32,
	pub framebuffer: VkFramebuffer, pub occlusionQueryEnable: VkBool32,
	pub queryFlags: VkQueryControlFlags, pub pipelineStatistics: VkQueryPipelineStatisticFlags
}
#[repr(C)] pub struct VkCommandBufferBeginInfo
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub flags: VkCommandBufferUsageFlags, pub pInheritanceInfo: *const VkCommandBufferInheritanceInfo
}
#[repr(C)] pub struct VkShaderModuleCreateInfo
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub flags: VkShaderModuleCreateFlags,
	pub codeSize: size_t, pub pCode: *const u32
}
#[repr(C)] pub struct VkPipelineCacheCreateInfo
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub flags: VkPipelineCacheCreateFlags,
	pub initialDataSize: size_t, pub pInitialData: *const c_void
}
#[repr(C)] pub struct VkSpecializationMapEntry(pub u32, pub u32, pub size_t);		// id, offset, size
#[repr(C)] pub struct VkSpecializationInfo
{
	pub mapEntryCount: u32, pub pMapEntries: *const VkSpecializationMapEntry,
	pub dataSize: size_t, pub pData: *const c_void
}
#[repr(C)] pub struct VkPipelineShaderStageCreateInfo
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub flags: VkPipelineShaderStageCreateFlags,
	pub stage: VkShaderStageFlagBits,
	pub module: VkShaderModule, pub pName: *const c_char,
	pub pSpecializationInfo: *const VkSpecializationInfo
}
/// Binding, Stride, InputRate
#[repr(C)] pub struct VkVertexInputBindingDescription(pub u32, pub u32, pub VkVertexInputRate);
/// Location, Binding, Format, Offset
#[repr(C)] pub struct VkVertexInputAttributeDescription(pub u32, pub u32, pub VkFormat, pub u32);
#[repr(C)] pub struct VkPipelineVertexInputStateCreateInfo
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub flags: VkPipelineVertexInputStateCreateFlags,
	pub vertexBindingDescriptionCount: u32,
	pub pVertexBindingDescriptions: *const VkVertexInputBindingDescription,
	pub vertexAttributeDescriptionCount: u32,
	pub pVertexAttributeDescriptions: *const VkVertexInputAttributeDescription
}
#[repr(C)] pub struct VkPipelineInputAssemblyStateCreateInfo
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub flags: VkPipelineInputAssemblyStateCreateFlags,
	pub topology: VkPrimitiveTopology,
	pub primitiveRestartEnable: VkBool32
}
#[repr(C)] pub struct VkPipelineTessellationStateCreateInfo
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub flags: VkPipelineTessellationStateCreateFlags,
	pub patchControlPoints: u32
}
#[repr(C)] pub struct VkPipelineViewportStateCreateInfo
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub flags: VkPipelineViewportStateCreateFlags,
	pub viewportCount: u32, pub pViewports: *const VkViewport,
	pub scissorCount: u32, pub pScissors: *const VkRect2D
}
#[repr(C)] pub struct VkPipelineRasterizationStateCreateInfo
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub flags: VkPipelineRasterizationStateCreateFlags,
	pub depthClampEnable: VkBool32, pub rasterizerDiscardEnable: VkBool32,
	pub polygonMode: VkPolygonMode, pub cullMode: VkCullModeFlags,
	pub frontFace: VkFrontFace, pub depthBiasEnable: VkBool32,
	pub depthBiasConstantFactor: f32, pub depthBiasClamp: f32,
	pub depthBiasSlopeFactor: f32, pub lineWidth: f32
}
#[repr(C)] pub struct VkPipelineMultisampleStateCreateInfo
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub flags: VkPipelineMultisampleStateCreateFlags,
	pub rasterizationSamples: VkSampleCountFlagBits,
	pub sampleShadingEnable: VkBool32,
	pub minSampleShading: f32, pub pSampleMask: *const VkSampleMask,
	pub alphaToCoverageEnable: VkBool32,
	pub alphaToOneEnable: VkBool32
}
#[repr(C)] pub struct VkStencilOpState
{
	pub failOp: VkStencilOp, pub passOp: VkStencilOp,
	pub depthFailOp: VkStencilOp, pub compareop: VkCompareOp,
	pub compareMask: u32, pub writeMask: u32, pub reference: u32
}
#[repr(C)] pub struct VkPipelineDepthStencilStateCreateInfo
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub flags: VkPipelineDepthStencilStateCreateFlags,
	pub depthTestEnable: VkBool32, pub depthWriteEnable: VkBool32,
	pub depthCompareOp: VkCompareOp, pub depthBoundsTestEnable: VkBool32,
	pub stencilTestEnable: VkBool32, pub front: VkStencilOpState, pub back: VkStencilOpState,
	pub minDepthBounds: f32, pub maxDepthBounds: f32
}
#[repr(C)] pub struct VkPipelineColorBlendAttachmentState
{
	pub blendEnable: VkBool32,
	pub srcColorBlendFactor: VkBlendFactor, pub dstColorBlendFactor: VkBlendFactor,
	pub colorBlendOp: VkBlendOp,
	pub srcAlphaBlendFactor: VkBlendFactor, pub dstAlphaBlendFactor: VkBlendFactor,
	pub alphaBlendOp: VkBlendOp, pub colorWriteMask: VkColorComponentFlags
}
#[repr(C)] pub struct VkPipelineColorBlendStateCreateInfo
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub flags: VkPipelineColorBlendStateCreateFlags,
	pub logicOpEnable: VkBool32, pub logicOp: VkLogicOp,
	pub attachmentCount: u32,
	pub pAttachments: *const VkPipelineColorBlendAttachmentState,
	pub blendConstants: [f32; 4]
}
#[repr(C)] pub struct VkPipelineDynamicStateCreateInfo
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub flags: VkPipelineDynamicStateCreateFlags,
	pub dynamicStateCount: u32,
	pub pDynamicStates: *const VkDynamicState
}
#[repr(C)] #[derive(Clone, Copy)] pub struct VkGraphicsPipelineCreateInfo
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub flags: VkPipelineCreateFlags,
	pub stageCount: u32,
	pub pStages: *const VkPipelineShaderStageCreateInfo,
	pub pVertexInputState: *const VkPipelineVertexInputStateCreateInfo,
	pub pInputAssemblyState: *const VkPipelineInputAssemblyStateCreateInfo,
	pub pTessellationState: *const VkPipelineTessellationStateCreateInfo,
	pub pViewportState: *const VkPipelineViewportStateCreateInfo,
	pub pRasterizationState: *const VkPipelineRasterizationStateCreateInfo,
	pub pMultisampleState: *const VkPipelineMultisampleStateCreateInfo,
	pub pDepthStencilState: *const VkPipelineDepthStencilStateCreateInfo,
	pub pColorBlendState: *const VkPipelineColorBlendStateCreateInfo,
	pub pDynamicState: *const VkPipelineDynamicStateCreateInfo,
	pub layout: VkPipelineLayout, pub renderPass: VkRenderPass, pub subpass: u32,
	pub basePipelineHandle: VkPipeline, pub basePipelineIndex: u32
}
#[repr(C)] pub struct VkPushConstantRange(pub VkShaderStageFlags, pub u32, pub u32);
#[repr(C)] pub struct VkPipelineLayoutCreateInfo
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub flags: VkPipelineLayoutCreateFlags,
	pub setLayoutCount: u32, pub pSetLayouts: *const VkDescriptorSetLayout,
	pub pushConstantRangeCount: u32, pub pPushConstantRanges: *const VkPushConstantRange
}
#[repr(C)] #[derive(Clone)] pub struct VkMemoryBarrier
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub srcAccessMask: VkAccessFlags, pub dstAccessMask: VkAccessFlags
}
#[repr(C)] #[derive(Clone)] pub struct VkBufferMemoryBarrier
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub srcAccessMask: VkAccessFlags, pub dstAccessMask: VkAccessFlags,
	pub srcQueueFamilyIndex: u32, pub dstQueueFamilyIndex: u32,
	pub buffer: VkBuffer, pub offset: VkDeviceSize, pub size: VkDeviceSize
}
#[repr(C)] #[derive(Clone)] pub struct VkImageMemoryBarrier
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub srcAccessMask: VkAccessFlags, pub dstAccessMask: VkAccessFlags,
	pub oldLayout: VkImageLayout, pub newLayout: VkImageLayout,
	pub srcQueueFamilyIndex: u32, pub dstQueueFamilyIndex: u32,
	pub image: VkImage, pub subresourceRange: VkImageSubresourceRange
}
#[repr(C)] pub struct VkRenderPassBeginInfo
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub renderPass: VkRenderPass, pub framebuffer: VkFramebuffer,
	pub renderArea: VkRect2D, pub clearValueCount: u32,
	pub pClearValues: *const VkClearValue
}
#[repr(C)] #[derive(Clone, Copy)] pub struct VkClearColorValue(pub f32, pub f32, pub f32, pub f32);
#[repr(C)] #[derive(Clone, Copy)] pub struct VkClearDepthStencilValue(pub f32, pub u32);
#[repr(C)] #[derive(Clone, Copy)] pub struct VkClearValue(pub VkClearColorValue);

#[repr(C)] pub struct VkFenceCreateInfo
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub flags: VkFenceCreateFlags
}
#[repr(C)] pub struct VkSemaphoreCreateInfo
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub flags: VkSemaphoreCreateFlags
}
#[repr(C)] pub struct VkSubmitInfo
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub waitSemaphoreCount: u32, pub pWaitSemaphores: *const VkSemaphore, pub pWaitDstStageMask: *const VkPipelineStageFlags,
	pub commandBufferCount: u32, pub pCommandBuffers: *const VkCommandBuffer,
	pub signalSemaphoreCount: u32, pub pSignalSemaphores: *const VkSemaphore
}
#[repr(C)] pub struct VkMemoryAllocateInfo
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub allocationSize: VkDeviceSize, pub memoryTypeIndex: u32
}
#[repr(C)] pub struct VkMappedMemoryRange
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub memory: VkDeviceMemory, pub offset: VkDeviceSize,
	pub size: VkDeviceSize
}
#[repr(C)] #[derive(Copy, Clone)] pub struct VkMemoryRequirements
{
	pub size: VkDeviceSize, pub alignment: VkDeviceSize,
	pub memoryTypeBits: u32
}

#[repr(C)] pub struct VkBufferCreateInfo
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub flags: VkBufferCreateFlags, pub size: VkDeviceSize,
	pub usage: VkBufferUsageFlags, pub sharingMode: VkSharingMode,
	pub queueFamilyIndexCount: u32, pub pQueueFamilyIndices: *const u32
}
#[repr(C)] #[derive(Clone, Copy)] pub struct VkImageCreateInfo
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub flags: VkImageCreateFlags, pub imageType: VkImageType,
	pub format: VkFormat, pub extent: VkExtent3D,
	pub mipLevels: u32, pub arrayLayers: u32,
	pub samples: VkSampleCountFlagBits, pub tiling: VkImageTiling,
	pub usage: VkImageUsageFlags, pub sharingMode: VkSharingMode,
	pub queueFamilyIndexCount: u32, pub pQueueFamilyIndices: *const u32,
	pub initialLayout: VkImageLayout
}

#[repr(C)] pub struct VkDescriptorSetLayoutBinding
{
	pub binding: u32, pub descriptorType: VkDescriptorType, pub descriptorCount: u32,
	pub stageFlags: VkShaderStageFlags, pub pImmutableSamplers: *const VkSampler
}
#[repr(C)] pub struct VkDescriptorSetLayoutCreateInfo
{
	pub sType: VkStructureType,pub pNext: *const c_void,
	pub flags: VkDescriptorSetLayoutCreateFlags,
	pub bindingCount: u32, pub pBindings: *const VkDescriptorSetLayoutBinding
}
#[repr(C)] pub struct VkDescriptorPoolSize(pub VkDescriptorType, pub u32);						// type, descriptorCount
#[repr(C)] pub struct VkDescriptorPoolCreateInfo
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub flags: VkDescriptorPoolCreateFlags,
	pub maxSets: u32, pub poolSizeCount: u32, pub pPoolSizes: *const VkDescriptorPoolSize
}
#[repr(C)] pub struct VkDescriptorSetAllocateInfo
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub descriptorPool: VkDescriptorPool, pub descriptorSetCount: u32, pub pSetLayouts: *const VkDescriptorSetLayout
}
#[repr(C)] pub struct VkDescriptorImageInfo(pub VkSampler, pub VkImageView, pub VkImageLayout);			// sampler, view, layout
#[repr(C)] pub struct VkDescriptorBufferInfo(pub VkBuffer, pub VkDeviceSize, pub VkDeviceSize);			// buffer, offset, range
#[repr(C)] #[derive(Clone, Copy)] pub struct VkWriteDescriptorSet
{
	pub sType: VkStructureType, pub pNext: *const c_void, pub dstSet: VkDescriptorSet,
	pub dstBinding: u32, pub dstArrayElement: u32, pub descriptorCount: u32, pub descriptorType: VkDescriptorType,
	pub pImageInfo: *const VkDescriptorImageInfo, pub pBufferInfo: *const VkDescriptorBufferInfo, pub pTexelBufferView: *const VkBufferView
}
#[repr(C)] pub struct VkCopyDescriptorSet
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub srcSet: VkDescriptorSet, pub srcBinding: u32, pub srcArrayElement: u32,
	pub dstSet: VkDescriptorSet, pub dstBinding: u32, pub dstArrayElement: u32,
	pub descriptorCount: u32
}
#[repr(C)] pub struct VkImageSubresourceLayers(pub VkImageAspectFlags, pub u32, pub u32, pub u32);		// aspect_mask, mip_level, base_array_layer, layer_count
#[repr(C)] pub struct VkBufferCopy(pub VkDeviceSize, pub VkDeviceSize, pub VkDeviceSize);		// src_offset, dst_offset, size
#[repr(C)] pub struct VkImageCopy(pub VkImageSubresourceLayers, pub VkOffset3D, pub VkImageSubresourceLayers, pub VkOffset3D, pub VkExtent3D);		// src_subresource, src_offset, dst_subresource, dst_offset, extent
#[repr(C)] pub struct VkImageBlit
{
	pub srcSubresource: VkImageSubresourceLayers, pub srcOffsets: [VkOffset3D; 2],
	pub dstSubresource: VkImageSubresourceLayers, pub dstOffsets: [VkOffset3D; 2]
}
#[repr(C)] pub struct VkSamplerCreateInfo
{
	pub sType: VkStructureType, pub pNext: *const c_void, pub flags: VkSamplerCreateFlags,
	pub magFilter: VkFilter, pub minFilter: VkFilter, pub mipmapMode: VkSamplerMipmapMode,
	pub addressModeU: VkSamplerAddressMode, pub addressModeV: VkSamplerAddressMode, pub addressModeW: VkSamplerAddressMode,
	pub mipLodBias: f32, pub anisotropyEnable: VkBool32, pub maxAnisotropy: f32,
	pub compareEnable: VkBool32, pub compareOp: VkCompareOp,
	pub minLod: f32, pub maxLod: f32, pub borderColor: VkBorderColor,
	pub unnormalizedCoordinates: VkBool32
}
#[repr(C)] pub struct VkDrawIndirectCommand(pub u32, pub u32, pub u32, pub u32);		// vcount, icount, firstV, firstI
#[repr(C)] pub struct VkDrawIndexedIndirectCommand(pub u32, pub u32, pub u32, pub i32, pub u32);	// idxcount, icount, firstIdx, voffs, firstI

// Surface/Swapchain Extensions //
#[repr(C)]
pub struct VkXcbSurfaceCreateInfoKHR
{
	pub sType: VkStructureType, pub pNext: *const c_void, pub flags: VkXlibSurfaceCreateFlagsKHR,
	pub connection: *mut xcb::ffi::xcb_connection_t, pub window: xcb::ffi::xcb_window_t
}
#[repr(C)]
pub struct VkSurfaceCapabilitiesKHR
{
	pub minImageCount: u32,
	pub maxImageCount: u32,
	pub currentExtent: VkExtent2D,
	pub minImageExtent: VkExtent2D,
	pub maxImageExtent: VkExtent2D,
	pub maxImageArrayLayers: u32,
	pub supportedTransforms: VkSurfaceTransformFlagsKHR,
	pub currentTransform: VkSurfaceTransformFlagsKHR,
	pub supportedCompositeAlpha: VkCompositeAlphaFlagsKHR,
	pub supportedUsageFlags: VkImageUsageFlags
}
#[repr(C)] #[derive(Clone)]
pub struct VkSurfaceFormatKHR
{
	pub format: VkFormat,
	pub colorSpace: VkColorSpaceKHR
}

#[repr(C)]
pub struct VkSwapchainCreateInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub flags: VkSwapchainCreateFlagsKHR,
	pub surface: VkSurfaceKHR,
	pub minImageCount: u32,
	pub imageFormat: VkFormat,
	pub imageColorSpace: VkColorSpaceKHR,
	pub imageExtent: VkExtent2D,
	pub imageArrayLayers: u32,
	pub imageUsage: VkImageUsageFlags,
	pub imageSharingMode: VkSharingMode,
	pub queueFamilyIndexCount: u32,
	pub pQueueFamilyIndices: *const u32,
	pub preTransform: VkSurfaceTransformFlagsKHR,
	pub compositeAlpha: VkCompositeAlphaFlagsKHR,
	pub presentMode: VkPresentModeKHR,
	pub clipped: VkBool32,
	pub oldSwapchain: VkSwapchainKHR
}
#[repr(C)]
pub struct VkPresentInfoKHR
{
	pub sType: VkStructureType,
	pub pNext: *const c_void,
	pub waitSemaphoreCount: u32,
	pub pWaitSemaphores: *const VkSemaphore,
	pub swapchainCount: u32,
	pub pSwapchains: *const VkSwapchainKHR,
	pub pImageIndices: *const u32,
	pub pResults: *mut VkResult
}

#[repr(C)] pub struct VkDebugReportCallbackCreateInfoEXT
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub flags: VkDebugReportFlagsEXT, pub pfnCallback: PFN_vkDebugReportCallbackEXT,
	pub pUserData: *mut c_void
}

use std;
// Structure Conversions
impl std::convert::From<VkExtent3D> for VkExtent2D
{
	fn from(v: VkExtent3D) -> Self { VkExtent2D(v.0, v.1) }
}
impl std::convert::From<VkExtent3D> for u32
{
	fn from(v: VkExtent3D) -> Self { v.0 }
}
impl std::convert::From<VkExtent2D> for u32
{
	fn from(v: VkExtent2D) -> Self { v.0 }
}

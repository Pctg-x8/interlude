use super::ffi::*;
use std;

impl std::default::Default for VkApplicationInfo
{
	fn default() -> Self
	{
		VkApplicationInfo
		{
			sType: VkStructureType::ApplicationInfo, pNext: std::ptr::null(),
			apiVersion: VK_API_VERSION_1_0,
			pApplicationName: std::ptr::null(), pEngineName: std::ptr::null(),
			applicationVersion: 0, engineVersion: 0
		}
	}
}
impl std::default::Default for VkInstanceCreateInfo
{
	fn default() -> Self
	{
		VkInstanceCreateInfo
		{
			sType: VkStructureType::InstanceCreateInfo, pNext: std::ptr::null(), flags: 0,
			pApplicationInfo: std::ptr::null(),
			enabledLayerCount: 0, enabledExtensionCount: 0,
			ppEnabledLayerNames: std::ptr::null(), ppEnabledExtensionNames: std::ptr::null()
		}
	}
}
impl std::default::Default for VkPhysicalDeviceFeatures
{
	fn default() -> Self
	{
		VkPhysicalDeviceFeatures
		{
			robustBufferAccess: 0,
			fullDrawIndexUint32: 0,
			imageCubeArray: 0,
			independentBlend: 0,
			geometryShader: 0,
			tessellationShader: 0,
			sampleRateShading: 0,
			dualSrcBlend: 0,
			logicOp: 0,
			multiDrawIndirect: 0,
			drawIndirectFirstInstance: 0,
			depthClamp: 0,
			depthBiasClamp: 0,
			fillModeNonSolid: 0,
			depthBounds: 0,
			wideLines: 0,
			largePoints: 0,
			alphaToOne: 0,
			multiViewport: 0,
			samplerAnisotropy: 0,
			textureCompressionETC2: 0,
			textureCompressionASTC_LDR: 0,
			textureCompressionBC: 0,
			occlusionQueryPrecise: 0,
			pipelineStatisticsQuery: 0,
			vertexPipelineStoresAndAtomics: 0,
			fragmentStoresAndAtomics: 0,
			shaderTessellationAndGeometryPointSize: 0,
			shaderImageGatherExtended: 0,
			shaderStorageImageExtendedFormats: 0,
			shaderStorageImageMultisample: 0,
			shaderStorageImageReadWithoutFormat: 0,
			shaderStorageImageWriteWithoutFormat: 0,
			shaderUniformBufferArrayDynamicIndexing: 0,
			shaderSampledImageArrayDynamicIndexing: 0,
			shaderStorageBufferArrayDynamicIndexing: 0,
			shaderStorageImageArrayDynamicIndexing: 0,
			shaderClipDistance: 0, shaderCullDistance: 0,
			shaderFloat64: 0, shaderInt64: 0, shaderInt16: 0,
			shaderResourceResidency: 0, shaderResoruceMinLod: 0,
			sparseBinding: 0,
			sparseResidencyBuffer: 0, sparseResidencyImage2D: 0, sparseResidencyImage3D: 0,
			sparseResidency2Samples: 0, sparseResidency4SAmples: 0, sparseResidency8Samples: 0, sparseResidency16Samples: 0,
			sparseResidencyAliased: 0,
			variableMultisampleRate: 0,
			inheritedQueries: 0
		}
	}
}

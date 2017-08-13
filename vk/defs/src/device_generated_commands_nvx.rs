//! VK_NVX_device_generated_commands extensions

pub const VK_NVX_DEVICE_GENERATED_COMMANDS_SPEC_VERSION: usize = 1;
pub static VK_NVX_DEVICE_GENERATED_COMMANDS_EXTENSION_NAME: &'static str = "VK_NVX_device_generated_commands";

use libc::*;
use super::*;

mod nd_handle_base_ts { pub enum VkObjectTableNVX {} pub enum VkIndirectCommandsLayoutNVX {} }
pub type VkObjectTableNVX = VK_NON_DISPATCHABLE_HANDLE!(VkObjectTableNVX);
pub type VkIndirectCommandsLayoutNVX = VK_NON_DISPATCHABLE_HANDLE!(VkIndirectCommandsLayoutNVX);

pub type VkIndirectCommandsTokenTypeNVX = i32;
pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_PIPELINE_NVX: VkIndirectCommandsTokenTypeNVX = 0;
pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DESCRIPTOR_SET_NVX: VkIndirectCommandsTokenTypeNVX = 1;
pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_INDEX_BUFFER_NVX: VkIndirectCommandsTokenTypeNVX = 2;
pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_VERTEX_BUFFER_NVX: VkIndirectCommandsTokenTypeNVX = 3;
pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NVX: VkIndirectCommandsTokenTypeNVX = 4;
pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_INDEXED_NVX: VkIndirectCommandsTokenTypeNVX = 5;
pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_NXV: VkIndirectCommandsTokenTypeNVX = 6;
pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DISPATCH_NVX: VkIndirectCommandsTokenTypeNVX = 7;

pub type VkObjectEntryTypeNVX = i32;
pub const VK_OBJECT_ENTRY_TYPE_DESCRIPTOR_SET_NVX: VkObjectEntryTypeNVX = 0;
pub const VK_OBJECT_ENTRY_TYPE_PIPELINE_NVX: VkObjectEntryTypeNVX = 1;
pub const VK_OBJECT_ENTRY_TYPE_INDEX_BUFFER_NVX: VkObjectEntryTypeNVX = 2;
pub const VK_OBJECT_ENTRY_TYPE_VERTEX_BUFFER_NVX: VkObjectEntryTypeNVX = 3;
pub const VK_OBJECT_ENTRY_TYPE_PUSH_CONSTANT_NVX: VkObjectEntryTypeNVX = 4;

pub type VkIndirectCommandsLayoutUsageFlagsNVX = VkFlags;
pub const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_UNORDERED_SEQUENCES_BIT_NVX: VkIndirectCommandsLayoutUsageFlagsNVX = 0x01;
pub const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_SPARSE_SEQUENCES_BIT_NVX: VkIndirectCommandsLayoutUsageFlagsNVX = 0x02;
pub const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_EMPTY_EXECUTIONS_BIT_NVX: VkIndirectCommandsLayoutUsageFlagsNVX = 0x04;
pub const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_INDEXED_SEQUENCES_BIT_NVX: VkIndirectCommandsLayoutUsageFlagsNVX = 0x08;

pub type VkObjectEntryUsageFlagsNVX = VkFlags;
pub const VK_OBJECT_ENTRY_USAGE_GRAPHICS_BIT_NVX: VkObjectEntryUsageFlagsNVX = 0x01;
pub const VK_OBJECT_ENTRY_USAGE_COMPUTE_BIT_NVX: VkObjectEntryUsageFlagsNVX = 0x02;

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDeviceGeneratedCommandsFeaturesNVX
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub computeBindingPointSupport: VkBool32
}
impl Default for VkDeviceGeneratedCommandsFeaturesNVX
{
	fn default() -> Self
	{
		VkDeviceGeneratedCommandsFeaturesNVX
		{
			sType: VK_STRUCTURE_TYPE_DEVICE_GENERATED_COMMANDS_FEATURES_NVX,
			.. unsafe { std::mem::zeroed() }
		}
	}
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDeviceGeneratedCommandsLimitsNVX
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub maxIndirectCommandsLayoutTokenCount: u32,
	pub maxObjectEntryCounts: u32,
	pub minSequenceCountBufferOffsetAlignment: u32,
	pub minSequenceIndexBufferOffsetAlignment: u32,
	pub minCommandsTokenBufferOffsetAlignment: u32
}
impl Default for VkDeviceGeneratedCommandsLimitsNVX
{
	fn default() -> Self
	{
		VkDeviceGeneratedCommandsLimitsNVX
		{
			sType: VK_STRUCTURE_TYPE_DEVICE_GENERATED_COMMANDS_LIMITS_NVX,
			.. unsafe { std::mem::zeroed() }
		}
	}
}

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkIndirectCommandsTokenNVX
{
	pub tokenType: VkIndirectCommandsTokenTypeNVX,
	pub buffer: VkBuffer, pub offset: VkDeviceSize
}
impl Default for VkIndirectCommandsTokenNVX
{
	fn default() -> Self { unsafe { std::mem::zeroed() } }
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkIndirectCommandsLayoutTokenNVX
{
	pub tokenType: VkIndirectCommandsTokenTypeNVX,
	pub bindingUnit: u32, pub dynamicCount: u32, pub divisor: u32
}
impl Default for VkIndirectCommandsLayoutTokenNVX
{
	fn default() -> Self { unsafe { std::mem::zeroed() } }
}

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkIndirectCommandsLayoutCreateInfoNVX
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub pipelineBindPoint: VkPipelineBindPoint,
	pub flags: VkIndirectCommandsLayoutUsageFlagsNVX,
	pub tokenCount: u32, pub pTokens: *const VkIndirectCommandsLayoutTokenNVX
}
impl Default for VkIndirectCommandsLayoutCreateInfoNVX
{
	fn default() -> Self
	{
		VkIndirectCommandsLayoutCreateInfoNVX
		{
			sType: VK_STRUCTURE_TYPE_INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NVX,
			.. unsafe { std::mem::zeroed() }
		}
	}
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkCmdProcessCommandsInfoNVX
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub objectTable: VkObjectTableNVX, pub indirectCommandsLayout: VkIndirectCommandsLayoutNVX,
	pub indirectCommandsTokenCount: u32,
	pub pIndirectCommandsTokens: *const VkIndirectCommandsTokenNVX,
	pub maxSequencesCount: u32, pub targetCommandBuffer: VkCommandBuffer,
	pub sequencesCountBuffer: VkBuffer, pub sequencesCountOffset: VkDeviceSize,
	pub sequencesIndexBuffer: VkBuffer, pub sequencesIndexOffset: VkDeviceSize
}
impl Default for VkCmdProcessCommandsInfoNVX
{
	fn default() -> Self
	{
		VkCmdProcessCommandsInfoNVX
		{
			sType: VK_STRUCTURE_TYPE_CMD_PROCESS_COMMANDS_INFO_NVX,
			.. unsafe { std::mem::zeroed() }
		}
	}
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkCmdReserveSpaceForCommandsInfoNVX
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub objectTable: VkObjectTableNVX,
	pub indirectCommandsLayout: VkIndirectCommandsLayoutNVX,
	pub maxSequencesCount: u32
}
impl Default for VkCmdReserveSpaceForCommandsInfoNVX
{
	fn default() -> Self
	{
		VkCmdReserveSpaceForCommandsInfoNVX
		{
			sType: VK_STRUCTURE_TYPE_CMD_RESERVE_SPACE_FOR_COMMANDS_INFO_NVX,
			.. unsafe { std::mem::zeroed() }
		}
	}
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkObjectTableCreateInfoNVX
{
	pub sType: VkStructureType, pub pNext: *const c_void,
	pub objectCount: u32, pub pObjectEntryTypes: *const VkObjectEntryTypeNVX,
	pub pObjectEntryCounts: *const u32, pub pObjectEntryUsageFlags: *const VkObjectEntryUsageFlagsNVX,
	pub maxUniformBuffersPerDescriptor: u32, pub maxStorageBuffersPerDescriptor: u32,
	pub maxStorageImagesPerDescriptor: u32, pub maxSampledImagesPerDescriptor: u32,
	pub maxPipelineLayouts: u32
}
impl Default for VkObjectTableCreateInfoNVX
{
	fn default() -> Self
	{
		VkObjectTableCreateInfoNVX
		{
			sType: VK_STRUCTURE_TYPE_OBJECT_TABLE_CREATE_INFO_NVX,
			.. unsafe { std::mem::zeroed() }
		}
	}
}

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkObjectTableEntryNVX
{
	pub _type: VkObjectEntryTypeNVX, pub flags: VkObjectEntryUsageFlagsNVX
}
impl Default for VkObjectTableEntryNVX
{
	fn default() -> Self { unsafe { std::mem::zeroed() } }
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkObjectTablePipelineEntryNVX
{
	pub _type: VkObjectEntryTypeNVX, pub flags: VkObjectEntryUsageFlagsNVX,
	pub pipeline: VkPipeline
}
impl Default for VkObjectTablePipelineEntryNVX
{
	fn default() -> Self
	{
		VkObjectTablePipelineEntryNVX
		{
			_type: VK_OBJECT_ENTRY_TYPE_PIPELINE_NVX,
			.. unsafe { std::mem::zeroed() }
		}
	}
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkObjectTableDescriptorSetEntryNVX
{
	pub _type: VkObjectEntryTypeNVX, pub flags: VkObjectEntryUsageFlagsNVX,
	pub pipelineLayout: VkPipelineLayout, pub descriptorSet: VkDescriptorSet
}
impl Default for VkObjectTableDescriptorSetEntryNVX
{
	fn default() -> Self
	{
		VkObjectTableDescriptorSetEntryNVX
		{
			_type: VK_OBJECT_ENTRY_TYPE_DESCRIPTOR_SET_NVX,
			.. unsafe { std::mem::zeroed() }
		}
	}
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkObjectTableVertexBufferEntryNVX
{
	pub _type: VkObjectEntryTypeNVX, pub flags: VkObjectEntryUsageFlagsNVX,
	pub buffer: VkBuffer
}
impl Default for VkObjectTableVertexBufferEntryNVX
{
	fn default() -> Self
	{
		VkObjectTableVertexBufferEntryNVX
		{
			_type: VK_OBJECT_ENTRY_TYPE_VERTEX_BUFFER_NVX,
			.. unsafe { std::mem::zeroed() }
		}
	}
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkObjectTableIndexBufferEntryNVX
{
	pub _type: VkObjectEntryTypeNVX, pub flags: VkObjectEntryUsageFlagsNVX,
	pub buffer: VkBuffer, pub indexType: VkIndexType
}
impl Default for VkObjectTableIndexBufferEntryNVX
{
	fn default() -> Self
	{
		VkObjectTableIndexBufferEntryNVX
		{
			_type: VK_OBJECT_ENTRY_TYPE_INDEX_BUFFER_NVX,
			.. unsafe { std::mem::zeroed() }
		}
	}
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkObjectTablePushConstantEntryNVX
{
	pub _type: VkObjectEntryTypeNVX, pub flags: VkObjectEntryUsageFlagsNVX,
	pub pipelineLayout: VkPipelineLayout, pub stageFlags: VkShaderStageFlags
}
impl Default for VkObjectTablePushConstantEntryNVX
{
	fn default() -> Self
	{
		VkObjectTablePushConstantEntryNVX
		{
			_type: VK_OBJECT_ENTRY_TYPE_PUSH_CONSTANT_NVX,
			.. unsafe { std::mem::zeroed() }
		}
	}
}

pub type PFN_vkCmdProcessCommandsNVX = extern "system" fn(commandBuffer: VkCommandBuffer, pProcessCommandsInfo: *const VkCmdProcessCommandsInfoNVX);
pub type PFN_vkCmdReserveSpaceForCommandsNVX = extern "system" fn(commandBuffer: VkCommandBuffer, pReserveSpaceInfo: *const VkCmdReserveSpaceForCommandsInfoNVX);
pub type PFN_vkCreateIndirectCommandsLayoutNVX = extern "system" fn(device: VkDevice, pCreateInfo: *const VkIndirectCommandsLayoutCreateInfoNVX, pAllocator: *const VkAllocationCallbacks, pIndirectCommandsLayout: *mut VkIndirectCommandsLayoutNVX) -> VkResult;
pub type PFN_vkDestroyIndirectCommandsLayoutNVX = extern "system" fn(device: VkDevice, indirectCommandsLayout: VkIndirectCommandsLayoutNVX, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkCreateObjectTableNVX = extern "system" fn(device: VkDevice, pCreateInfo: *const VkObjectTableCreateInfoNVX, pAllocator: *const VkAllocationCallbacks, pObjectTable: *mut VkObjectTableNVX) -> VkResult;
pub type PFN_vkDestroyObjectTableNVX = extern "system" fn(device: VkDevice, objectTable: VkObjectTableNVX, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkRegisterObjectsNVX = extern "system" fn(device: VkDevice, objectTable: VkObjectTableNVX, objectCount: u32, ppObjectTableEntries: *const *const VkObjectTableEntryNVX, pObjectIndices: *const u32) -> VkResult;
pub type PFN_vkUnregisterObjectsNVX = extern "system" fn(device: VkDevice, objectTable: VkObjectTableNVX, objectCount: u32, pObjectEntryTypes: *const VkObjectEntryTypeNVX, pObjectIndices: *const u32) -> VkResult;
pub type PFN_vkGetPhysicalDeviceGeneratedCommandsPropertiesNVX = extern "system" fn(physicalDevice: VkPhysicalDevice, pFeatures: *mut VkDeviceGeneratedCommandsFeaturesNVX, pLimits: *mut VkDeviceGeneratedCommandsLimitsNVX);

#![allow(non_snake_case)]
#![allow(dead_code)]

// Vulkan C to Rust FFI Enumerations
use super::types::VkFlags;

#[repr(C)]
pub enum VkSystemAllocationScope
{
	Command = 0, Object = 1, Cache = 2, Device = 3, Instance = 4
}
#[repr(C)]
pub enum VkInternalAllocationType
{
	Executable = 0
}
#[repr(C)] #[derive(Clone, Copy, PartialEq, Eq)]
pub enum VkStructureType
{
	ApplicationInfo = 0,
	InstanceCreateInfo = 1,
	DeviceQueueCreateInfo = 2,
	DeviceCreateInfo = 3,
	SubmitInfo = 4,
	MemoryAllocateInfo = 5,
	MappedMemoryRange = 6,
	BindSparseInfo = 7,
	FenceCreateInfo = 8,
	SemaphoreCreateInfo = 9,
	EventCreateInfo = 10,
	QueryPoolCreateInfo = 11,
	BufferCreateInfo = 12,
	BufferViewCreateInfo = 13,
	ImageCreateInfo = 14,
	ImageViewCreateInfo = 15,
	ShaderModuleCreateInfo = 16,
	PipelineCacheCreateInfo = 17,
	Pipeline_ShaderStageCreateInfo = 18,
	Pipeline_VertexInputStateCreateInfo = 19,
	Pipeline_InputAssemblyStateCreateInfo = 20,
	Pipeline_TessellationStateCreateInfo = 21,
	Pipeline_ViewportStateCreateInfo = 22,
	Pipeline_RasterizationStateCreateInfo = 23,
	Pipeline_MultisampleStateCreateInfo = 24,
	Pipeline_DepthStencilStateCreateInfo = 25,
	Pipeline_ColorBlendStateCreateInfo = 26,
	Pipeline_DynamicStateCreateInfo = 27,
	GraphicsPipelineCreateInfo = 28,
	ComputePipelineCreateInfo = 29,
	PipelineLayoutCreateInfo = 30,
	SamplerCreateInfo = 31,
	DescriptorSetLayoutCreateInfo = 32,
	DescriptorPoolCreateInfo = 33,
	DescriptorSetAllocateInfo = 34,
	WriteDescriptorSet = 35,
	CopyDescriptorSet = 36,
	FramebufferCreateInfo = 37,
	RenderPassCreateInfo = 38,
	CommandPoolCreateInfo = 39,
	CommandBufferAllocateInfo = 40,
	CommandBufferInheritanceInfo = 41,
	CommandBufferBeginInfo = 42,
	RenderPassBeginInfo = 43,
	BufferMemoryBarrier = 44,
	ImageMemoryBarrier = 45,
	MemoryBarrier = 46,
	LoaderInstanceCreateInfo = 47,
	LoaderDeviceCreateInfo = 48,
	SwapchainCreateInfoKHR = 1000001000,
	PresentInfoKHR= 1000001001,
	DisplayModeCreateInfoKHR = 1000002000,
	DisplaySurfaceCreateInfoKHR = 1000002001,
	DisplayPresentInfoKHR = 1000003000,
	XlibSurfaceCreateInfoKHR = 1000004000,
	XcbSurfaceCreateInfoKHR = 1000005000,
	WaylandSurfaceCreateInfoKHR = 1000006000,
	MIRSurfaceCreateInfoKHR = 1000007000,
	AndroidSurfaceCreateInfoKHR = 1000008000,
	Win32SurfaceCreateInfoKHR = 1000009000,
	DebugReportCallbackCreateInfoEXT = 1000011000,
	Pipeline_RasterizationState_RasterizationOrderAMD = 1000018000,
	DebugMarker_ObjectNameInfoEXT = 1000022000,
	DebugMarker_ObjectTagInfoEXT = 1000022001,
	DebugMarker_MarkerInfoEXT = 1000022002
}
#[repr(C)] #[derive(Clone, Debug, Eq, PartialEq, Copy, Hash)]
pub enum VkFormat
{
	UNDEFINED = 0,
	R4G4_UNORM_PACK8 = 1,
	R4G4B4A4_UNORM_PACK16 = 2,
	B4G4R4A4_UNORM_PACK16 = 3,
	R5G6B5_UNORM_PACK16 = 4,
	B5G6R5_UNORM_PACK16 = 5,
	R5G5B5A1_UNORM_PACK16 = 6,
	B5G5R5A1_UNORM_PACK16 = 7,
	A1R5G5B5_UNORM_PACK16 = 8,
	R8_UNORM = 9,
	R8_SNORM = 10,
	R8_USCALED = 11,
	R8_SSCALED = 12,
	R8_UINT = 13,
	R8_SINT = 14,
	R8_SRGB = 15,
	R8G8_UNORM = 16,
	R8G8_SNORM = 17,
	R8G8_USCALED = 18,
	R8G8_SSCALED = 19,
	R8G8_UINT = 20,
	R8G8_SINT = 21,
	R8G8_SRGB = 22,
	R8G8B8_UNORM = 23,
	R8G8B8_SNORM = 24,
	R8G8B8_USCALED = 25,
	R8G8B8_SSCALED = 26,
	R8G8B8_UINT = 27,
	R8G8B8_SINT = 28,
	R8G8B8_SRGB = 29,
	B8G8R8_UNORM = 30,
	B8G8R8_SNORM = 31,
	B8G8R8_USCALED = 32,
	B8G8R8_SSCALED = 33,
	B8G8R8_UINT = 34,
	B8G8R8_SINT = 35,
	B8G8R8_SRGB = 36,
	R8G8B8A8_UNORM = 37,
	R8G8B8A8_SNORM = 38,
	R8G8B8A8_USCALED = 39,
	R8G8B8A8_SSCALED = 40,
	R8G8B8A8_UINT = 41,
	R8G8B8A8_SINT = 42,
	R8G8B8A8_SRGB = 43,
	B8G8R8A8_UNORM = 44,
	B8G8R8A8_SNORM = 45,
	B8G8R8A8_USCALED = 46,
	B8G8R8A8_SSCALED = 47,
	B8G8R8A8_UINT = 48,
	B8G8R8A8_SINT = 49,
	B8G8R8A8_SRGB = 50,
	A8B8G8R8_UNORM_PACK32 = 51,
	A8B8G8R8_SNORM_PACK32 = 52,
	A8B8G8R8_USCALED_PACK32 = 53,
	A8B8G8R8_SSCALED_PACK32 = 54,
	A8B8G8R8_UINT_PACK32 = 55,
	A8B8G8R8_SINT_PACK32 = 56,
	A8B8G8R8_SRGB_PACK32 = 57,
	A2R10G10B10_UNORM_PACK32 = 58,
	A2R10G10B10_SNORM_PACK32 = 59,
	A2R10G10B10_USCALED_PACK32 = 60,
	A2R10G10B10_SSCALED_PACK32 = 61,
	A2R10G10B10_UINT_PACK32 = 62,
	A2R10G10B10_SINT_PACK32 = 63,
	A2B10G10R10_UNORM_PACK32 = 64,
	A2B10G10R10_SNORM_PACK32 = 65,
	A2B10G10R10_USCALED_PACK32 = 66,
	A2B10G10R10_SSCALED_PACK32 = 67,
	A2B10G10R10_UINT_PACK32 = 68,
	A2B10G10R10_SINT_PACK32 = 69,
	R16_UNORM = 70,
	R16_SNORM = 71,
	R16_USCALED = 72,
	R16_SSCALED = 73,
	R16_UINT = 74,
	R16_SINT = 75,
	R16_SFLOAT = 76,
	R16G16_UNORM = 77,
	R16G16_SNORM = 78,
	R16G16_USCALED = 79,
	R16G16_SSCALED = 80,
	R16G16_UINT = 81,
	R16G16_SINT = 82,
	R16G16_SFLOAT = 83,
	R16G16B16_UNORM = 84,
	R16G16B16_SNORM = 85,
	R16G16B16_USCALED = 86,
	R16G16B16_SSCALED = 87,
	R16G16B16_UINT = 88,
	R16G16B16_SINT = 89,
	R16G16B16_SFLOAT = 90,
	R16G16B16A16_UNORM = 91,
	R16G16B16A16_SNORM = 92,
	R16G16B16A16_USCALED = 93,
	R16G16B16A16_SSCALED = 94,
	R16G16B16A16_UINT = 95,
	R16G16B16A16_SINT = 96,
	R16G16B16A16_SFLOAT = 97,
	R32_UINT = 98,
	R32_SINT = 99,
	R32_SFLOAT = 100,
	R32G32_UINT = 101,
	R32G32_SINT = 102,
	R32G32_SFLOAT = 103,
	R32G32B32_UINT = 104,
	R32G32B32_SINT = 105,
	R32G32B32_SFLOAT = 106,
	R32G32B32A32_UINT = 107,
	R32G32B32A32_SINT = 108,
	R32G32B32A32_SFLOAT = 109,
	R64_UINT = 110,
	R64_SINT = 111,
	R64_SFLOAT = 112,
	R64G64_UINT = 113,
	R64G64_SINT = 114,
	R64G64_SFLOAT = 115,
	R64G64B64_UINT = 116,
	R64G64B64_SINT = 117,
	R64G64B64_SFLOAT = 118,
	R64G64B64A64_UINT = 119,
	R64G64B64A64_SINT = 120,
	R64G64B64A64_SFLOAT = 121,
	B10G11R11_UFLOAT_PACK32 = 122,
	E5B9G9R9_UFLOAT_PACK32 = 123,
	D16_UNORM = 124,
	X8_D24_UNORM_PACK32 = 125,
	D32_SFLOAT = 126,
	S8_UINT = 127,
	D16_UNORM_S8_UINT = 128,
	D24_UNORM_S8_UINT = 129,
	D32_SFLOAT_S8_UINT = 130,
	BC1_RGB_UNORM_BLOCK = 131,
	BC1_RGB_SRGB_BLOCK = 132,
	BC1_RGBA_UNORM_BLOCK = 133,
	BC1_RGBA_SRGB_BLOCK = 134,
	BC2_UNORM_BLOCK = 135,
	BC2_SRGB_BLOCK = 136,
	BC3_UNORM_BLOCK = 137,
	BC3_SRGB_BLOCK = 138,
	BC4_UNORM_BLOCK = 139,
	BC4_SNORM_BLOCK = 140,
	BC5_UNORM_BLOCK = 141,
	BC5_SNORM_BLOCK = 142,
	BC6H_UFLOAT_BLOCK = 143,
	BC6H_SFLOAT_BLOCK = 144,
	BC7_UNORM_BLOCK = 145,
	BC7_SRGB_BLOCK = 146,
	ETC2_R8G8B8_UNORM_BLOCK = 147,
	ETC2_R8G8B8_SRGB_BLOCK = 148,
	ETC2_R8G8B8A1_UNORM_BLOCK = 149,
	ETC2_R8G8B8A1_SRGB_BLOCK = 150,
	ETC2_R8G8B8A8_UNORM_BLOCK = 151,
	ETC2_R8G8B8A8_SRGB_BLOCK = 152,
	EAC_R11_UNORM_BLOCK = 153,
	EAC_R11_SNORM_BLOCK = 154,
	EAC_R11G11_UNORM_BLOCK = 155,
	EAC_R11G11_SNORM_BLOCK = 156,
	ASTC_4x4_UNORM_BLOCK = 157,
	ASTC_4x4_SRGB_BLOCK = 158,
	ASTC_5x4_UNORM_BLOCK = 159,
	ASTC_5x4_SRGB_BLOCK = 160,
	ASTC_5x5_UNORM_BLOCK = 161,
	ASTC_5x5_SRGB_BLOCK = 162,
	ASTC_6x5_UNORM_BLOCK = 163,
	ASTC_6x5_SRGB_BLOCK = 164,
	ASTC_6x6_UNORM_BLOCK = 165,
	ASTC_6x6_SRGB_BLOCK = 166,
	ASTC_8x5_UNORM_BLOCK = 167,
	ASTC_8x5_SRGB_BLOCK = 168,
	ASTC_8x6_UNORM_BLOCK = 169,
	ASTC_8x6_SRGB_BLOCK = 170,
	ASTC_8x8_UNORM_BLOCK = 171,
	ASTC_8x8_SRGB_BLOCK = 172,
	ASTC_10x5_UNORM_BLOCK = 173,
	ASTC_10x5_SRGB_BLOCK = 174,
	ASTC_10x6_UNORM_BLOCK = 175,
	ASTC_10x6_SRGB_BLOCK = 176,
	ASTC_10x8_UNORM_BLOCK = 177,
	ASTC_10x8_SRGB_BLOCK = 178,
	ASTC_10x10_UNORM_BLOCK = 179,
	ASTC_10x10_SRGB_BLOCK = 180,
	ASTC_12x10_UNORM_BLOCK = 181,
	ASTC_12x10_SRGB_BLOCK = 182,
	ASTC_12x12_UNORM_BLOCK = 183,
	ASTC_12x12_SRGB_BLOCK = 184
}
#[repr(C)] #[derive(PartialEq, Eq, Debug)]
pub enum VkResult
{
	Success = 0,
	NotReady = 1,
	Timeout = 2,
	EventSet = 3,
	EventReset = 4,
	Incomplete = 5,
	Error_OutOfHostMemory = -1,
	Error_OutOfDeviceMemory = -2,
	Error_InitializationFailed = -3,
	Error_DeviceLost = -4,
	Error_MemoryMapFailed = -5,
	Error_LayerNotPresented = -6,
	Error_ExtensionNotPresented = -7,
	Error_FeatureNotPresent = -8,
	Error_IncompatibleDriver = -9,
	Error_TooManyObjects = -10,
	Error_FormatNotSupported = -11,
	Error_SurfaceLostKHR = -1000000000,
	Error_NativeWindowInUseKHR = -1000000001,
	SuboptimalKHR = 1000001003,
	Error_OutOfDateKHR = -1000001004,
	Error_IncompatibleDisplayKHR = -1000003001,
	Error_ValidationFailedEXT = -1000011001,
	Error_InvalidShaderNV = -1000012000
}

pub const VK_QUEUE_GRAPHICS_BIT: VkFlags		= 0x01;
pub const VK_QUEUE_COMPUTE_BIT: VkFlags			= 0x02;
pub const VK_QUEUE_TRANSFER_BIT: VkFlags		= 0x04;
pub const VK_QUEUE_SPARSE_BINDING_BIT: VkFlags	= 0x08;
#[repr(C)] pub enum VkPhysicalDeviceType
{
	Other = 0,
	IntegratedGPU = 1,
	DiscreteGPU = 2,
	VirtualGPU = 3,
	CPU = 4
}
#[repr(C)] pub enum VkImageViewType
{
	Dim1 = 0, Dim2 = 1, Dim3 = 2, Cube = 3, Dim1Array = 4, Dim2Array = 5, CubeArray = 6
}
#[repr(C)] pub enum VkComponentSwizzle
{
	Identity = 0, Zero = 1, One = 2,
	R = 3, G = 4, B = 5, A = 6
}
#[repr(C)] #[derive(Clone, Copy)] pub enum VkSharingMode
{
	Exclusive = 0,
	Concurrent = 1
}
#[repr(C)] #[derive(Clone, Copy)] pub enum VkImageLayout
{
	Undefined = 0, General = 1,
	ColorAttachmentOptimal = 2, DepthStencilAttachmentOptimal = 3, DepthStencilReadOnlyOptimal = 4,
	ShaderReadOnlyOptimal = 5, TransferSrcOptimal = 6, TransferDestOptimal = 7, Preinitialized = 8,
	PresentSrcKHR = 1000001002
}
pub const VK_IMAGE_ASPECT_COLOR_BIT: VkFlags	= 0x00000001;
pub const VK_IMAGE_ASPECT_DEPTH_BIT: VkFlags	= 0x00000002;
pub const VK_IMAGE_ASPECT_STENCIL_BIT: VkFlags	= 0x00000004;
pub const VK_IMAGE_ASPECT_METADATA_BIT: VkFlags	= 0x00000008;

pub const VK_ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT: VkFlags = 0x00000001;

pub const VK_ACCESS_INDIRECT_COMMAND_READ_BIT: VkFlags			= 0x00000001;
pub const VK_ACCESS_INDEX_READ_BIT: VkFlags						= 0x00000002;
pub const VK_ACCESS_VERTEX_ATTRIBUTE_READ_BIT: VkFlags			= 0x00000004;
pub const VK_ACCESS_UNIFORM_READ_BIT: VkFlags					= 0x00000008;
pub const VK_ACCESS_INPUT_ATTACHMENT_READ_BIT: VkFlags			= 0x00000010;
pub const VK_ACCESS_SHADER_READ_BIT: VkFlags					= 0x00000020;
pub const VK_ACCESS_SHADER_WRITE_BIT: VkFlags					= 0x00000040;
pub const VK_ACCESS_COLOR_ATTACHMENT_READ_BIT: VkFlags			= 0x00000080;
pub const VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT: VkFlags			= 0x00000100;
pub const VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT: VkFlags	= 0x00000200;
pub const VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT: VkFlags	= 0x00000400;
pub const VK_ACCESS_TRANSFER_READ_BIT: VkFlags					= 0x00000800;
pub const VK_ACCESS_TRANSFER_WRITE_BIT: VkFlags					= 0x00001000;
pub const VK_ACCESS_HOST_READ_BIT: VkFlags						= 0x00002000;
pub const VK_ACCESS_HOST_WRITE_BIT: VkFlags						= 0x00004000;
pub const VK_ACCESS_MEMORY_READ_BIT: VkFlags					= 0x00008000;
pub const VK_ACCESS_MEMORY_WRITE_BIT: VkFlags					= 0x00010000;

pub const VK_SAMPLE_COUNT_1_BIT: VkFlags	= 0x00000001;
pub const VK_SAMPLE_COUNT_2_BIT: VkFlags	= 0x00000002;
pub const VK_SAMPLE_COUNT_4_BIT: VkFlags	= 0x00000004;
pub const VK_SAMPLE_COUNT_8_BIT: VkFlags	= 0x00000008;
pub const VK_SAMPLE_COUNT_16_BIT: VkFlags	= 0x00000010;
pub const VK_SAMPLE_COUNT_32_BIT: VkFlags	= 0x00000020;
pub const VK_SAMPLE_COUNT_64_BIT: VkFlags	= 0x00000040;

pub const VK_IMAGE_CREATE_SPARSE_BINDING_BIT: VkFlags	= 0x01;
pub const VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT: VkFlags	= 0x02;
pub const VK_IMAGE_CREATE_SPARSE_ALIASED_BIT: VkFlags	= 0x04;
pub const VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT: VkFlags	= 0x08;
pub const VK_IMAGE_CREATE_CUBE_COMPATIBLE_BIT: VkFlags	= 0x10;

pub const VK_IMAGE_USAGE_TRANSFER_SRC_BIT: VkFlags				= 0x01;
pub const VK_IMAGE_USAGE_TRANSFER_DST_BIT: VkFlags				= 0x02;
pub const VK_IMAGE_USAGE_SAMPLED_BIT: VkFlags					= 0x04;
pub const VK_IMAGE_USAGE_STORAGE_BIT: VkFlags					= 0x08;
pub const VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT: VkFlags			= 0x10;
pub const VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT: VkFlags	= 0x20;
pub const VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT: VkFlags		= 0x40;
pub const VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT: VkFlags			= 0x80;

#[repr(C)] #[derive(Copy, Clone)] pub enum VkImageTiling
{
	Optimal = 0, Linear = 1
}
#[repr(C)] #[derive(PartialEq, Debug, Clone, Copy)] pub enum VkImageType
{
	Dim1 = 0, Dim2 = 1, Dim3 = 2
}

#[repr(C)] pub enum VkAttachmentLoadOp
{
	Load = 0, Clear = 1, DontCare = 2
}
#[repr(C)] pub enum VkAttachmentStoreOp
{
	Store = 0, DontCare = 1
}
#[repr(C)] pub enum VkPipelineBindPoint
{
	Graphics = 0, Compute = 1
}
#[repr(C)] pub enum VkCommandBufferLevel
{
	Primary = 0, Secondary = 1
}
#[repr(C)] pub enum VkIndexType
{
	U16 = 0, U32 = 1
}
#[repr(C)] pub enum VkSubpassContents
{
	Inline = 0, SecondaryCommandBuffers = 1
}

pub const VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT: VkFlags					= 0x00000001;
pub const VK_PIPELINE_STAGE_DRAW_INDIRECT_BIT: VkFlags					= 0x00000002;
pub const VK_PIPELINE_STAGE_VERTEX_INPUT_BIT: VkFlags					= 0x00000004;
pub const VK_PIPELINE_STAGE_VERTEX_SHADER_BIT: VkFlags					= 0x00000008;
pub const VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT: VkFlags	= 0x00000010;
pub const VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT: VkFlags	= 0x00000020;
pub const VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT: VkFlags				= 0x00000040;
pub const VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT: VkFlags				= 0x00000080;
pub const VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT: VkFlags			= 0x00000100;
pub const VK_PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT: VkFlags			= 0x00000200;
pub const VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT: VkFlags		= 0x00000400;
pub const VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT: VkFlags					= 0x00000800;
pub const VK_PIPELINE_STAGE_TRANSFER_BIT: VkFlags						= 0x00001000;
pub const VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT: VkFlags					= 0x00002000;
pub const VK_PIPELINE_STAGE_HOST_BIT: VkFlags							= 0x00004000;
pub const VK_PIPELINE_STAGE_ALL_GRAPHICS_BIT: VkFlags					= 0x00008000;
pub const VK_PIPELINE_STAGE_ALL_COMMANDS_BIT: VkFlags					= 0x00010000;

pub const VK_DEPENDENCY_BY_REGION_BIT: VkFlags = 0x00000001;

pub const VK_COMMAND_POOL_CREATE_TRANSIENT_BIT: VkFlags             = 0x00000001;
pub const VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT: VkFlags  = 0x00000002;

pub const VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT: VkFlags		= 0x01;
pub const VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT: VkFlags	= 0x02;
pub const VK_COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT: VkFlags		= 0x04;

pub const VK_PIPELINE_CREATE_DISABLE_OPTIMIZATION_BIT: VkFlags  = 0x00000001;
pub const VK_PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT: VkFlags     = 0x00000002;
pub const VK_PIPELINE_CREATE_DERIVATIVE_BIT: VkFlags            = 0x00000004;

pub const VK_SHADER_STAGE_VERTEX_BIT: VkFlags                   = 0x00000001;
pub const VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT: VkFlags     = 0x00000002;
pub const VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT: VkFlags  = 0x00000004;
pub const VK_SHADER_STAGE_GEOMETRY_BIT: VkFlags                 = 0x00000008;
pub const VK_SHADER_STAGE_FRAGMENT_BIT: VkFlags                 = 0x00000010;
pub const VK_SHADER_STAGE_COMPUTE_BIT: VkFlags                  = 0x00000020;
pub const VK_SHADER_STAGE_ALL_GRAPHICS: VkFlags                 = 0x0000001f;
pub const VK_SHADER_STAGE_ALL: VkFlags                          = 0x7fffffff;

pub const VK_CULL_MODE_NONE: VkFlags            = 0;
pub const VK_CULL_MODE_FRONT_BIT: VkFlags       = 0x01;
pub const VK_CULL_MODE_BACK_BIT: VkFlags        = 0x02;
pub const VK_CULL_MODE_FRONT_AND_BACK: VkFlags  = 0x03;

pub const VK_COLOR_COMPONENT_R_BIT: VkFlags     = 0x01;
pub const VK_COLOR_COMPONENT_G_BIT: VkFlags     = 0x02;
pub const VK_COLOR_COMPONENT_B_BIT: VkFlags     = 0x04;
pub const VK_COLOR_COMPONENT_A_BIT: VkFlags     = 0x08;

pub const VK_FENCE_CREATE_SIGNALED_BIT: VkFlags	 = 0x01;

pub const VK_QUEUE_FAMILY_IGNORED: u32 = 0u32;

#[repr(C)] pub enum VkVertexInputRate
{
	Vertex = 0, Instance = 1
}
#[repr(C)] pub enum VkPrimitiveTopology
{
	PointList = 0, LineList = 1, LineStrip = 2,
	TriangleList = 3, TriangleStrip = 4, TriangleFan = 5,
	LineListWithAdjacency = 6, LineStripWithAdjacency = 7,
	TriangleListWithAdjacency = 8, TriangleStripWithAdjacency = 9,
	PatchList = 10
}
#[repr(C)] pub enum VkPolygonMode
{
	Fill = 0, Line = 1, Point = 2
}
#[repr(C)] pub enum VkFrontFace
{
	CounterClockwise = 0, Clockwise = 1
}
#[repr(C)] pub enum VkCompareOp
{
	Never = 0, Less = 1, Equal = 2, LessOrEqual = 3,
	Greater = 4, NotEqual = 5, GreaterOrEqual = 6,
	Always = 7
}
#[repr(C)] pub enum VkStencilOp
{
	Keep = 0, Zero = 1, Replace = 2,
	IncrementAndClamp = 3, DecrementAndClamp = 4,
	Invert = 5, IncrementAndWrap = 6, DecrementAndWrap = 7
}
#[repr(C)] pub enum VkLogicOp
{
	Clear = 0, And = 1, AndReverse = 2,
	Copy = 3, AndInverted = 4, NOP = 5,
	Xor = 6, Or = 7, Nor = 8, Equivalent = 9,
	Invert = 10, OrReverse = 11, CopyInverted = 12,
	OrInverted = 13, Nand = 14, Set = 15
}
#[repr(C)] pub enum VkBlendFactor
{
	Zero = 0, One = 1,
	SrcColor = 2, OneMinusSrcColor = 3,
	DstColor = 4, OneMinusDstColor = 5,
	SrcAlpha = 6, OneMinusSrcAlpha = 7,
	DstAlpha = 8, OneMinusDstAlpha = 9,
	ConstantColor = 10, OneMinusConstantColor = 11,
	ConstantAlpha = 12, OneMinusConstantAlpha = 13,
	SrcAlphaSaturate = 14,
	Src1Color = 15, OneMinusSrc1Color = 16,
	Src1Alpha = 17, OneMinusSrc1Alpha = 18
}
#[repr(C)] pub enum VkBlendOp
{
	Add = 0, Subtract = 1, ReverseSubtract = 2,
	Min = 3, Max = 4
}
#[repr(C)] pub enum VkDynamicState
{
	Viewport = 0, Scissor = 1, LineWidth = 2,
	DepthBias = 3, BlendConstants = 4, DepthBounds = 5,
	StencilCompareMask = 6, StencilWriteMask = 7,
	StencilReference = 8
}

pub const VK_BUFFER_CREATE_SPARSE_BINDING_BIT: VkFlags		= 0x01;
pub const VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT: VkFlags	= 0x02;
pub const VK_BUFFER_CREATE_SPARSE_ALIASED_BIT: VkFlags		= 0x04;

pub const VK_BUFFER_USAGE_TRANSFER_SRC_BIT: VkFlags			= 0x0001;
pub const VK_BUFFER_USAGE_TRANSFER_DST_BIT: VkFlags			= 0x0002;
pub const VK_BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT: VkFlags	= 0x0004;
pub const VK_BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT: VkFlags	= 0x0008;
pub const VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT: VkFlags		= 0x0010;
pub const VK_BUFFER_USAGE_STORAGE_BUFFER_BIT: VkFlags		= 0x0020;
pub const VK_BUFFER_USAGE_INDEX_BUFFER_BIT: VkFlags			= 0x0040;
pub const VK_BUFFER_USAGE_VERTEX_BUFFER_BIT: VkFlags		= 0x0080;
pub const VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT: VkFlags		= 0x0100;

pub const VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT: VkFlags		= 0x01;
pub const VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT: VkFlags		= 0x02;
pub const VK_MEMORY_PROPERTY_HOST_COHERENT_BIT: VkFlags		= 0x04;
pub const VK_MEMORY_PROPERTY_HOST_CACHED_BIT: VkFlags		= 0x08;
pub const VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT: VkFlags	= 0x10;

pub const VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT: VkFlags	= 0x01;

#[repr(C)] #[derive(Clone, Copy)] pub enum VkDescriptorType
{
	Sampler = 0,
	CombinedImageSampler = 1,
	SampledImage = 2,
	StorageImage = 3,
	UniformTexelBuffer = 4,
	StorageTexelBuffer = 5,
	UniformBuffer = 6,
	StorageBuffer = 7,
	UniformBufferDynamic = 8,
	StorageBufferDynamic = 9,
	InputAttachment = 10
}
#[repr(C)] #[derive(PartialEq, Eq, Debug)] pub enum VkFilter
{
	Nearest = 0, Linear = 1,
	CubicImg = 1000015000
}
#[repr(C)] pub enum VkSamplerMipmapMode
{
	Nearest = 0, Linear = 1
}
#[repr(C)] pub enum VkSamplerAddressMode
{
	Repeat = 0, MirroredRepeat = 1,
	ClampToEdge = 2, ClampToBorder = 3,
	MirrorClampToEdge = 4
}
#[repr(C)] pub enum VkBorderColor
{
	FloatTransparentBlack = 0, IntTransparentBlack = 1,
	FloatOpaqueBlack = 2, IntOpaqueBlack = 3,
	FloatOpaqueWhite = 4, IntOpaqueWhite = 5
}

#[repr(C)] #[derive(Clone, Copy)]
pub enum VkColorSpaceKHR
{
	SRGB_NonLinear = 0
}
#[repr(C)] #[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VkPresentModeKHR
{
	Immediate = 0,
	Mailbox = 1,
	FIFO = 2,
	FIFORelaxed = 3
}
// VkSurfaceTransformFlagBitsKHR //
pub const VK_SURFACE_TRANSFORM_IDENTITY_BIT: VkFlags                    = 0x0001;
pub const VK_SURFACE_TRANSFORM_ROTATE90_BIT: VkFlags                    = 0x0002;
pub const VK_SURFACE_TRANSFORM_ROTATE180_BIT: VkFlags                   = 0x0004;
pub const VK_SURFACE_TRANSFORM_ROTATE270_BIT: VkFlags                   = 0x0008;
pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_BIT: VkFlags           = 0x0010;
pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE90_BIT: VkFlags  = 0x0020;
pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE180_BIT: VkFlags = 0x0040;
pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE270_BIT: VkFlags = 0x0080;
pub const VK_SURFACE_TRANSFORM_INHERIT_BIT: VkFlags                     = 0x0100;
// VkCompositeAlphaFlagBitsKHR //
pub const VK_COMPOSITE_ALPHA_OPAQUE_BIT: VkFlags            = 0x01;
pub const VK_COMPOSITE_ALPHA_PREMULTIPLIED_BIT: VkFlags     = 0x02;
pub const VK_COMPOSITE_ALPHA_POSTMULTIPLIED_BIT: VkFlags    = 0x04;
pub const VK_COMPOSSITE_ALPHA_INHERIT: VkFlags              = 0x08;

#[derive(Debug)]
#[repr(C)] pub enum VkDebugReportObjectTypeEXT
{
	Unknown = 0, Instance = 1,
	PhysicalDevice = 2, Device = 3,
	Queue = 4, Semaphore = 5, CommandBuffer = 6,
	Fence = 7, DeviceMemory = 8, Buffer = 9, Image = 10, Event = 11,
	QueryPool = 12, BufferView = 13, ImageView = 14, ShaderModule = 15,
	PipelineCache = 16, PipelineLayout = 17, RenderPass = 18, Pipeline = 19,
	DescriptorSetLayout = 20, Sampler = 21, DescriptorPool = 22, DescriptorSet = 23,
	Framebuffer = 24, CommandPool = 25, SurfaceKHR = 26, SwapchainKHR = 27, DebugReportEXT = 28
}
#[repr(C)] pub enum VkDebugReportErrorEXT
{
	_None = 0, CallbackRef = 1
}
pub const VK_DEBUG_REPORT_INFORMATION_BIT_EXT: VkFlags			= 0x01;
pub const VK_DEBUG_REPORT_WARNING_BIT_EXT: VkFlags				= 0x02;
pub const VK_DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT: VkFlags	= 0x04;
pub const VK_DEBUG_REPORT_ERROR_BIT_EXT: VkFlags				= 0x08;
pub const VK_DEBUG_REPORT_DEBUG_BIT_EXT: VkFlags				= 0x10;

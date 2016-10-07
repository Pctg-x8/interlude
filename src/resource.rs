// Interlude: Resources(Buffer and Image)

#![allow(dead_code)]

use super::internals::*;
use {std, vk};
use vk::ffi::*;
use vk::traits::*;
use std::os::raw::c_void;
use std::rc::Rc;

pub trait Resource { fn get_memory_requirements(&self) -> VkMemoryRequirements; }
pub trait BufferResource { fn get_resource(&self) -> VkBuffer; }
pub trait ImageResource { fn get_resource(&self) -> VkImage; }
pub trait DescriptedImage : std::marker::Sized { fn new(engine: &Engine, desc: &VkImageCreateInfo) -> Result<Self, EngineError>; }

pub trait BufferInternals : std::marker::Sized
{
	fn new(engine: &Engine, size: VkDeviceSize, usage: VkBufferUsageFlags) -> Result<Self, EngineError>;
}
pub trait LinearImage2DInternals : std::marker::Sized
{
	fn new(engine: &Engine, size: VkExtent2D, format: VkFormat) -> Result<Self, EngineError>;
}
pub struct Buffer { internal: vk::Buffer, size: VkDeviceSize }
impl Resource for Buffer { fn get_memory_requirements(&self) -> VkMemoryRequirements { self.internal.get_memory_requirements() } }
impl BufferResource for Buffer { fn get_resource(&self) -> VkBuffer { self.internal.get() } }
pub struct Image1D { internal: vk::Image, size: u32 }
pub struct Image2D { internal: vk::Image, size: VkExtent2D }
pub struct LinearImage2D { internal: vk::Image, size: VkExtent2D }
pub struct Image3D { internal: vk::Image, size: VkExtent3D }
impl Resource for Image1D { fn get_memory_requirements(&self) -> VkMemoryRequirements { self.internal.get_memory_requirements() } }
impl Resource for Image2D { fn get_memory_requirements(&self) -> VkMemoryRequirements { self.internal.get_memory_requirements() } }
impl Resource for LinearImage2D { fn get_memory_requirements(&self) -> VkMemoryRequirements { self.internal.get_memory_requirements() } }
impl Resource for Image3D { fn get_memory_requirements(&self) -> VkMemoryRequirements { self.internal.get_memory_requirements() } }
impl ImageResource for Image1D { fn get_resource(&self) -> VkImage { self.internal.get() } }
impl ImageResource for Image2D { fn get_resource(&self) -> VkImage { self.internal.get() } }
impl ImageResource for LinearImage2D { fn get_resource(&self) -> VkImage { self.internal.get() } }
impl ImageResource for Image3D { fn get_resource(&self) -> VkImage { self.internal.get() } }
impl InternalExports<vk::Image> for Image1D { fn get_internal(&self) -> &vk::Image { &self.internal } }
impl InternalExports<vk::Image> for Image2D { fn get_internal(&self) -> &vk::Image { &self.internal } }
impl InternalExports<vk::Image> for LinearImage2D { fn get_internal(&self) -> &vk::Image { &self.internal } }
impl InternalExports<vk::Image> for Image3D { fn get_internal(&self) -> &vk::Image { &self.internal } }
impl BufferInternals for Buffer
{
	fn new(engine: &Engine, size: VkDeviceSize, usage: VkBufferUsageFlags) -> Result<Self, EngineError>
	{
		vk::Buffer::new(engine.get_device().get_internal(), &VkBufferCreateInfo
		{
			sType: VkStructureType::BufferCreateInfo, pNext: std::ptr::null(), flags: 0,
			size: size, usage: usage, sharingMode: VkSharingMode::Exclusive,
			queueFamilyIndexCount: 0, pQueueFamilyIndices: std::ptr::null(),
		}).map(|buf| Buffer { internal: buf, size: size }).map_err(EngineError::from)
	}
}
impl LinearImage2DInternals for LinearImage2D
{
	fn new(engine: &Engine, size: VkExtent2D, format: VkFormat) -> Result<Self, EngineError>
	{
		let VkExtent2D(width, height) = size;
		vk::Image::new(engine.get_device().get_internal(), &VkImageCreateInfo
		{
			sType: VkStructureType::ImageCreateInfo, pNext: std::ptr::null(), flags: 0,
			imageType: VkImageType::Dim2, format: format, extent: VkExtent3D(width, height, 1),
			mipLevels: 1, arrayLayers: 1, samples: VK_SAMPLE_COUNT_1_BIT, tiling: VkImageTiling::Linear,
			usage: VK_IMAGE_USAGE_TRANSFER_SRC_BIT, sharingMode: VkSharingMode::Exclusive,
			initialLayout: VkImageLayout::Preinitialized,
			queueFamilyIndexCount: 0, pQueueFamilyIndices: std::ptr::null()
		}).map(|img| LinearImage2D { internal: img, size: size }).map_err(EngineError::from)
	}
}
impl DescriptedImage for Image1D
{
	fn new(engine: &Engine, desc: &VkImageCreateInfo) -> Result<Self, EngineError>
	{
		vk::Image::new(engine.get_device().get_internal(), &VkImageCreateInfo
		{
			usage: desc.usage | VK_IMAGE_USAGE_TRANSFER_DST_BIT,
			.. *desc
		}).map(|img| Image1D { internal: img, size: desc.extent.0 }).map_err(EngineError::from)
	}
}
impl DescriptedImage for Image2D
{
	fn new(engine: &Engine, desc: &VkImageCreateInfo) -> Result<Self, EngineError>
	{
		vk::Image::new(engine.get_device().get_internal(), &VkImageCreateInfo
		{
			usage: desc.usage | VK_IMAGE_USAGE_TRANSFER_DST_BIT,
			.. *desc
		}).map(|img| Image2D { internal: img, size: VkExtent2D(desc.extent.0, desc.extent.1) }).map_err(EngineError::from)
	}
}
impl DescriptedImage for Image3D
{
	fn new(engine: &Engine, desc: &VkImageCreateInfo) -> Result<Self, EngineError>
	{
		vk::Image::new(engine.get_device().get_internal(), &VkImageCreateInfo
		{
			usage: desc.usage | VK_IMAGE_USAGE_TRANSFER_DST_BIT,
			.. *desc
		}).map(|img| Image3D { internal: img, size: desc.extent }).map_err(EngineError::from)
	}
}
pub enum SampleCount { Bit1, Bit2, Bit4, Bit8, Bit16, Bit32, Bit64 }
pub trait ImageDescriptor : std::marker::Sized + InternalExports<VkImageCreateInfo>
{
	fn mip_levels(mut self, levels: u32) -> Self;
	fn array_layers(mut self, layers: u32) -> Self;
	fn sample_flags(mut self, samples: &[SampleCount]) -> Self;
}
pub struct ImageDescriptor1 { internal: VkImageCreateInfo, device_resource: bool }
pub struct ImageDescriptor2 { internal: VkImageCreateInfo, device_resource: bool }
pub struct ImageDescriptor3 { internal: VkImageCreateInfo }
impl ImageDescriptor1
{
	pub fn new(format: VkFormat, extent: u32, usage: VkImageUsageFlags) -> Self
	{
		ImageDescriptor1
		{
			internal: VkImageCreateInfo
			{
				sType: VkStructureType::ImageCreateInfo, pNext: std::ptr::null(), flags: 0,
				imageType: VkImageType::Dim1, format: format, extent: VkExtent3D(extent, 1, 1),
				mipLevels: 1, arrayLayers: 1, samples: VK_SAMPLE_COUNT_1_BIT, tiling: VkImageTiling::Optimal,
				usage: usage, sharingMode: VkSharingMode::Exclusive, initialLayout: VkImageLayout::Preinitialized,
				queueFamilyIndexCount: 0, pQueueFamilyIndices: std::ptr::null()
			}, device_resource: false
		}
	}
	pub fn device_resource(mut self) -> Self { self.device_resource = true; self }
	pub fn is_device_resource(&self) -> bool { self.device_resource }
}
impl ImageDescriptor2
{
	pub fn new(format: VkFormat, extent: VkExtent2D, usage: VkImageUsageFlags) -> Self
	{
		let VkExtent2D(width, height) = extent;
		ImageDescriptor2
		{
			internal: VkImageCreateInfo
			{
				sType: VkStructureType::ImageCreateInfo, pNext: std::ptr::null(), flags: 0,
				imageType: VkImageType::Dim2, format: format, extent: VkExtent3D(width, height, 1),
				mipLevels: 1, arrayLayers: 1, samples: VK_SAMPLE_COUNT_1_BIT, tiling: VkImageTiling::Optimal,
				usage: usage, sharingMode: VkSharingMode::Exclusive, initialLayout: VkImageLayout::Preinitialized,
				queueFamilyIndexCount: 0, pQueueFamilyIndices: std::ptr::null()
			},
			device_resource: false
		}
	}
	pub fn device_resource(mut self) -> Self
	{
		self.device_resource = true;
		self
	}
	pub fn is_device_resource(&self) -> bool { self.device_resource }
}
impl ImageDescriptor3
{
	pub fn new(format: VkFormat, extent: VkExtent3D, usage: VkImageUsageFlags) -> Self
	{
		ImageDescriptor3
		{
			internal: VkImageCreateInfo
			{
				sType: VkStructureType::ImageCreateInfo, pNext: std::ptr::null(), flags: 0,
				imageType: VkImageType::Dim3, format: format, extent: extent,
				mipLevels: 1, arrayLayers: 1, samples: VK_SAMPLE_COUNT_1_BIT, tiling: VkImageTiling::Optimal,
				usage: usage, sharingMode: VkSharingMode::Exclusive, initialLayout: VkImageLayout::Preinitialized,
				queueFamilyIndexCount: 0, pQueueFamilyIndices: std::ptr::null()
			}
		}
	}
}
impl InternalExports<VkImageCreateInfo> for ImageDescriptor1 { fn get_internal(&self) -> &VkImageCreateInfo { &self.internal } }
impl InternalExports<VkImageCreateInfo> for ImageDescriptor2 { fn get_internal(&self) -> &VkImageCreateInfo { &self.internal } }
impl InternalExports<VkImageCreateInfo> for ImageDescriptor3 { fn get_internal(&self) -> &VkImageCreateInfo { &self.internal } }
macro_rules! ImplImageDescriptor
{
	(for $t: ty) =>
	{
		impl ImageDescriptor for $t
		{
			fn mip_levels(mut self, levels: u32) -> Self
			{
				self.internal.mipLevels = levels;
				self
			}
			fn array_layers(mut self, layers: u32) -> Self
			{
				self.internal.arrayLayers = layers;
				self
			}
			fn sample_flags(mut self, samples: &[SampleCount]) -> Self
			{
				self.internal.samples = samples.into_iter().fold(0, |flags, c| match c
				{
					&SampleCount::Bit1 => flags | VK_SAMPLE_COUNT_1_BIT,
					&SampleCount::Bit2 => flags | VK_SAMPLE_COUNT_2_BIT,
					&SampleCount::Bit4 => flags | VK_SAMPLE_COUNT_4_BIT,
					&SampleCount::Bit8 => flags | VK_SAMPLE_COUNT_8_BIT,
					&SampleCount::Bit16 => flags | VK_SAMPLE_COUNT_16_BIT,
					&SampleCount::Bit32 => flags | VK_SAMPLE_COUNT_32_BIT,
					&SampleCount::Bit64 => flags | VK_SAMPLE_COUNT_64_BIT
				});
				self
			}
		}
	}
}
ImplImageDescriptor!(for ImageDescriptor1);
ImplImageDescriptor!(for ImageDescriptor2);
ImplImageDescriptor!(for ImageDescriptor3);

#[derive(Clone, Copy)]
pub struct ImageSubresourceRange(VkImageAspectFlags, u32, u32, u32, u32);
impl ImageSubresourceRange
{
	pub fn base_color() -> Self
	{
		ImageSubresourceRange(VK_IMAGE_ASPECT_COLOR_BIT, 0, 1, 0, 1)
	}
}
impl std::convert::Into<VkImageSubresourceRange> for ImageSubresourceRange
{
	fn into(self) -> VkImageSubresourceRange { (&self).into() }
}
impl <'a> std::convert::Into<VkImageSubresourceRange> for &'a ImageSubresourceRange
{
	fn into(self) -> VkImageSubresourceRange
	{
		let ImageSubresourceRange(aspect, base_mip, level_count, base_array, layer_count) = *self;
		VkImageSubresourceRange
		{
			aspectMask: aspect,
			baseMipLevel: base_mip, levelCount: level_count,
			baseArrayLayer: base_array, layerCount: layer_count
		}
	}
}

#[derive(Clone, Copy)]
pub struct ImageSubresourceLayers(VkImageAspectFlags, u32, u32, u32);
impl ImageSubresourceLayers
{
	pub fn base_color() -> Self
	{
		ImageSubresourceLayers(VK_IMAGE_ASPECT_COLOR_BIT, 0, 0, 1)
	}
}
impl std::convert::Into<VkImageSubresourceLayers> for ImageSubresourceLayers
{
	fn into(self) -> VkImageSubresourceLayers
	{
		let ImageSubresourceLayers(aspect, base_mip, base_array, count) = self;
		VkImageSubresourceLayers(aspect, base_mip, base_array, count)
	}
}

#[derive(Clone, Copy)]
pub enum BufferDataType
{
	Vertex, Index, Uniform, IndirectCallParam
}
pub struct BufferPreallocator
{
	usage_flags: VkBufferUsageFlags, offsets: Vec<usize>
}
pub trait BufferPreallocatorInternals
{
	fn new(usage: VkBufferUsageFlags, offsets: Vec<usize>) -> Self;
	fn get_usage(&self) -> VkBufferUsageFlags;
}
impl BufferPreallocatorInternals for BufferPreallocator
{
	fn new(usage: VkBufferUsageFlags, offsets: Vec<usize>) -> Self { BufferPreallocator { usage_flags: usage, offsets: offsets } }
	fn get_usage(&self) -> VkBufferUsageFlags { self.usage_flags }
}
impl BufferPreallocator
{
	pub fn offset(&self, index: usize) -> usize { self.offsets[index] }
	pub fn total_size(&self) -> usize { self.offsets.last().map(|&x| x).unwrap_or(0) }
}

#[allow(non_snake_case)]
pub mod ImageUsagePresets
{
	#![allow(non_upper_case_globals)]
	use vk::ffi::*;
	
	pub const AsColorTexture: VkImageUsageFlags = VK_IMAGE_USAGE_SAMPLED_BIT | VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT;
}

pub struct ImagePreallocator<'a>
{
	dim1_images: Vec<&'a ImageDescriptor1>,
	dim2_images: Vec<&'a ImageDescriptor2>,
	dim3_images: Vec<&'a ImageDescriptor3>
}
impl <'a> ImagePreallocator<'a>
{
	pub fn new() -> Self
	{
		ImagePreallocator { dim1_images: Vec::new(), dim2_images: Vec::new(), dim3_images: Vec::new() }
	}
	pub fn image_1d(mut self, i1ds: Vec<&'a ImageDescriptor1>) -> Self
	{
		self.dim1_images = i1ds;
		self
	}
	pub fn image_2d(mut self, i2ds: Vec<&'a ImageDescriptor2>) -> Self
	{
		self.dim2_images = i2ds;
		self
	}
	pub fn image_3d(mut self, i3ds: Vec<&'a ImageDescriptor3>) -> Self
	{
		self.dim3_images = i3ds;
		self
	}
}
pub trait ImagePreallocatorInternals<'a>
{
	fn dim1_images(&self) -> &[&'a ImageDescriptor1];
	fn dim2_images(&self) -> &[&'a ImageDescriptor2];
	fn dim3_images(&self) -> &[&'a ImageDescriptor3];
}
impl <'a> ImagePreallocatorInternals<'a> for ImagePreallocator<'a>
{
	fn dim1_images(&self) -> &[&'a ImageDescriptor1] { &self.dim1_images }
	fn dim2_images(&self) -> &[&'a ImageDescriptor2] { &self.dim2_images }
	fn dim3_images(&self) -> &[&'a ImageDescriptor3] { &self.dim3_images }
}

pub struct DeviceBuffer
{
	buffer: vk::Buffer, memory: vk::DeviceMemory, size: VkDeviceSize
}
pub trait DeviceBufferInternals : std::marker::Sized
{
	fn new(engine: &Engine, size: VkDeviceSize, usage: VkBufferUsageFlags) -> Result<Self, EngineError>;
}
impl DeviceBufferInternals for DeviceBuffer
{
	fn new(engine: &Engine, size: VkDeviceSize, usage: VkBufferUsageFlags) -> Result<Self, EngineError>
	{
		vk::Buffer::new(engine.get_device().get_internal(), &VkBufferCreateInfo
		{
			sType: VkStructureType::BufferCreateInfo, pNext: std::ptr::null(), flags: 0,
			size: size, usage: usage | VK_BUFFER_USAGE_TRANSFER_DST_BIT, sharingMode: VkSharingMode::Exclusive,
			queueFamilyIndexCount: 0, pQueueFamilyIndices: std::ptr::null()
		}).and_then(|buffer| vk::DeviceMemory::alloc(engine.get_device().get_internal(), &VkMemoryAllocateInfo
		{
			sType: VkStructureType::MemoryAllocateInfo, pNext: std::ptr::null(),
			allocationSize: buffer.get_memory_requirements().size, memoryTypeIndex: engine.get_memory_type_index_for_device_local()
		}).map(move |memory| (buffer, memory))).and_then(|(buffer, memory)| memory.bind_buffer(&buffer, 0)
			.map(move |()| DeviceBuffer { buffer: buffer, memory: memory, size: size }))
		.map_err(EngineError::from)
	}
}
impl BufferResource for DeviceBuffer
{
	fn get_resource(&self) -> VkBuffer { self.buffer.get() }
}
pub struct StagingBuffer
{
	buffer: vk::Buffer, memory: vk::DeviceMemory, size: VkDeviceSize
}
pub trait StagingBufferInternals : std::marker::Sized
{
	fn new(engine: &Engine, size: VkDeviceSize) -> Result<Self, EngineError>;
}
impl StagingBufferInternals for StagingBuffer
{
	fn new(engine: &Engine, size: VkDeviceSize) -> Result<Self, EngineError>
	{
		vk::Buffer::new(engine.get_device().get_internal(), &VkBufferCreateInfo
		{
			sType: VkStructureType::BufferCreateInfo, pNext: std::ptr::null(), flags: 0,
			size: size, usage: VK_BUFFER_USAGE_TRANSFER_SRC_BIT, sharingMode: VkSharingMode::Exclusive,
			queueFamilyIndexCount: 0, pQueueFamilyIndices: std::ptr::null()
		}).and_then(|buffer| vk::DeviceMemory::alloc(engine.get_device().get_internal(), &VkMemoryAllocateInfo
		{
			sType: VkStructureType::MemoryAllocateInfo, pNext: std::ptr::null(),
			allocationSize: buffer.get_memory_requirements().size, memoryTypeIndex: engine.get_memory_type_index_for_host_visible()
		}).map(move |memory| (buffer, memory))).and_then(|(buffer, memory)| memory.bind_buffer(&buffer, 0)
			.map(move |()| StagingBuffer { buffer: buffer, memory: memory, size: size }))
		.map_err(EngineError::from)
	}
}
impl BufferResource for StagingBuffer
{
	fn get_resource(&self) -> VkBuffer { self.buffer.get() }
}
impl StagingBuffer
{
	pub fn map(&self) -> Result<MemoryMappedRange, EngineError>
	{
		self.memory.map(0 .. self.size).map(|ptr| MemoryMappedRange { parent: self, ptr: ptr }).map_err(EngineError::from)
	}
}

pub struct DeviceImage
{
	dim1: Vec<Rc<Image1D>>, dim2: Vec<Rc<Image2D>>, dim3: Vec<Rc<Image3D>>,
	memory: vk::DeviceMemory, size: VkDeviceSize
}
pub trait DeviceImageInternals : std::marker::Sized
{
	fn new(engine: &Engine, d1_images: Vec<Image1D>, d2_images: Vec<Image2D>, d3_images: Vec<Image3D>)
		-> Result<Self, EngineError>;
}
impl DeviceImageInternals for DeviceImage
{
	fn new(engine: &Engine, d1_images: Vec<Image1D>, d2_images: Vec<Image2D>, d3_images: Vec<Image3D>)
		-> Result<Self, EngineError>
	{
		let image_offsets = {
			let d1_image_requirements = d1_images.iter().map(|b| b.get_memory_requirements());
			let d2_image_requirements = d2_images.iter().map(|b| b.get_memory_requirements());
			let d3_image_requirements = d3_images.iter().map(|b| b.get_memory_requirements());
			
			d1_image_requirements.chain(d2_image_requirements).chain(d3_image_requirements)
				.chain([VkMemoryRequirements { size: 0, alignment: 1, memoryTypeBits: 0 }].into_iter().map(|&x| x)).scan(0, |offs, req|
				{
					let current_offs = ((*offs as f64 / req.alignment as f64).ceil() as VkDeviceSize) * req.alignment;
					*offs = current_offs + req.size;
					Some(current_offs)
				}).collect::<Vec<_>>()
		};
		let memory_size = try!(image_offsets.last().map(|&x| x).ok_or(EngineError::AllocateMemoryWithEmptyResources));
		info!(target: "Prelude::Resource", "Going to allocate buffer for device {} bytes", memory_size);

		vk::DeviceMemory::alloc(engine.get_device().get_internal(), &VkMemoryAllocateInfo
		{
			sType: VkStructureType::MemoryAllocateInfo, pNext: std::ptr::null(),
			allocationSize: memory_size, memoryTypeIndex: engine.get_memory_type_index_for_device_local()
		}).and_then(|memory|
		{
			let image_resources = d1_images.iter().map(|i| i as &InternalExports<vk::Image>)
				.chain(d2_images.iter().map(|i| i as &InternalExports<vk::Image>))
				.chain(d3_images.iter().map(|i| i as &InternalExports<vk::Image>));
			
			for (&offs, res) in image_offsets.iter().zip(image_resources)
			{
				try!(memory.bind_image(res.get_internal(), offs));
			}
			Ok(memory)
		}).map(move |memory| DeviceImage
		{
			dim1: d1_images.into_iter().map(Rc::new).collect(),
			dim2: d2_images.into_iter().map(Rc::new).collect(),
			dim3: d3_images.into_iter().map(Rc::new).collect(),
			memory: memory, size: memory_size
		}).map_err(EngineError::from)
	}
}
impl DeviceImage
{
	pub fn dim1vec(&self) -> &Vec<Rc<Image1D>> { &self.dim1 }
	pub fn dim2vec(&self) -> &Vec<Rc<Image2D>> { &self.dim2 }
	pub fn dim3vec(&self) -> &Vec<Rc<Image3D>> { &self.dim3 }
	pub fn dim2(&self, index: usize) -> &Rc<Image2D>
	{
		&self.dim2[index]
	}
}

pub struct StagingImage
{
	linear_dim2_images: Vec<LinearImage2D>, linear_dim2_images_offset: Vec<VkDeviceSize>,
	memory: vk::DeviceMemory, size: VkDeviceSize
}
pub trait StagingImageInternals : std::marker::Sized
{
	fn new(engine: &Engine, ld2_images: Vec<LinearImage2D>) -> Result<Self, EngineError>;
}
impl StagingImageInternals for StagingImage
{
	fn new(engine: &Engine, ld2_images: Vec<LinearImage2D>) -> Result<Self, EngineError>
	{
		let image_offsets =
		{
			let ld2_image_requirements = ld2_images.iter().map(|b| b.get_memory_requirements());

			ld2_image_requirements.chain([VkMemoryRequirements { size: 0, alignment: 1, memoryTypeBits: 0 }].into_iter().map(|&x| x))
				.scan(0, |offs, req|
				{
					let current_offs = ((*offs as f64 / req.alignment as f64).ceil() as VkDeviceSize) * req.alignment;
					*offs = current_offs + req.size;
					Some(current_offs)
				}).collect::<Vec<_>>()
		};
		let memory_size = try!(image_offsets.last().map(|&x| x).ok_or(EngineError::AllocateMemoryWithEmptyResources));
		info!(target: "Prelude::Resource", "Going to allocate buffer for host {} bytes", memory_size);

		vk::DeviceMemory::alloc(engine.get_device().get_internal(), &VkMemoryAllocateInfo
		{
			sType: VkStructureType::MemoryAllocateInfo, pNext: std::ptr::null(),
			allocationSize: memory_size, memoryTypeIndex: engine.get_memory_type_index_for_host_visible()
		}).and_then(|memory|
		{
			for (&offs, res) in image_offsets.iter().zip(ld2_images.iter())
			{
				try!(memory.bind_image(res.get_internal(), offs));
			}
			Ok(memory)
		}).map(move |memory| StagingImage
		{
			linear_dim2_images: ld2_images, linear_dim2_images_offset: image_offsets,
			memory: memory, size: memory_size
		}).map_err(EngineError::from)
	}
}
impl StagingImage
{
	pub fn map(&self) -> Result<MemoryMappedRange, EngineError>
	{
		self.memory.map(0 .. self.size).map(|ptr| MemoryMappedRange { parent: self, ptr: ptr }).map_err(EngineError::from)
	}
	pub fn dim2(&self, index: usize) -> &LinearImage2D
	{
		&self.linear_dim2_images[index]
	}
	pub fn image2d_offset(&self, index: usize) -> VkDeviceSize
	{
		self.linear_dim2_images_offset[index]
	}
	pub fn image2d_offsets(&self) -> &Vec<VkDeviceSize>
	{
		&self.linear_dim2_images_offset
	}
	pub fn dim2vec(&self) -> &Vec<LinearImage2D> { &self.linear_dim2_images }
	pub fn size(&self) -> VkDeviceSize { self.size }
}

pub trait StagingResource
{
	fn unmap(&self);
}
impl StagingResource for StagingBuffer
{
	fn unmap(&self)
	{
		self.memory.unmap()
	}
}
impl StagingResource for StagingImage
{
	fn unmap(&self)
	{
		self.memory.unmap()
	}
}

pub struct MemoryMappedRange<'a>
{
	parent: &'a StagingResource, ptr: *mut c_void
}
impl <'a> MemoryMappedRange<'a>
{
	pub fn map_mut<MappedStructureT>(&self, offset: usize) -> &'a mut MappedStructureT
	{
		let t: &mut MappedStructureT = unsafe { std::mem::transmute(std::mem::transmute::<_, usize>(self.ptr) + offset) };
		t
	}
	pub fn range_mut<MappedStructureT>(&self, offset: usize, count: usize) -> &'a mut [MappedStructureT]
	{
		unsafe { std::slice::from_raw_parts_mut(self.ptr.offset(offset as isize) as *mut MappedStructureT, count) }
	}
	pub unsafe fn into_raw(mut self) -> *mut c_void
	{
		let ret = self.ptr;
		self.ptr = std::ptr::null_mut();
		ret
	}
	pub unsafe fn from_raw(parent: &'a StagingResource, ptr: *mut c_void) -> Self
	{
		MemoryMappedRange { parent: parent, ptr: ptr }
	}
}
impl <'a> std::ops::Drop for MemoryMappedRange<'a>
{
	fn drop(&mut self) { if !self.ptr.is_null() { self.parent.unmap(); } }
}

#[derive(Clone, Copy, Debug)]
pub enum Filter
{
	Nearest, Linear
}
impl std::convert::Into<VkFilter> for Filter
{
	fn into(self) -> VkFilter
	{
		match self
		{
			Filter::Nearest => VkFilter::Nearest,
			Filter::Linear => VkFilter::Linear
		}
	}
}
#[derive(Clone, Copy)]
pub enum AddressMode
{
	Clamp, Repeat, Mirror
}
impl std::convert::Into<VkSamplerAddressMode> for AddressMode
{
	fn into(self) -> VkSamplerAddressMode
	{
		match self
		{
			AddressMode::Clamp => VkSamplerAddressMode::ClampToBorder,
			AddressMode::Repeat => VkSamplerAddressMode::Repeat,
			AddressMode::Mirror => VkSamplerAddressMode::MirroredRepeat
		}
	}
}
pub struct SamplerState
{
	mag_filter: Filter, min_filter: Filter,
	address_mode: (AddressMode, AddressMode, AddressMode),
	max_anisotropy: Option<f32>
}
impl SamplerState
{
	pub fn new() -> Self
	{
		SamplerState
		{
			mag_filter: Filter::Linear, min_filter: Filter::Linear,
			address_mode: (AddressMode::Clamp, AddressMode::Clamp, AddressMode::Clamp),
			max_anisotropy: None
		}
	}
	pub fn filters(mut self, mag_filter: Filter, min_filter: Filter) -> Self
	{
		self.mag_filter = mag_filter; self.min_filter = min_filter;
		self
	}
}
impl <'a> std::convert::Into<VkSamplerCreateInfo> for &'a SamplerState
{
	fn into(self) -> VkSamplerCreateInfo
	{
		let (amu, amv, amw) = self.address_mode;
		VkSamplerCreateInfo
		{
			sType: VkStructureType::SamplerCreateInfo, pNext: std::ptr::null(), flags: 0,
			magFilter: self.mag_filter.into(), minFilter: self.min_filter.into(), mipmapMode: VkSamplerMipmapMode::Linear,
			addressModeU: amu.into(), addressModeV: amv.into(), addressModeW: amw.into(),
			mipLodBias: 0.0f32, anisotropyEnable: self.max_anisotropy.is_some() as VkBool32, maxAnisotropy: self.max_anisotropy.unwrap_or(0.0f32),
			compareEnable: false as VkBool32, compareOp: VkCompareOp::Never, minLod: 0.0f32, maxLod: 1.0f32,
			borderColor: VkBorderColor::FloatTransparentBlack, unnormalizedCoordinates: false as VkBool32
		}
	}
}

pub struct Sampler
{
	internal: vk::Sampler
}
pub trait SamplerInternals : std::marker::Sized
{
	fn new(engine: &Engine, info: &VkSamplerCreateInfo) -> Result<Self, EngineError>;
	fn get_native(&self) -> VkSampler;
}
impl SamplerInternals for Sampler
{
	fn new(engine: &Engine, info: &VkSamplerCreateInfo) -> Result<Self, EngineError>
	{
		vk::Sampler::new(engine.get_device().get_internal(), info).map(|s| Sampler { internal: s })
			.map_err(EngineError::from)
	}
	fn get_native(&self) -> VkSampler { self.internal.get() }
}

#[derive(Clone, Copy, Debug)]
pub enum ComponentSwizzle { R, G, B, A }
impl std::convert::Into<VkComponentSwizzle> for ComponentSwizzle
{
	fn into(self) -> VkComponentSwizzle
	{
		match self
		{
			ComponentSwizzle::R => VkComponentSwizzle::R,
			ComponentSwizzle::G => VkComponentSwizzle::G,
			ComponentSwizzle::B => VkComponentSwizzle::B,
			ComponentSwizzle::A => VkComponentSwizzle::A,
		}
	}
}
#[derive(Clone, Copy, Debug)]
pub struct ComponentMapping(pub ComponentSwizzle, pub ComponentSwizzle, pub ComponentSwizzle, pub ComponentSwizzle);
impl ComponentMapping
{
	pub fn single_swizzle(org: ComponentSwizzle) -> Self
	{
		ComponentMapping(org, org, org, org)
	}
	pub fn double_swizzle_rep(org1: ComponentSwizzle, org2: ComponentSwizzle) -> Self
	{
		ComponentMapping(org1, org2, org1, org2)
	}
	pub fn straight() -> Self
	{
		ComponentMapping(ComponentSwizzle::R, ComponentSwizzle::G, ComponentSwizzle::B, ComponentSwizzle::A)
	}
	pub fn reversed() -> Self
	{
		ComponentMapping(ComponentSwizzle::B, ComponentSwizzle::G, ComponentSwizzle::R, ComponentSwizzle::A)
	}
}
impl std::convert::Into<VkComponentMapping> for ComponentMapping
{
	fn into(self) -> VkComponentMapping
	{
		let ComponentMapping(r, g, b, a) = self;
		VkComponentMapping
		{
			r: r.into(), g: g.into(), b: b.into(), a: a.into()
		}
	}
}
pub struct ImageView1D { parent: Rc<Image1D>, internal: vk::ImageView }
pub struct ImageView2D { parent: Rc<Image2D>, internal: vk::ImageView }
pub struct ImageView3D { parent: Rc<Image3D>, internal: vk::ImageView }
pub trait ImageViewFactory<ResourceT: ImageResource>: std::marker::Sized
{
	fn new(engine: &Engine, res: &Rc<ResourceT>, format: VkFormat, cm: ComponentMapping, subrange: ImageSubresourceRange) -> Result<Self, EngineError>;
}
impl ImageViewFactory<Image1D> for ImageView1D
{
	fn new(engine: &Engine, res: &Rc<Image1D>, format: VkFormat, cm: ComponentMapping, subrange: ImageSubresourceRange) -> Result<Self, EngineError>
	{
		vk::ImageView::new(engine.get_device().get_internal(), &VkImageViewCreateInfo
		{
			sType: VkStructureType::ImageViewCreateInfo, pNext: std::ptr::null(), flags: 0,
			image: res.get_resource(), viewType: VkImageViewType::Dim1, format: format,
			components: cm.into(), subresourceRange: subrange.into()	
		}).map(|v| ImageView1D { parent: res.clone(), internal: v }).map_err(EngineError::from)
	}
}
impl ImageViewFactory<Image2D> for ImageView2D
{
	fn new(engine: &Engine, res: &Rc<Image2D>, format: VkFormat, cm: ComponentMapping, subrange: ImageSubresourceRange) -> Result<Self, EngineError>
	{
		vk::ImageView::new(engine.get_device().get_internal(), &VkImageViewCreateInfo
		{
			sType: VkStructureType::ImageViewCreateInfo, pNext: std::ptr::null(), flags: 0,
			image: res.get_resource(), viewType: VkImageViewType::Dim2, format: format,
			components: cm.into(), subresourceRange: subrange.into()	
		}).map(|v| ImageView2D { parent: res.clone(), internal: v }).map_err(EngineError::from)
	}
}
impl ImageViewFactory<Image3D> for ImageView3D
{
	fn new(engine: &Engine, res: &Rc<Image3D>, format: VkFormat, cm: ComponentMapping, subrange: ImageSubresourceRange) -> Result<Self, EngineError>
	{
		vk::ImageView::new(engine.get_device().get_internal(), &VkImageViewCreateInfo
		{
			sType: VkStructureType::ImageViewCreateInfo, pNext: std::ptr::null(), flags: 0,
			image: res.get_resource(), viewType: VkImageViewType::Dim3, format: format,
			components: cm.into(), subresourceRange: subrange.into()	
		}).map(|v| ImageView3D { parent: res.clone(), internal: v }).map_err(EngineError::from)
	}
}
impl std::ops::Deref for ImageView1D { type Target = Image1D; fn deref(&self) -> &Image1D { self.parent.deref() } }
impl std::ops::Deref for ImageView2D { type Target = Image2D; fn deref(&self) -> &Image2D { self.parent.deref() } }
impl std::ops::Deref for ImageView3D { type Target = Image3D; fn deref(&self) -> &Image3D { self.parent.deref() } }
impl ImageResource for ImageView1D { fn get_resource(&self) -> VkImage { self.parent.get_resource() } }
impl ImageResource for ImageView2D { fn get_resource(&self) -> VkImage { self.parent.get_resource() } }
impl ImageResource for ImageView3D { fn get_resource(&self) -> VkImage { self.parent.get_resource() } }
pub trait ImageView
{
	fn get_native(&self) -> VkImageView;
}
pub trait UserImageView<ResourceT: ImageResource>: ImageView
{
	fn get_resource(&self) -> &Rc<ResourceT>;
}
impl ImageView for ImageView1D
{
	fn get_native(&self) -> VkImageView { self.internal.get() }
}
impl ImageView for ImageView2D
{
	fn get_native(&self) -> VkImageView { self.internal.get() }
}
impl ImageView for ImageView3D
{
	fn get_native(&self) -> VkImageView { self.internal.get() }
}
impl UserImageView<Image1D> for ImageView1D
{
	fn get_resource(&self) -> &Rc<Image1D> { &self.parent }
}
impl UserImageView<Image2D> for ImageView2D
{
	fn get_resource(&self) -> &Rc<Image2D> { &self.parent }
}
impl UserImageView<Image3D> for ImageView3D
{
	fn get_resource(&self) -> &Rc<Image3D> { &self.parent }
}

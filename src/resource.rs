#![allow(dead_code)]

///! Interlude: Resources(Buffer and Image)

use super::internals::*;
use vk::*;
use vk::traits::*;
use {std, vk};
use std::os::raw::c_void;
use std::rc::Rc;
use ginterface::{GraphicsInterface, MemoryIndexType};
use EngineResult;

// Resource DataType //
pub struct Buffer(vk::Buffer, VkDeviceSize);
pub struct Image1D(vk::Image, u32);
pub struct Image2D(vk::Image, Size2);
pub struct Image3D(vk::Image, Size3);
pub struct LinearImage2D(vk::Image, Size2);

/// The trait indicates that type is a resource.
pub trait Resource
{
	type Type;
	fn resource(&self) -> Self::Type;
}
impl Resource for Buffer { type Type = VkBuffer; fn resource(&self) -> VkBuffer { *self.0 } }
impl Resource for Image1D { type Type = VkImage; fn resource(&self) -> VkImage { *self.0 } }
impl Resource for Image2D { type Type = VkImage; fn resource(&self) -> VkImage { *self.0 } }
impl Resource for Image3D { type Type = VkImage; fn resource(&self) -> VkImage { *self.0 } }
impl Resource for LinearImage2D { type Type = VkImage; fn resource(&self) -> VkImage { *self.0 } }

/// The trait indicates that type has a memory requirements
pub trait MemoryRequirements { fn memory_requirements(&self) -> VkMemoryRequirements; }
impl MemoryRequirements for Buffer { fn memory_requirements(&self) -> VkMemoryRequirements { self.0.get_memory_requirements() } }
impl MemoryRequirements for Image1D { fn memory_requirements(&self) -> VkMemoryRequirements { self.0.get_memory_requirements() } }
impl MemoryRequirements for Image2D { fn memory_requirements(&self) -> VkMemoryRequirements { self.0.get_memory_requirements() } }
impl MemoryRequirements for Image3D { fn memory_requirements(&self) -> VkMemoryRequirements { self.0.get_memory_requirements() } }
impl MemoryRequirements for LinearImage2D { fn memory_requirements(&self) -> VkMemoryRequirements { self.0.get_memory_requirements() } }

pub trait BufferResource : Resource { fn size(&self) -> VkDeviceSize; }
pub trait ImageResource : Resource { type Size; fn size(&self) -> &Self::Size; }
impl BufferResource for Buffer { fn size(&self) -> VkDeviceSize { self.1 } }
impl ImageResource for Image1D { type Size = u32; fn size(&self) -> &u32 { &self.1 } }
impl ImageResource for Image2D { type Size = Size2; fn size(&self) -> &Size2 { &self.1 } }
impl ImageResource for Image3D { type Size = Size3; fn size(&self) -> &Size3 { &self.1 } }
impl ImageResource for LinearImage2D { type Size = Size2; fn size(&self) -> &Size2 { &self.1 } }

impl LinearImage2D
{
	fn new(engine: &GraphicsInterface, size: &Size2, format: VkFormat) -> EngineResult<Self>
	{
		let &Size2(w, h) = size;
		vk::Image::new(engine.device(), &VkImageCreateInfo
		{
			sType: VkStructureType::ImageCreateInfo, pNext: std::ptr::null(), flags: 0,
			imageType: VkImageType::Dim2, format: format, extent: VkExtent3D(w, h, 1), mipLevels: 1, arrayLayers: 1,
			samples: VK_SAMPLE_COUNT_1_BIT, tiling: VkImageTiling::Linear,
			usage: VK_IMAGE_USAGE_TRANSFER_SRC_BIT, sharingMode: VkSharingMode::Exclusive, initialLayout: VkImageLayout::Preinitialized,
			queueFamilyIndexCount: 0, pQueueFamilyIndices: std::ptr::null()
		}).map(|o| LinearImage2D(o, size.clone())).map_err(From::from)
	}
}
impl Image1D
{
	fn new(engine: &GraphicsInterface, desc: &VkImageCreateInfo) -> EngineResult<Self>
	{
		vk::Image::new(engine.device(), &VkImageCreateInfo
		{
			usage: desc.usage | VK_IMAGE_USAGE_TRANSFER_DST_BIT, .. *desc
		}).map(|o| Image1D(o, desc.extent.0)).map_err(From::from)
	}
}
impl Image2D
{
	fn new(engine: &GraphicsInterface, desc: &VkImageCreateInfo) -> EngineResult<Self>
	{
		vk::Image::new(engine.device(), &VkImageCreateInfo
		{
			usage: desc.usage | VK_IMAGE_USAGE_TRANSFER_DST_BIT, .. *desc
		}).map(|o| Image2D(o, Size2(desc.extent.0, desc.extent.1))).map_err(From::from)
	}
}
impl Image3D
{
	fn new(engine: &GraphicsInterface, desc: &VkImageCreateInfo) -> EngineResult<Self>
	{
		vk::Image::new(engine.device(), &VkImageCreateInfo
		{
			usage: desc.usage | VK_IMAGE_USAGE_TRANSFER_DST_BIT, .. *desc
		}).map(|o| Image3D(o, desc.extent.into())).map_err(From::from)
	}
}
/// SampleCount: e.g. 1 << 4 = 4 samples
pub type SampleCount = u32;
#[derive(Clone)]
pub struct ImageDescriptor1(VkImageCreateInfo, bool);
#[derive(Clone)]
pub struct ImageDescriptor2(VkImageCreateInfo, bool);
#[derive(Clone)]
pub struct ImageDescriptor3(VkImageCreateInfo);
impl ImageDescriptor1
{
	pub fn new(format: VkFormat, extent: u32, usage: VkImageUsageFlags) -> Self
	{
		ImageDescriptor1(VkImageCreateInfo
		{
			sType: VkStructureType::ImageCreateInfo, pNext: std::ptr::null(), flags: 0,
			imageType: VkImageType::Dim1, format: format, extent: VkExtent3D(extent, 1, 1),
			mipLevels: 1, arrayLayers: 1, samples: VK_SAMPLE_COUNT_1_BIT, tiling: VkImageTiling::Optimal,
			usage: usage, sharingMode: VkSharingMode::Exclusive, initialLayout: VkImageLayout::Preinitialized,
			queueFamilyIndexCount: 0, pQueueFamilyIndices: std::ptr::null()
		}, false)
	}
	pub fn device_local(mut self) -> Self { self.1 = true; self }
	pub fn is_device_local(&self) -> bool { self.1 }
}
impl ImageDescriptor2
{
	pub fn new(format: VkFormat, extent: Size2, usage: VkImageUsageFlags) -> Self
	{
		let Size2(width, height) = extent;
		ImageDescriptor2(VkImageCreateInfo
		{
			sType: VkStructureType::ImageCreateInfo, pNext: std::ptr::null(), flags: 0,
			imageType: VkImageType::Dim2, format: format, extent: VkExtent3D(width, height, 1),
			mipLevels: 1, arrayLayers: 1, samples: VK_SAMPLE_COUNT_1_BIT, tiling: VkImageTiling::Optimal,
			usage: usage, sharingMode: VkSharingMode::Exclusive, initialLayout: VkImageLayout::Preinitialized,
			queueFamilyIndexCount: 0, pQueueFamilyIndices: std::ptr::null()
		}, false)
	}
	pub fn device_local(mut self) -> Self { self.1 = true; self }
	pub fn is_device_local(&self) -> bool { self.1 }
}
impl ImageDescriptor3
{
	pub fn new(format: VkFormat, extent: Size3, usage: VkImageUsageFlags) -> Self
	{
		ImageDescriptor3(VkImageCreateInfo
		{
			sType: VkStructureType::ImageCreateInfo, pNext: std::ptr::null(), flags: 0,
			imageType: VkImageType::Dim3, format: format, extent: unsafe { std::mem::transmute(extent) },
			mipLevels: 1, arrayLayers: 1, samples: VK_SAMPLE_COUNT_1_BIT, tiling: VkImageTiling::Optimal,
			usage: usage, sharingMode: VkSharingMode::Exclusive, initialLayout: VkImageLayout::Preinitialized,
			queueFamilyIndexCount: 0, pQueueFamilyIndices: std::ptr::null()
		})
	}
}
macro_rules! ImplImageDescriptorMutations
{
	(for $t: ty) =>
	{
		impl $t
		{
			fn mip_levels(mut self, levels: u32) -> Self
			{
				self.0.mipLevels = levels;
				self
			}
			fn array_layers(mut self, layers: u32) -> Self
			{
				self.0.arrayLayers = layers;
				self
			}
			fn sample_flags(mut self, samples: SampleCount) -> Self
			{
				self.0.samples = samples;
				self
			}
		}
	}
}
ImplImageDescriptorMutations!(for ImageDescriptor1);
ImplImageDescriptorMutations!(for ImageDescriptor2);
ImplImageDescriptorMutations!(for ImageDescriptor3);

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
impl<'a> std::convert::Into<VkImageSubresourceRange> for &'a ImageSubresourceRange
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

// Content Type with size in bytes
pub enum BufferContent
{
	Vertex(usize), Index(usize), Uniform(usize), Storage(usize), IndirectCallParam(usize)
}
impl BufferContent
{
	fn usage_bit(&self) -> VkBufferUsageFlags
	{
		match self
		{
			&BufferContent::Vertex(_) => VK_BUFFER_USAGE_VERTEX_BUFFER_BIT,
			&BufferContent::Index(_) => VK_BUFFER_USAGE_INDEX_BUFFER_BIT,
			&BufferContent::Uniform(_) => VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT,
			&BufferContent::Storage(_) => VK_BUFFER_USAGE_STORAGE_BUFFER_BIT,
			&BufferContent::IndirectCallParam(_) => VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT
		}
	}
}
/// BufferPreallocator: Calculate offsets of contents and Hold their
pub struct BufferPreallocator<'a> { engine: &'a GraphicsInterface, usage_flags: VkBufferUsageFlags, offsets: Vec<usize> }
pub struct BufferOffsets(Vec<usize>);
impl<'a> BufferPreallocator<'a>
{
	pub fn new(engine: &'a GraphicsInterface, contents: &[BufferContent]) -> Self
	{
		fn alignment(v: usize, a: usize) -> usize { (v as f64 / a as f64).ceil() as usize * a }
		let (alignment_u, alignment_s) =
		(
			engine.device_limits.minUniformBufferOffsetAlignment as usize,
			engine.device_limits.minStorageBufferOffsetAlignment as usize
		);
		let usage_flags = contents.iter().map(BufferContent::usage_bit).fold(0, |a, d| a | d);
		let offsets = contents.iter().chain(&[BufferContent::Vertex(0)]).scan(0usize, |a, d|
		{
			let (current, size) = match d
			{
				&BufferContent::Vertex(s) | &BufferContent::Index(s) | &BufferContent::IndirectCallParam(s) => (*a, s),
				&BufferContent::Uniform(s) => (alignment(*a, alignment_u), s),
				&BufferContent::Storage(s) => (alignment(*a, alignment_s), s)
			};
			*a = current + size;
			Some(current)
		}).collect();

		info!(target: "Interlude::BufferPreallocator", "Preallocation: ");
		info!(target: "Interlude::BufferPreallocator", "-- Minimum Alignment for: UniformBuffer={} bytes, StorageBuffer={} bytes", alignment_u, alignment_s);
		info!(target: "Interlude::BufferPreallocator", "-- Offsets: {:?}", offsets);

		BufferPreallocator { engine: engine, usage_flags: usage_flags, offsets: offsets }
	}
	pub fn offset(&self, index: usize) -> usize { self.offsets[index] }
	pub fn total_size(&self) -> usize { self.offsets.last().map(|&x| x).unwrap_or(0) }
	pub fn instantiate(&self) -> EngineResult<(DeviceBuffer, StagingBuffer)>
	{
		let d = DeviceBuffer::new(self.engine, self.total_size() as VkDeviceSize, self.usage_flags);
		let s = StagingBuffer::new(self.engine, self.total_size() as VkDeviceSize, self.usage_flags);
		(d, s).flatten()
	}
	pub fn independence(self) -> BufferOffsets { BufferOffsets(self.offsets) }
}
impl BufferOffsets
{
	pub fn offset(&self, index: usize) -> usize { self.0[index] }
	pub fn total_size(&self) -> usize { self.0.last().map(|&x| x).unwrap_or(0) }
}

pub struct ImagePreallocator<'a>
{
	engine: &'a GraphicsInterface,
	dim1_images: Vec<&'a ImageDescriptor1>, dim2_images: Vec<&'a ImageDescriptor2>, dim3_images: Vec<&'a ImageDescriptor3>
}
impl<'a> ImagePreallocator<'a>
{
	pub fn new(engine: &'a GraphicsInterface,
		dim1_images: Vec<&'a ImageDescriptor1>, dim2_images: Vec<&'a ImageDescriptor2>, dim3_images: Vec<&'a ImageDescriptor3>) -> Self
	{
		ImagePreallocator { engine: engine, dim1_images: dim1_images, dim2_images: dim2_images, dim3_images: dim3_images }
	}
	pub fn instantiate(&self) -> EngineResult<(DeviceImage, Option<StagingImage>)>
	{
		let (i1, i2, i3) =
		(
			try!(self.dim1_images.iter().map(|&&ImageDescriptor1(ref d, _)| Image1D::new(self.engine, d)).collect::<Result<_, _>>()),
			try!(self.dim2_images.iter().map(|&&ImageDescriptor2(ref d, _)| Image2D::new(self.engine, d)).collect::<Result<_, _>>()),
			try!(self.dim3_images.iter().map(|&&ImageDescriptor3(ref d)| Image3D::new(self.engine, d)).collect::<Result<_, _>>())
		);
		let si = try!(self.dim1_images.iter().filter(|&&&ImageDescriptor1(_, dl)| dl == false).map(|&&ImageDescriptor1(ref d, _)| (Size2(d.extent.0, d.extent.1), d.format))
			.chain(self.dim2_images.iter().filter(|&&&ImageDescriptor2(_, dl)| dl == false).map(|&&ImageDescriptor2(ref d, _)| (Size2(d.extent.0, d.extent.1), d.format)))
			.map(|(s, f)| LinearImage2D::new(self.engine, &s, f)).collect::<Result<Vec<_>, _>>());
		
		let d = DeviceImage::new(self.engine, i1, i2, i3);
		let s = if si.is_empty() { None } else { try!(StagingImage::new(self.engine, si).map(Some)) };
		d.map(|d| (d, s))
	}
}

pub struct DeviceBuffer { buffer: vk::Buffer, memory: vk::DeviceMemory, size: VkDeviceSize }
pub struct StagingBuffer { buffer: vk::Buffer, memory: vk::DeviceMemory, size: VkDeviceSize }
impl DeviceBuffer
{
	fn new(engine: &GraphicsInterface, size: VkDeviceSize, usage: VkBufferUsageFlags) -> EngineResult<Self>
	{
		vk::Buffer::new(engine.device(), &VkBufferCreateInfo
		{
			sType: VkStructureType::BufferCreateInfo, pNext: std::ptr::null(), flags: 0,
			size: size, usage: usage | VK_BUFFER_USAGE_TRANSFER_DST_BIT, sharingMode: VkSharingMode::Exclusive,
			queueFamilyIndexCount: 0, pQueueFamilyIndices: std::ptr::null()	
		}).and_then(|b| vk::DeviceMemory::alloc(engine.device(), &VkMemoryAllocateInfo
		{
			sType: VkStructureType::MemoryAllocateInfo, pNext: std::ptr::null(),
			allocationSize: b.get_memory_requirements().size, memoryTypeIndex: engine.memindex(MemoryIndexType::DeviceLocal)
		}).map(|m| (b, m))).and_then(|(b, m)| m.bind_buffer(&b, 0).map(|_| DeviceBuffer { buffer: b, memory: m, size: size })).map_err(From::from)
	}
}
impl StagingBuffer
{
	fn new(engine: &GraphicsInterface, size: VkDeviceSize, usage: VkBufferUsageFlags) -> EngineResult<Self>
	{
		vk::Buffer::new(engine.device(), &VkBufferCreateInfo
		{
			sType: VkStructureType::BufferCreateInfo, pNext: std::ptr::null(), flags: 0,
			size: size, usage: usage | VK_BUFFER_USAGE_TRANSFER_SRC_BIT, sharingMode: VkSharingMode::Exclusive,
			queueFamilyIndexCount: 0, pQueueFamilyIndices: std::ptr::null()
		}).and_then(|b| vk::DeviceMemory::alloc(engine.device(), &VkMemoryAllocateInfo
		{
			sType: VkStructureType::MemoryAllocateInfo, pNext: std::ptr::null(),
			allocationSize: b.get_memory_requirements().size, memoryTypeIndex: engine.memindex(MemoryIndexType::HostVisible)
		}).map(|m| (b, m))).and_then(|(b, m)| m.bind_buffer(&b, 0).map(|_| StagingBuffer { buffer: b, memory: m, size: size })).map_err(From::from)
	}

	pub fn map(&self) -> EngineResult<MemoryMappedRange>
	{
		self.memory.map(0 .. self.size).map(|ptr| MemoryMappedRange { parent: self, ptr: ptr }).map_err(From::from)
	}
}
impl Resource for DeviceBuffer { type Type = VkBuffer; fn resource(&self) -> VkBuffer { *self.buffer } }
impl Resource for StagingBuffer { type Type = VkBuffer; fn resource(&self) -> VkBuffer { *self.buffer } }

pub struct DeviceImage
{
	dim1: Vec<Rc<Image1D>>, dim2: Vec<Rc<Image2D>>, dim3: Vec<Rc<Image3D>>,
	memory: vk::DeviceMemory, size: VkDeviceSize
}
impl DeviceImage
{
	fn new(engine: &GraphicsInterface, d1_images: Vec<Image1D>, d2_images: Vec<Image2D>, d3_images: Vec<Image3D>)
		-> Result<Self, EngineError>
	{
		let image_offsets = {
			let d1_image_requirements = d1_images.iter().map(MemoryRequirements::memory_requirements);
			let d2_image_requirements = d2_images.iter().map(MemoryRequirements::memory_requirements);
			let d3_image_requirements = d3_images.iter().map(MemoryRequirements::memory_requirements);
			
			d1_image_requirements.chain(d2_image_requirements).chain(d3_image_requirements)
				.chain([VkMemoryRequirements { size: 0, alignment: 1, memoryTypeBits: 0 }].into_iter().map(|&x| x)).scan((0, 0), |tup, req|
				{
					let compatible_bits = tup.1 | (req.memoryTypeBits & 0xff);
					let current_offs = ((tup.0 as f64 / req.alignment as f64).ceil() as VkDeviceSize) * req.alignment;
					let offs = current_offs + req.size;
					*tup = (offs, compatible_bits);
					Some((current_offs, compatible_bits))
				}).collect::<Vec<_>>()
		};
		let memory_size = try!(image_offsets.last().map(|&(x, _)| x).ok_or(EngineError::AllocateMemoryWithEmptyResources));
		info!(target: "Interlude::Resource", "Going to allocate image for device {} bytes", memory_size);

		vk::DeviceMemory::alloc(engine.device(), &VkMemoryAllocateInfo
		{
			sType: VkStructureType::MemoryAllocateInfo, pNext: std::ptr::null(),
			allocationSize: memory_size, memoryTypeIndex: engine.memindex(MemoryIndexType::DeviceLocal)
		}).and_then(|memory|
		{
			let image_resources = d1_images.iter().map(|&Image1D(ref o, _)| o)
				.chain(d2_images.iter().map(|&Image2D(ref o, _)| o))
				.chain(d3_images.iter().map(|&Image3D(ref o, _)| o));
			
			for (&(offs, _), res) in image_offsets.iter().zip(image_resources)
			{
				try!(memory.bind_image(res, offs));
			}
			Ok(memory)
		}).map(move |memory| DeviceImage
		{
			dim1: d1_images.into_iter().map(Rc::new).collect(),
			dim2: d2_images.into_iter().map(Rc::new).collect(),
			dim3: d3_images.into_iter().map(Rc::new).collect(),
			memory: memory, size: memory_size
		}).map_err(From::from)
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
impl StagingImage
{
	fn new(engine: &GraphicsInterface, ld2_images: Vec<LinearImage2D>) -> Result<Self, EngineError>
	{
		let image_offsets =
		{
			let ld2_image_requirements = ld2_images.iter().map(MemoryRequirements::memory_requirements);

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

		vk::DeviceMemory::alloc(engine.device(), &VkMemoryAllocateInfo
		{
			sType: VkStructureType::MemoryAllocateInfo, pNext: std::ptr::null(),
			allocationSize: memory_size, memoryTypeIndex: engine.memindex(MemoryIndexType::HostVisible)
		}).and_then(|memory|
		{
			for (&offs, &LinearImage2D(ref o, _)) in image_offsets.iter().zip(ld2_images.iter())
			{
				try!(memory.bind_image(o, offs));
			}
			Ok(memory)
		}).map(move |memory| StagingImage
		{
			linear_dim2_images: ld2_images, linear_dim2_images_offset: image_offsets,
			memory: memory, size: memory_size
		}).map_err(From::from)
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl<'a> std::convert::Into<VkSamplerCreateInfo> for &'a SamplerState
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

pub struct Sampler(vk::Sampler);
impl Sampler
{
	pub fn new(engine: &GraphicsInterface, info: &SamplerState) -> EngineResult<Self>
	{
		vk::Sampler::new(engine.device(), &info.into()).map(Sampler).map_err(From::from)
	}
}
impl InternalExports for Sampler { type InternalT = vk::Sampler; fn get_internal(&self) -> &vk::Sampler { &self.0 } }

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
pub struct ImageView1D { parent: Rc<Image1D>, internal: vk::ImageView, format: VkFormat }
pub struct ImageView2D { parent: Rc<Image2D>, internal: vk::ImageView, format: VkFormat }
pub struct ImageView3D { parent: Rc<Image3D>, internal: vk::ImageView, format: VkFormat }
impl ImageView1D
{
	pub fn make_from(res: &Rc<Image1D>, format: VkFormat, cm: ComponentMapping, subrange: ImageSubresourceRange) -> EngineResult<Self>
	{
		vk::ImageView::new(res.0.parent(), &VkImageViewCreateInfo
		{
			sType: VkStructureType::ImageViewCreateInfo, pNext: std::ptr::null(), flags: 0,
			image: res.resource(), viewType: VkImageViewType::Dim1, format: format,
			components: cm.into(), subresourceRange: subrange.into()	
		}).map(|v| ImageView1D { parent: res.clone(), internal: v, format: format }).map_err(EngineError::from)
	}
}
impl ImageView2D
{
	pub fn make_from(res: &Rc<Image2D>, format: VkFormat, cm: ComponentMapping, subrange: ImageSubresourceRange) -> EngineResult<Self>
	{
		vk::ImageView::new(res.0.parent(), &VkImageViewCreateInfo
		{
			sType: VkStructureType::ImageViewCreateInfo, pNext: std::ptr::null(), flags: 0,
			image: res.resource(), viewType: VkImageViewType::Dim2, format: format,
			components: cm.into(), subresourceRange: subrange.into()	
		}).map(|v| ImageView2D { parent: res.clone(), internal: v, format: format }).map_err(EngineError::from)
	}
}
impl ImageView3D
{
	pub fn make_from(res: &Rc<Image3D>, format: VkFormat, cm: ComponentMapping, subrange: ImageSubresourceRange) -> EngineResult<Self>
	{
		vk::ImageView::new(res.0.parent(), &VkImageViewCreateInfo
		{
			sType: VkStructureType::ImageViewCreateInfo, pNext: std::ptr::null(), flags: 0,
			image: res.resource(), viewType: VkImageViewType::Dim3, format: format,
			components: cm.into(), subresourceRange: subrange.into()	
		}).map(|v| ImageView3D { parent: res.clone(), internal: v, format: format }).map_err(EngineError::from)
	}
}
impl std::ops::Deref for ImageView1D { type Target = Image1D; fn deref(&self) -> &Image1D { self.parent.deref() } }
impl std::ops::Deref for ImageView2D { type Target = Image2D; fn deref(&self) -> &Image2D { self.parent.deref() } }
impl std::ops::Deref for ImageView3D { type Target = Image3D; fn deref(&self) -> &Image3D { self.parent.deref() } }
pub trait ImageView
{
	fn get_native(&self) -> VkImageView;
	fn format(&self) -> VkFormat;
}
pub trait UserImageView<ResourceT: ImageResource>: ImageView
{
	fn get_resource(&self) -> &Rc<ResourceT>;
}
impl ImageView for ImageView1D
{
	fn get_native(&self) -> VkImageView { *self.internal }
	fn format(&self) -> VkFormat { self.format }
}
impl ImageView for ImageView2D
{
	fn get_native(&self) -> VkImageView { *self.internal }
	fn format(&self) -> VkFormat { self.format }
}
impl ImageView for ImageView3D
{
	fn get_native(&self) -> VkImageView { *self.internal }
	fn format(&self) -> VkFormat { self.format }
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

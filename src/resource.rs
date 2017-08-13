///! Interlude: Resources(Buffer and Image)

use interlude_vk_defs::*;
use interlude_vk_funport::*;
use data::*;
use device::Device;
use ginterface::MemoryIndexType;
use {EngineResult, EngineError, GraphicsInterface};
use std::slice::from_raw_parts_mut;
use std::ops::{BitOr, BitOrAssign, Deref, Range};
use std::rc::Rc;
use std::ptr::{null, null_mut};
use std::mem::{replace, transmute, uninitialized as reserved};
use libc::c_void;
use subsystem_layer::{NativeHandleProvider, NativeResultValueHandler, NativeImage};

fn alignment(v: usize, a: usize) -> usize { (v as f64 / a as f64).ceil() as usize * a }

// Resource DataType //
#[derive(Clone)] pub struct Image1D(Rc<NativeImage<u32>>);
#[derive(Clone)] pub struct Image2D(Rc<NativeImage<Size2>>);
#[derive(Clone)] pub struct Image3D(Rc<NativeImage<Size3>>);
#[derive(Clone)] pub struct LinearImage(Rc<NativeImage<Size2>>);

/// Memory Requirements of Resource
pub trait RequireMemory { fn memory_requirements(&self) -> VkMemoryRequirements; }
macro_rules! ReqMemory
{
	([Image] $($t: ty),*) =>
	{
		$(impl RequireMemory for $t
		{
			fn memory_requirements(&self) -> VkMemoryRequirements
			{
				let mut mreq = unsafe { reserved() };
				unsafe { vkGetImageMemoryRequirements((self.0).1.native(), self.0.native(), &mut mreq) }; mreq
			}
		})*
	}
}
ReqMemory!([Image] Image1D, Image2D, Image3D, LinearImage);

pub trait BufferResource { fn internal(&self) -> u64; fn size(&self) -> VkDeviceSize; }
pub trait ImageResource { fn internal(&self) -> u64; }
impl ImageResource for Image1D { fn internal(&self) -> u64 { unsafe { transmute(self.0.native()) } } }
impl ImageResource for Image2D { fn internal(&self) -> u64 { unsafe { transmute(self.0.native()) } } }
impl ImageResource for Image3D { fn internal(&self) -> u64 { unsafe { transmute(self.0.native()) } } }
impl ImageResource for LinearImage { fn internal(&self) -> u64 { unsafe { transmute(self.0.native()) } } }

impl LinearImage
{
	fn new(engine: &GraphicsInterface, size: &Size2, format: VkFormat) -> EngineResult<Self>
	{
		let mut img = unsafe { reserved() };
		unsafe { vkCreateImage(engine.device().native(), &VkImageCreateInfo
		{
			imageType: VK_IMAGE_TYPE_2D, format: format, extent: VkExtent3D::new(size.0, size.1, 1), mipLevels: 1, arrayLayers: 1,
			samples: VK_SAMPLE_COUNT_1_BIT, tiling: VK_IMAGE_TILING_LINEAR, usage: VK_IMAGE_USAGE_TRANSFER_SRC_BIT,
			initialLayout: VK_IMAGE_LAYOUT_PREINITIALIZED, .. Default::default()
		}, null(), &mut img) }.make_result_with(|| LinearImage(Rc::new(NativeImage(img, engine.device().clone(), size.clone()))))
	}
}
impl Image1D
{
	fn new(engine: &GraphicsInterface, desc: &VkImageCreateInfo) -> EngineResult<Self>
	{
		let mut img = unsafe { reserved() };
		unsafe { vkCreateImage(engine.device().native(), desc, null(), &mut img) }
			.make_result_with(|| Image1D(Rc::new(NativeImage(img, engine.device().clone(), desc.extent.width))))
	}
}
impl Image2D
{
	fn new(engine: &GraphicsInterface, desc: &VkImageCreateInfo) -> EngineResult<Self>
	{
		let mut img = unsafe { reserved() };
		unsafe { vkCreateImage(engine.device().native(), desc, null(), &mut img) }
			.make_result_with(|| Image2D(Rc::new(NativeImage(img, engine.device().clone(), Size2(desc.extent.width, desc.extent.height)))))
	}
}
impl Image3D
{
	fn new(engine: &GraphicsInterface, desc: &VkImageCreateInfo) -> EngineResult<Self>
	{
		let mut img = unsafe { reserved() };
		unsafe { vkCreateImage(engine.device().native(), desc, null(), &mut img) }
			.make_result_with(|| Image3D(Rc::new(NativeImage(img, engine.device().clone(), desc.extent.clone().into()))))
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
			imageType: VK_IMAGE_TYPE_1D, format: format, extent: VkExtent3D { width: extent, height: 1, depth: 1 },
			mipLevels: 1, arrayLayers: 1, samples: VK_SAMPLE_COUNT_1_BIT, tiling: VK_IMAGE_TILING_OPTIMAL,
			usage, initialLayout: VK_IMAGE_LAYOUT_PREINITIALIZED, .. Default::default()
		}, false)
	}
	pub fn device_local(mut self) -> Self { self.1 = true; self }
	pub fn is_device_local(&self) -> bool { self.1 }

	pub fn mip_levels(mut self, levels: u32) -> Self
	{
		self.0.mipLevels = levels;
		self
	}
	pub fn array_layers(mut self, layers: u32) -> Self
	{
		self.0.arrayLayers = layers;
		self
	}
	pub fn sample_flags(mut self, samples: SampleCount) -> Self
	{
		self.0.samples = samples;
		self
	}
}
impl ImageDescriptor2
{
	pub fn new(format: VkFormat, extent: Size2, usage: VkImageUsageFlags) -> Self
	{
		let Size2(width, height) = extent;
		ImageDescriptor2(VkImageCreateInfo
		{
			imageType: VK_IMAGE_TYPE_2D, format: format, extent: VkExtent3D { width, height, depth: 1 },
			mipLevels: 1, arrayLayers: 1, samples: VK_SAMPLE_COUNT_1_BIT, tiling: VK_IMAGE_TILING_OPTIMAL,
			usage, initialLayout: VK_IMAGE_LAYOUT_PREINITIALIZED, .. Default::default()
		}, false)
	}
	pub fn device_local(mut self) -> Self { self.1 = true; self }
	pub fn is_device_local(&self) -> bool { self.1 }
	
	pub fn mip_levels(mut self, levels: u32) -> Self
	{
		self.0.mipLevels = levels;
		self
	}
	pub fn array_layers(mut self, layers: u32) -> Self
	{
		self.0.arrayLayers = layers;
		self
	}
	pub fn sample_flags(mut self, samples: SampleCount) -> Self
	{
		self.0.samples = samples;
		self
	}
}
impl ImageDescriptor3
{
	pub fn new(format: VkFormat, extent: Size3, usage: VkImageUsageFlags) -> Self
	{
		ImageDescriptor3(VkImageCreateInfo
		{
			imageType: VK_IMAGE_TYPE_3D, format: format, extent: extent.into(),
			mipLevels: 1, arrayLayers: 1, samples: VK_SAMPLE_COUNT_1_BIT, tiling: VK_IMAGE_TILING_OPTIMAL,
			usage, initialLayout: VK_IMAGE_LAYOUT_PREINITIALIZED, .. Default::default()
		})
	}
	
	pub fn mip_levels(mut self, levels: u32) -> Self
	{
		self.0.mipLevels = levels;
		self
	}
	pub fn array_layers(mut self, layers: u32) -> Self
	{
		self.0.arrayLayers = layers;
		self
	}
	pub fn sample_flags(mut self, samples: SampleCount) -> Self
	{
		self.0.samples = samples;
		self
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)] #[repr(u8)]
pub enum ImageAspect
{
	Color = VK_IMAGE_ASPECT_COLOR_BIT as u8, Depth = VK_IMAGE_ASPECT_DEPTH_BIT as u8,
	Stencil = VK_IMAGE_ASPECT_STENCIL_BIT as u8, Metadata = VK_IMAGE_ASPECT_METADATA_BIT as u8
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)] #[repr(C)]
pub struct ImageAspectSet(VkFlags);
impl Into<ImageAspectSet> for ImageAspect { fn into(self) -> ImageAspectSet { ImageAspectSet(self as _) } }
impl BitOr for ImageAspect { type Output = ImageAspectSet; fn bitor(self, rhs: Self) -> ImageAspectSet { ImageAspectSet(self as VkFlags | rhs as VkFlags) } }
impl BitOr for ImageAspectSet { type Output = Self; fn bitor(self, rhs: Self) -> Self { ImageAspectSet(self.0 | rhs.0) } }
impl BitOr<ImageAspectSet> for ImageAspect { type Output = ImageAspectSet; fn bitor(self, rhs: ImageAspectSet) -> ImageAspectSet { ImageAspectSet(self as VkFlags | rhs.0) } }
impl BitOr<ImageAspect> for ImageAspectSet { type Output = Self; fn bitor(self, rhs: ImageAspect) -> Self { ImageAspectSet(self.0 | rhs as VkFlags) } }
impl BitOrAssign for ImageAspectSet { fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; } }
impl BitOrAssign<ImageAspect> for ImageAspectSet { fn bitor_assign(&mut self, rhs: ImageAspect) { self.0 |= rhs as VkFlags; } }
impl Into<VkImageAspectFlags> for ImageAspectSet { fn into(self) -> VkImageAspectFlags { self.0 } }

/// Image Subresource Parameters //
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImageSubresourceRange
{
	pub aspect: ImageAspectSet, pub mip_level_range: Range<u32>, pub array_range: Range<u32>
}
impl Default for ImageSubresourceRange
{
	/// Single layer and mip
	fn default() -> Self { ImageSubresourceRange { aspect: ImageAspectSet(0), mip_level_range: 0 .. 1, array_range: 0 .. 1 } }
}
impl<'a> Into<VkImageSubresourceRange> for &'a ImageSubresourceRange
{
	fn into(self) -> VkImageSubresourceRange
	{
		VkImageSubresourceRange
		{
			aspectMask: self.aspect.into(),
			baseMipLevel: self.mip_level_range.start, levelCount: self.mip_level_range.len() as _,
			baseArrayLayer: self.array_range.start, layerCount: self.array_range.len() as _
		}
	}
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImageSubresourceLayers
{
	pub aspect: ImageAspectSet, pub array_range: Range<u32>, pub mip_level: u32
}
impl Default for ImageSubresourceLayers
{
	/// Single layers, base mip
	fn default() -> Self { ImageSubresourceLayers { aspect: ImageAspectSet(0), array_range: 0 .. 1, mip_level: 0 } }
}
impl<'a> Into<VkImageSubresourceLayers> for &'a ImageSubresourceLayers
{
	fn into(self) -> VkImageSubresourceLayers
	{
		VkImageSubresourceLayers
		{
			aspectMask: self.aspect.into(), baseArrayLayer: self.array_range.start, layerCount: self.array_range.len() as _,
			mipLevel: self.mip_level
		}
	}
}

// Content Type with size in bytes
pub enum BufferContent
{
	Vertex(usize), Index(usize), Uniform(usize), Storage(usize), IndirectCallParam(usize),
	Custom(VkBufferUsageFlags, usize)
}
impl BufferContent
{
	fn deconstruct(&self) -> (VkBufferUsageFlags, usize)
	{
		match self
		{
			&BufferContent::Vertex(s) => (VK_BUFFER_USAGE_VERTEX_BUFFER_BIT, s),
			&BufferContent::Index(s) => (VK_BUFFER_USAGE_INDEX_BUFFER_BIT, s),
			&BufferContent::Uniform(s) => (VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT, s),
			&BufferContent::Storage(s) => (VK_BUFFER_USAGE_STORAGE_BUFFER_BIT, s),
			&BufferContent::IndirectCallParam(s) => (VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT, s),
			&BufferContent::Custom(u, s) => (u, s)
		}
	}
}
/// BufferPreallocator: Calculate offsets of contents and Hold their
pub struct BufferPreallocator<'a> { engine: &'a GraphicsInterface, usage_flags: VkBufferUsageFlags, offsets: Vec<usize>, total: usize }
pub struct BufferOffsets(Vec<usize>, usize);
impl<'a> BufferPreallocator<'a>
{
	pub fn new(engine: &'a GraphicsInterface, contents: &[BufferContent]) -> Self
	{
		let alignment_u = engine.device_limits.minUniformBufferOffsetAlignment as usize;
		let alignment_s = engine.device_limits.minStorageBufferOffsetAlignment as usize;

		// Aggregate usage bits and offsets //
		let (mut usage_flags, mut current_offset, mut offsets) = (0, 0, Vec::with_capacity(contents.len() + 1));
		for (usage, size) in contents.into_iter().map(BufferContent::deconstruct)
		{
			usage_flags |= usage;
			let offset = if (usage & VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT) != 0 { alignment(current_offset, alignment_u) }
			else if (usage & VK_BUFFER_USAGE_STORAGE_BUFFER_BIT) != 0 { alignment(current_offset, alignment_s) }
			else { current_offset };
			offsets.push(offset);
			current_offset = offset + size;
		}

		info!(target: "Interlude::BufferPreallocator", "Preallocation: ");
		info!(target: "Interlude::BufferPreallocator", "-- Minimum Alignment for: UniformBuffer={} bytes, StorageBuffer={} bytes", alignment_u, alignment_s);
		info!(target: "Interlude::BufferPreallocator", "-- Offsets: {:?}", offsets);

		BufferPreallocator { engine: engine, usage_flags: usage_flags, offsets: offsets, total: current_offset }
	}
	pub fn offset(&self, index: usize) -> usize { self.offsets[index] }
	pub fn total_size(&self) -> usize { self.total }
	pub fn instantiate(&self) -> EngineResult<(DeviceBuffer, StagingBuffer)>
	{
		let d = DeviceBuffer::new(self.engine, self.total as _, self.usage_flags)?;
		StagingBuffer::new(self.engine, self.total as _, self.usage_flags).map(|s| (d, s))
	}
	pub fn independence(self) -> BufferOffsets { BufferOffsets(self.offsets, self.total) }
}
impl BufferOffsets
{
	pub fn offset(&self, index: usize) -> usize { self.0[index] }
	pub fn total_size(&self) -> usize { self.1 }
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
	pub fn instantiate(&self) -> EngineResult<(DeviceImages, Option<StagingImages>)>
	{
		let i1 = self.dim1_images.iter().map(|&&ImageDescriptor1(ref d, _)| Image1D::new(self.engine, d)).collect::<EngineResult<_>>()?;
		let i2 = self.dim2_images.iter().map(|&&ImageDescriptor2(ref d, _)| Image2D::new(self.engine, d)).collect::<EngineResult<_>>()?;
		let i3 = self.dim3_images.iter().map(|&d3| Image3D::new(self.engine, &d3.0)).collect::<EngineResult<_>>()?;
		let si1 = self.dim1_images.iter().filter(|&&&ImageDescriptor1(_, dl)| !dl).map(|&&ImageDescriptor1(ref d, _)| (Size2(d.extent.width, d.extent.height), d.format));
		let si2 = self.dim2_images.iter().filter(|&&&ImageDescriptor2(_, dl)| !dl).map(|&&ImageDescriptor2(ref d, _)| (Size2(d.extent.width, d.extent.height), d.format));
		let si = si1.chain(si2).map(|(x, f)| LinearImage::new(self.engine, &x, f)).collect::<EngineResult<Vec<_>>>()?;
		
		let d = DeviceImages::new(self.engine, i1, i2, i3);
		let s = if si.is_empty() { None } else { StagingImages::new(self.engine, si).map(Some)? };
		d.map(|d| (d, s))
	}
}

pub struct DeviceBuffer { buffer: VkBuffer, memory: VkDeviceMemory, size: VkDeviceSize, parent: Rc<Device> }
pub struct StagingBuffer { buffer: VkBuffer, memory: VkDeviceMemory, size: VkDeviceSize, parent: Rc<Device> }
impl DeviceBuffer
{
	fn new(engine: &GraphicsInterface, size: VkDeviceSize, usage: VkBufferUsageFlags) -> EngineResult<Self>
	{
		let (mut buffer, mut mreq, mut memory) = unsafe { reserved() };
		unsafe
		{
			vkCreateBuffer(engine.device().native(), &VkBufferCreateInfo
			{
				size, usage: usage | VK_BUFFER_USAGE_TRANSFER_DST_BIT, .. Default::default()
			}, null(), &mut buffer).into_result()?;
			vkGetBufferMemoryRequirements(engine.device().native(), buffer, &mut mreq);
			vkAllocateMemory(engine.device().native(), &VkMemoryAllocateInfo
			{
				allocationSize: mreq.size, memoryTypeIndex: engine.memindex(MemoryIndexType::DeviceLocal), .. Default::default()
			}, null(), &mut memory).into_result()?;
			vkBindBufferMemory(engine.device().native(), buffer, memory, 0)
		}.make_result_with(|| DeviceBuffer { buffer, memory, size, parent: engine.device().clone() })
	}
}
impl StagingBuffer
{
	fn new(engine: &GraphicsInterface, size: VkDeviceSize, usage: VkBufferUsageFlags) -> EngineResult<Self>
	{
		let (mut buffer, mut mreq, mut memory) = unsafe { reserved() };
		unsafe
		{
			vkCreateBuffer(engine.device().native(), &VkBufferCreateInfo
			{
				size, usage: usage | VK_BUFFER_USAGE_TRANSFER_SRC_BIT, .. Default::default()
			}, null(), &mut buffer).into_result()?;
			vkGetBufferMemoryRequirements(engine.device().native(), buffer, &mut mreq);
			vkAllocateMemory(engine.device().native(), &VkMemoryAllocateInfo
			{
				allocationSize: mreq.size, memoryTypeIndex: engine.memindex(MemoryIndexType::HostVisible), .. Default::default()
			}, null(), &mut memory).into_result()?;
			vkBindBufferMemory(engine.device().native(), buffer, memory, 0)
		}.make_result_with(|| StagingBuffer { buffer, memory, size, parent: engine.device().clone() })
	}
}
// impl NativeHandleProvider for DeviceBuffer { type NativeT = VkBuffer; fn native(&self) -> VkBuffer { self.buffer } }
// impl NativeHandleProvider for StagingBuffer { type NativeT = VkBuffer; fn native(&self) -> VkBuffer { self.buffer } }
impl Drop for DeviceBuffer
{
	fn drop(&mut self)
	{
		unsafe { vkFreeMemory(self.parent.native(), self.memory, null()) };
		unsafe { vkDestroyBuffer(self.parent.native(), self.buffer, null()) };
	}
}
impl Drop for StagingBuffer
{
	fn drop(&mut self)
	{
		unsafe { vkFreeMemory(self.parent.native(), self.memory, null()) };
		unsafe { vkDestroyBuffer(self.parent.native(), self.buffer, null()) };
	}
}
impl BufferResource for DeviceBuffer { fn internal(&self) -> u64 { self.buffer as _ } fn size(&self) -> VkDeviceSize { self.size } }

pub struct DeviceImages
{
	memory: VkDeviceMemory, parent: Rc<Device>, size: VkDeviceSize, resources: (Vec<Image1D>, Vec<Image2D>, Vec<Image3D>)
}
impl DeviceImages
{
	fn placement(images: (&[Image1D], &[Image2D], &[Image3D])) -> EngineResult<(Vec<VkDeviceSize>, VkDeviceSize)>
	{
		let (mut current_offset, mut offsets) = (0, Vec::with_capacity(images.0.len() + images.1.len() + images.2.len()));
		for ireq in images.0.into_iter().map(RequireMemory::memory_requirements)
			.chain(images.1.into_iter().map(RequireMemory::memory_requirements))
			.chain(images.2.into_iter().map(RequireMemory::memory_requirements))
		{
			let offset = alignment(current_offset as _, ireq.alignment as _) as _;
			offsets.push(offset);
			current_offset += offset + ireq.size;
		}
		if current_offset == 0 { Err(EngineError::AllocateMemoryWithEmptyResources) }
		else { Ok((offsets, current_offset)) }
	}
	fn new(engine: &GraphicsInterface, d1_images: Vec<Image1D>, d2_images: Vec<Image2D>, d3_images: Vec<Image3D>)
		-> Result<Self, EngineError>
	{
		let (offsets, size) = Self::placement((&d1_images, &d2_images, &d3_images))?;
		info!(target: "Interlude::Resource", "Going to allocate images for device: {} bytes total", size);

		let mut memory = unsafe { reserved() };
		unsafe { vkAllocateMemory(engine.device().native(), &VkMemoryAllocateInfo
		{
			allocationSize: size, memoryTypeIndex: engine.memindex(MemoryIndexType::DeviceLocal), .. Default::default()
		}, null(), &mut memory) }.into_result()?;
		for (&offs, res) in offsets.iter().zip(d1_images.iter().map(|&Image1D(ref o)| o.native())
			.chain(d2_images.iter().map(|&Image2D(ref o)| o.native()))
			.chain(d3_images.iter().map(|&Image3D(ref o)| o.native())))
		{
			unsafe { vkBindImageMemory(engine.device().native(), res, memory, offs) }.into_result()?;
		}
		Ok(DeviceImages { memory, size, resources: (d1_images, d2_images, d3_images), parent: engine.device().clone() })
	}

	pub fn dim1(&self) -> &[Image1D] { &self.resources.0 }
	pub fn dim2(&self) -> &[Image2D] { &self.resources.1 }
	pub fn dim3(&self) -> &[Image3D] { &self.resources.2 }
}

pub struct StagingImages
{
	memory: VkDeviceMemory, parent: Rc<Device>, resources: Vec<LinearImage>,
	placement_offsets: Vec<VkDeviceSize>, size: VkDeviceSize
}
impl StagingImages
{
	fn placement(resources: &[LinearImage]) -> EngineResult<(Vec<VkDeviceSize>, VkDeviceSize)>
	{
		let (mut current_offset, mut offsets) = (0, Vec::with_capacity(resources.len()));
		for ireq in resources.iter().map(RequireMemory::memory_requirements)
		{
			let offset = alignment(current_offset as _, ireq.alignment as _) as _;
			offsets.push(offset);
			current_offset = offset + ireq.size;
		}
		if current_offset == 0 { Err(EngineError::AllocateMemoryWithEmptyResources) }
		else { Ok((offsets, current_offset)) }
	}
	fn new(engine: &GraphicsInterface, resources: Vec<LinearImage>) -> EngineResult<Self>
	{
		let (offsets, size) = Self::placement(&resources)?;
		info!(target: "Interlude::Resource", "Going to allocate images for host: {} bytes total", size);

		let mut memory = unsafe { reserved() };
		unsafe { vkAllocateMemory(engine.device().native(), &VkMemoryAllocateInfo
		{
			allocationSize: size, memoryTypeIndex: engine.memindex(MemoryIndexType::HostVisible), .. Default::default()
		}, null(), &mut memory) }.into_result()?;
		for (&offs, &LinearImage(ref o)) in offsets.iter().zip(resources.iter())
		{
			unsafe { vkBindImageMemory(engine.device().native(), o.native(), memory, offs) }.into_result()?;
		}
		Ok(StagingImages { memory, parent: engine.device().clone(), resources, placement_offsets: offsets, size })
	}

	pub fn offsets(&self) -> &[VkDeviceSize] { &self.placement_offsets }
	pub fn size(&self) -> VkDeviceSize { self.size }
}
impl Deref for StagingImages { type Target = [LinearImage]; fn deref(&self) -> &[LinearImage] { &self.resources } }
impl Drop for DeviceImages
{
	fn drop(&mut self) { unsafe { vkFreeMemory(self.parent.native(), self.memory, null()) }; }
}
impl Drop for StagingImages
{
	fn drop(&mut self) { unsafe { vkFreeMemory(self.parent.native(), self.memory, null()) }; }
}

/// Accessible Resource from Host, Supports map into host memory and unmap from host memory operations
pub trait StagingResource { fn map(&self) -> EngineResult<MappedRange>; fn unmap(&self); }
impl StagingResource for StagingBuffer
{
	fn map(&self) -> EngineResult<MappedRange>
	{
		let mut ptr = null_mut();
		unsafe { vkMapMemory(self.parent.native(), self.memory, 0, self.size, 0, &mut ptr) }.make_result_with(|| MappedRange { caller: self, ptr })
	}
	fn unmap(&self) { unsafe { vkUnmapMemory(self.parent.native(), self.memory) }; }
}
impl StagingResource for StagingImages
{
	fn map(&self) -> EngineResult<MappedRange>
	{
		let mut ptr = null_mut();
		unsafe { vkMapMemory(self.parent.native(), self.memory, 0, self.size, 0, &mut ptr) }.make_result_with(|| MappedRange { caller: self, ptr })
	}
	fn unmap(&self) { unsafe { vkUnmapMemory(self.parent.native(), self.memory) }; }
}

pub struct MappedRange<'a> { caller: &'a StagingResource, ptr: *mut c_void }
impl <'a> MappedRange<'a>
{
	pub fn map_mut<MappedStructureT>(&self, offset: usize) -> &'a mut MappedStructureT
	{
		unsafe { transmute(transmute::<_, usize>(self.ptr) + offset) }
	}
	pub fn range_mut<MappedStructureT>(&self, offset: usize, count: usize) -> &'a mut [MappedStructureT]
	{
		unsafe { from_raw_parts_mut(self.ptr.offset(offset as isize) as *mut _, count) }
	}
	pub fn into_raw(mut self) -> *mut c_void { replace(&mut self.ptr, null_mut()) }
	pub unsafe fn from_raw(caller: &'a StagingResource, ptr: *mut c_void) -> Self { MappedRange { caller, ptr } }
}
impl<'a> Drop for MappedRange<'a>
{
	fn drop(&mut self) { if !self.ptr.is_null() { self.caller.unmap(); } }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)] #[repr(u8)]
pub enum Filter
{
	Nearest = VK_FILTER_NEAREST as u8, Linear = VK_FILTER_LINEAR as u8
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)] #[repr(u8)]
pub enum AddressMode
{
	Clamp = VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER as u8, Repeat = VK_SAMPLER_ADDRESS_MODE_REPEAT as u8,
	Mirror = VK_SAMPLER_ADDRESS_MODE_MIRRORED_REPEAT as u8
}
pub struct SamplerState
{
	pub mag_filter: Filter, pub min_filter: Filter,
	pub address_mode: (AddressMode, AddressMode, AddressMode),
	pub max_anisotropy: Option<f32>
}
impl Default for SamplerState
{
	fn default() -> Self
	{
		SamplerState
		{
			mag_filter: Filter::Nearest, min_filter: Filter::Nearest,
			address_mode: (AddressMode::Clamp, AddressMode::Clamp, AddressMode::Clamp),
			max_anisotropy: None
		}
	}
}
impl<'a> Into<VkSamplerCreateInfo> for &'a SamplerState
{
	fn into(self) -> VkSamplerCreateInfo
	{
		VkSamplerCreateInfo
		{
			magFilter: self.mag_filter as _, minFilter: self.min_filter as _, mipmapMode: VK_SAMPLER_MIPMAP_MODE_LINEAR,
			addressModeU: self.address_mode.0 as _, addressModeV: self.address_mode.1 as _, addressModeW: self.address_mode.2 as _,
			anisotropyEnable: self.max_anisotropy.is_some() as _, maxAnisotropy: self.max_anisotropy.unwrap_or(0.0),
			minLod: 0.0f32, maxLod: 1.0f32, borderColor: VK_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK, .. Default::default()
		}
	}
}

pub struct Sampler(VkSampler, Rc<Device>);
impl Sampler
{
	pub fn new(engine: &GraphicsInterface, info: &SamplerState) -> EngineResult<Self>
	{
		let mut smp = 0 as _;
		unsafe { vkCreateSampler(engine.device().native(), &info.into(), null(), &mut smp) }
			.make_result_with(|| Sampler(smp, engine.device().clone()))
	}
}
impl NativeHandleProvider for Sampler
{
	type NativeT = VkSampler;
	fn native(&self) -> VkSampler { self.0 }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)] #[repr(C)]
pub enum ComponentSwizzle
{
	R = VK_COMPONENT_SWIZZLE_R as _, G = VK_COMPONENT_SWIZZLE_G as _, B = VK_COMPONENT_SWIZZLE_B as _, A = VK_COMPONENT_SWIZZLE_A as _,
	One = VK_COMPONENT_SWIZZLE_ONE as _, Zero = VK_COMPONENT_SWIZZLE_ZERO as _, Id = VK_COMPONENT_SWIZZLE_IDENTITY as _
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)] #[repr(C)]
pub struct ComponentMapping(pub ComponentSwizzle, pub ComponentSwizzle, pub ComponentSwizzle, pub ComponentSwizzle);
impl ComponentMapping
{
	/// aaaa
	pub fn single_swizzle(org: ComponentSwizzle) -> Self
	{
		ComponentMapping(org, org, org, org)
	}
	/// abab
	pub fn double_swizzle_rep(org1: ComponentSwizzle, org2: ComponentSwizzle) -> Self
	{
		ComponentMapping(org1, org2, org1, org2)
	}
	/// RGBA
	pub fn straight() -> Self
	{
		ComponentMapping(ComponentSwizzle::R, ComponentSwizzle::G, ComponentSwizzle::B, ComponentSwizzle::A)
	}
	/// BGRA
	pub fn reversed() -> Self
	{
		ComponentMapping(ComponentSwizzle::B, ComponentSwizzle::G, ComponentSwizzle::R, ComponentSwizzle::A)
	}
}
impl Into<VkComponentMapping> for ComponentMapping { fn into(self) -> VkComponentMapping { unsafe { transmute(self) } } }

pub struct ImageView1D { parent: Image1D, internal: VkImageView, format: VkFormat }
pub struct ImageView2D { parent: Image2D, internal: VkImageView, format: VkFormat }
pub struct ImageView3D { parent: Image3D, internal: VkImageView, format: VkFormat }
impl ImageView1D
{
	pub fn make(res: &Image1D, format: VkFormat, cm: ComponentMapping, subrange: &ImageSubresourceRange) -> EngineResult<Self>
	{
		let mut iv = unsafe { reserved() };
		unsafe { vkCreateImageView((res.0).1.native(), &VkImageViewCreateInfo
		{
			image: transmute(res.internal()), viewType: VK_IMAGE_VIEW_TYPE_1D, format: format,
			components: cm.into(), subresourceRange: subrange.into(), .. Default::default()
		}, null(), &mut iv) }.make_result_with(|| ImageView1D { internal: iv, parent: res.clone(), format })
	}
}
impl ImageView2D
{
	pub fn make(res: &Image2D, format: VkFormat, cm: ComponentMapping, subrange: &ImageSubresourceRange) -> EngineResult<Self>
	{
		let mut iv = unsafe { reserved() };
		unsafe { vkCreateImageView((res.0).1.native(), &VkImageViewCreateInfo
		{
			image: transmute(res.internal()), viewType: VK_IMAGE_VIEW_TYPE_2D, format: format,
			components: cm.into(), subresourceRange: subrange.into(), .. Default::default()
		}, null(), &mut iv) }.make_result_with(|| ImageView2D { internal: iv, parent: res.clone(), format })
	}
}
impl ImageView3D
{
	pub fn make(res: &Image3D, format: VkFormat, cm: ComponentMapping, subrange: &ImageSubresourceRange) -> EngineResult<Self>
	{
		let mut iv = unsafe { reserved() };
		unsafe { vkCreateImageView((res.0).1.native(), &VkImageViewCreateInfo
		{
			image: transmute(res.internal()), viewType: VK_IMAGE_VIEW_TYPE_3D, format: format,
			components: cm.into(), subresourceRange: subrange.into(), .. Default::default()
		}, null(), &mut iv) }.make_result_with(|| ImageView3D { internal: iv, parent: res.clone(), format })
	}
}
impl Deref for ImageView1D { type Target = Image1D; fn deref(&self) -> &Image1D { &self.parent } }
impl Deref for ImageView2D { type Target = Image2D; fn deref(&self) -> &Image2D { &self.parent } }
impl Deref for ImageView3D { type Target = Image3D; fn deref(&self) -> &Image3D { &self.parent } }
pub trait ImageView
{
	fn internal(&self) -> u64;
	fn format(&self) -> VkFormat;
}
impl ImageView for ImageView1D
{
	fn internal(&self) -> u64 { self.internal as _ }
	fn format(&self) -> VkFormat { self.format }
}
impl ImageView for ImageView2D
{
	fn internal(&self) -> u64 { self.internal as _ }
	fn format(&self) -> VkFormat { self.format }
}
impl ImageView for ImageView3D
{
	fn internal(&self) -> u64 { self.internal as _ }
	fn format(&self) -> VkFormat { self.format }
}
impl Drop for ImageView1D { fn drop(&mut self) { unsafe { vkDestroyImageView(self.parent.0 .1.native(), self.internal, null()) }; } }
impl Drop for ImageView2D { fn drop(&mut self) { unsafe { vkDestroyImageView(self.parent.0 .1.native(), self.internal, null()) }; } }
impl Drop for ImageView3D { fn drop(&mut self) { unsafe { vkDestroyImageView(self.parent.0 .1.native(), self.internal, null()) }; } }

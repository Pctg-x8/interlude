
#![allow(dead_code)]

///! Interlude: Command Pool and Buffers

use super::internals::*;
use {std, vk, device, shading, resource};
use vk::*;
use vk::traits::*;
use std::rc::Rc;
use {EngineResult, GraphicsInterface, PreciseRenderPass, Framebuffer, AttachmentClearValue};
use std::ops::Deref;
use libc::c_void;

// Crate-level Object: Graphics, Transfer, TransientT, TransientG
pub struct CommandPool
{
	graphics: Rc<vk::CommandPool>, transfer: Rc<vk::CommandPool>,
	graphics_t: vk::CommandPool, transfer_t: vk::CommandPool
}
impl CommandPool
{
	pub fn new(device: &device::Device) -> EngineResult<Self>
	{
		let g = vk::CommandPool::new(device, &device.graphics_queue, false).map(Rc::new);
		let t = vk::CommandPool::new(device, &device.transfer_queue, false).map(Rc::new);
		let gt = vk::CommandPool::new(device, &device.graphics_queue, true);
		let tt = vk::CommandPool::new(device, &device.transfer_queue, true);

		(g, t, gt, tt).flatten().map(|(g,t, gt, tt)| CommandPool
		{
			graphics: g, transfer: t, graphics_t: gt, transfer_t: tt
		}).map_err(From::from)
	}
}

// Memory Barriers //
/// Defines a Memory Barrier
#[derive(Clone)]
pub struct MemoryBarrier { pub src_mask: VkAccessFlags, pub dst_mask: VkAccessFlags }
/// Defines a Buffer Memory Barrier
#[derive(Clone)]
pub struct BufferMemoryBarrier<'a>
{
	pub src_mask: VkAccessFlags, pub dst_mask: VkAccessFlags,
	pub src_queue_family_index: u32, pub dst_queue_family_index: u32,
	pub buffer: &'a Resource<Type = VkBuffer>, pub range: std::ops::Range<usize>
}
impl<'a> BufferMemoryBarrier<'a>
{
	pub fn hold_ownership(buf: &'a Resource<Type = VkBuffer>, range: std::ops::Range<usize>, src_mask: VkAccessFlags, dst_mask: VkAccessFlags)
		-> Self
	{
		BufferMemoryBarrier
		{
			src_mask: src_mask, dst_mask: dst_mask, src_queue_family_index: VK_QUEUE_FAMILY_IGNORED, dst_queue_family_index: VK_QUEUE_FAMILY_IGNORED,
			buffer: buf, range: range
		}
	}
	pub fn flipped_access_mask(&self) -> Self
	{
		BufferMemoryBarrier
		{
			src_mask: self.dst_mask, dst_mask: self.src_mask,
			src_queue_family_index: self.src_queue_family_index, dst_queue_family_index: self.dst_queue_family_index,
			buffer: self.buffer, range: self.range.clone()
		}
	}
}
/// Defines a Image Memory Barrier
#[derive(Clone)]
pub struct ImageMemoryBarrier<'a>
{
	src_mask: VkAccessFlags, dst_mask: VkAccessFlags,
	src_layout: VkImageLayout, dst_layout: VkImageLayout,
	src_queue_family_index: u32, dst_queue_family_index: u32,
	image: &'a Resource<Type = VkImage>, subresource_range: resource::ImageSubresourceRange
}
impl<'a> ImageMemoryBarrier<'a>
{
	pub fn template(img: &'a Resource<Type = VkImage>, subresource_range: resource::ImageSubresourceRange) -> ImageMemoryBarrierTemplate<'a>
	{
		ImageMemoryBarrierTemplate(img, subresource_range)
	}
	pub fn hold_ownership(img: &'a Resource<Type = VkImage>, subresource_range: resource::ImageSubresourceRange,
		src_mask: VkAccessFlags, dst_mask: VkAccessFlags, src_layout: VkImageLayout, dst_layout: VkImageLayout) -> Self
	{
		ImageMemoryBarrier
		{
			src_mask: src_mask, dst_mask: dst_mask, src_layout: src_layout, dst_layout: dst_layout,
			src_queue_family_index: VK_QUEUE_FAMILY_IGNORED, dst_queue_family_index: VK_QUEUE_FAMILY_IGNORED,
			image: img, subresource_range: subresource_range
		}
	}
	pub fn initialize(img: &'a Resource<Type = VkImage>, subresource_range: resource::ImageSubresourceRange, dst_mask: VkAccessFlags, dst_layout: VkImageLayout) -> Self
	{
		ImageMemoryBarrier
		{
			src_mask: 0, dst_mask: dst_mask, src_layout: VkImageLayout::Preinitialized, dst_layout: dst_layout,
			src_queue_family_index: VK_QUEUE_FAMILY_IGNORED, dst_queue_family_index: VK_QUEUE_FAMILY_IGNORED,
			image: img, subresource_range: subresource_range
		}
	}
}
/// A Template for constructing Image Memory Barrier.
/// This holds a reference of ImageResource and a ImageSubresourceRange
pub struct ImageMemoryBarrierTemplate<'a>(&'a Resource<Type = VkImage>, resource::ImageSubresourceRange);
impl<'a> ImageMemoryBarrierTemplate<'a>
{
	pub fn hold_ownership(&self, src_mask: VkAccessFlags, dst_mask: VkAccessFlags, src_layout: VkImageLayout, dst_layout: VkImageLayout)
		-> ImageMemoryBarrier<'a>
	{
		ImageMemoryBarrier::hold_ownership(self.0, self.1.clone(), src_mask, dst_mask, src_layout, dst_layout)
	}
	pub fn into_transfer_src(&self, src_mask: VkAccessFlags, src_layout: VkImageLayout) -> ImageMemoryBarrier<'a>
	{
		self.hold_ownership(src_mask, VK_ACCESS_TRANSFER_READ_BIT, src_layout, VkImageLayout::TransferSrcOptimal)
	}
	pub fn into_transfer_dst(&self, src_mask: VkAccessFlags, src_layout: VkImageLayout) -> ImageMemoryBarrier<'a>
	{
		self.hold_ownership(src_mask, VK_ACCESS_TRANSFER_WRITE_BIT, src_layout, VkImageLayout::TransferDestOptimal)
	}
	pub fn from_transfer_src(&self, dst_mask: VkAccessFlags, dst_layout: VkImageLayout) -> ImageMemoryBarrier<'a>
	{
		self.hold_ownership(VK_ACCESS_TRANSFER_READ_BIT, dst_mask, VkImageLayout::TransferSrcOptimal, dst_layout)
	}
	pub fn from_transfer_dst(&self, dst_mask: VkAccessFlags, dst_layout: VkImageLayout) -> ImageMemoryBarrier<'a>
	{
		self.hold_ownership(VK_ACCESS_TRANSFER_WRITE_BIT, dst_mask, VkImageLayout::TransferDestOptimal, dst_layout)
	}
	pub fn initialize(&self, dst_mask: VkAccessFlags, dst_layout: VkImageLayout) -> ImageMemoryBarrier<'a>
	{
		self.hold_ownership(0, dst_mask, VkImageLayout::Preinitialized, dst_layout)
	}
}
// NativeForms //
impl<'a> std::convert::Into<VkMemoryBarrier> for &'a MemoryBarrier
{
	fn into(self) -> VkMemoryBarrier
	{
		VkMemoryBarrier
		{
			sType: VkStructureType::MemoryBarrier, pNext: std::ptr::null(),
			srcAccessMask: self.src_mask, dstAccessMask: self.dst_mask
		}
	}
}
impl<'a> std::convert::Into<VkBufferMemoryBarrier> for &'a BufferMemoryBarrier<'a>
{
	fn into(self) -> VkBufferMemoryBarrier
	{
		VkBufferMemoryBarrier
		{
			sType: VkStructureType::BufferMemoryBarrier, pNext: std::ptr::null(),
			srcAccessMask: self.src_mask, dstAccessMask: self.dst_mask,
			srcQueueFamilyIndex: self.src_queue_family_index, dstQueueFamilyIndex: self.dst_queue_family_index,
			buffer: self.buffer.resource(), offset: self.range.start as VkDeviceSize, size: (self.range.end - self.range.start) as VkDeviceSize
		}
	}
}
impl <'a> std::convert::Into<VkImageMemoryBarrier> for &'a ImageMemoryBarrier<'a>
{
	fn into(self) -> VkImageMemoryBarrier
	{
		VkImageMemoryBarrier
		{
			sType: VkStructureType::ImageMemoryBarrier, pNext: std::ptr::null(),
			srcAccessMask: self.src_mask, dstAccessMask: self.dst_mask,
			oldLayout: self.src_layout, newLayout: self.dst_layout,
			srcQueueFamilyIndex: self.src_queue_family_index, dstQueueFamilyIndex: self.dst_queue_family_index,
			image: self.image.resource(), subresourceRange: (&self.subresource_range).into()
		}
	}
}

/// An element of Indirect Draw
/// (vertex_count, instance_count, first_vertex, first_instance)
pub struct IndirectCallParameter(pub u32, pub u32, pub u32, pub u32);

// Typedefs for BufferType and View //
pub type GraphicsCommandBuffer = VkCommandBuffer;
pub type TransferCommandBuffer = VkCommandBuffer;
pub type GraphicsCommandBuffersView = [GraphicsCommandBuffer];
pub type TransferCommandBuffersView = [TransferCommandBuffer];
pub type BundledCommandBuffersView = [VkCommandBuffer];

/// A set of command buffers which contains Graphics Commands and has to be dispatched to Graphics Queue.
pub struct GraphicsCommandBuffers(Vec<VkCommandBuffer>, Rc<vk::CommandPool>);
/// A set of command buffers which will be used in other command buffer.
pub struct BundledCommandBuffers(Vec<VkCommandBuffer>, Rc<vk::CommandPool>);
/// A set of command buffers which contains Transfer Commands and has to be dispatched to Transfer Queue.
pub struct TransferCommandBuffers(Vec<VkCommandBuffer>, Rc<vk::CommandPool>);
/// A set of command buffers which contains Transfer Commands and has to be sent once only.
pub struct TransientTransferCommandBuffers<'a>(Vec<VkCommandBuffer>, &'a vk::CommandPool, &'a vk::Queue);
/// A set of command buffers which contains Graphics Commands and has to be sent once only.
pub struct TransientGraphicsCommandBuffers<'a>(Vec<VkCommandBuffer>, &'a vk::CommandPool, &'a vk::Queue);
impl GraphicsCommandBuffers
{
	pub fn allocate(engine: &GraphicsInterface, count: usize) -> EngineResult<Self>
	{
		engine.pools().graphics.allocate(VkCommandBufferLevel::Primary, count)
			.map(|cs| GraphicsCommandBuffers(cs, engine.pools().graphics.clone())).map_err(From::from)
	}
}
impl BundledCommandBuffers
{
	pub fn allocate(engine: &GraphicsInterface, count: usize) -> EngineResult<Self>
	{
		engine.pools().graphics.allocate(VkCommandBufferLevel::Secondary, count)
			.map(|cs| BundledCommandBuffers(cs, engine.pools().graphics.clone())).map_err(From::from)
	}
}
impl TransferCommandBuffers
{
	pub fn allocate(engine: &GraphicsInterface, count: usize) -> EngineResult<Self>
	{
		engine.pools().transfer.allocate(VkCommandBufferLevel::Primary, count)
			.map(|cs| TransferCommandBuffers(cs, engine.pools().transfer.clone())).map_err(From::from)
	}
}
impl<'a> TransientTransferCommandBuffers<'a>
{
	pub fn allocate(engine: &'a GraphicsInterface, count: usize) -> EngineResult<Self>
	{
		engine.pools().transfer_t.allocate(VkCommandBufferLevel::Primary, count)
			.map(|cs| TransientTransferCommandBuffers(cs, &engine.pools().transfer_t, &engine.device().transfer_queue)).map_err(From::from)
	}
}
impl<'a> TransientGraphicsCommandBuffers<'a>
{
	pub fn allocate(engine: &'a GraphicsInterface, count: usize) -> EngineResult<Self>
	{
		engine.pools().graphics_t.allocate(VkCommandBufferLevel::Primary, count)
			.map(|cs| TransientGraphicsCommandBuffers(cs, &engine.pools().graphics_t, &engine.device().graphics_queue)).map_err(From::from)
	}
}
// Dereferencing Operations(Provides slice of command buffers) //
impl Deref for GraphicsCommandBuffers { type Target = GraphicsCommandBuffersView; fn deref(&self) -> &Self::Target { &self.0 } }
impl Deref for BundledCommandBuffers { type Target = BundledCommandBuffersView; fn deref(&self) -> &Self::Target { &self.0 } }
impl Deref for TransferCommandBuffers { type Target = TransferCommandBuffersView; fn deref(&self) -> &Self::Target { &self.0 } }
impl<'a> Deref for TransientTransferCommandBuffers<'a> { type Target = TransferCommandBuffersView; fn deref(&self) -> &Self::Target { &self.0 } }
impl<'a> Deref for TransientGraphicsCommandBuffers<'a> { type Target = GraphicsCommandBuffersView; fn deref(&self) -> &Self::Target { &self.0 } }
// Concurrency Supports //
unsafe impl Sync for GraphicsCommandBuffers {}
unsafe impl Send for GraphicsCommandBuffers {}
unsafe impl Sync for TransferCommandBuffers {}
unsafe impl Send for TransferCommandBuffers {}
// Builder Methods only in crate //
pub trait PersistentCommandBuffersNew { fn new(parent: &Rc<vk::CommandPool>, cbs: Vec<VkCommandBuffer>) -> Self; }
pub trait TransientCommandBuffersNew<'a> { fn new(parent: &'a vk::CommandPool, queue: &'a vk::Queue, cbs: Vec<VkCommandBuffer>) -> Self; }
macro_rules! CommandBuffersNewImpl
{
	([Persistent] $name: ident) =>
	{
		impl PersistentCommandBuffersNew for $name
		{
			fn new(parent: &Rc<vk::CommandPool>, cbs: Vec<VkCommandBuffer>) -> Self { $name(cbs, parent.clone()) }
		}
	};
	([Transient] $name: ident) =>
	{
		impl<'a> TransientCommandBuffersNew<'a> for $name<'a>
		{
			fn new(parent: &'a vk::CommandPool, queue: &'a vk::Queue, cbs: Vec<VkCommandBuffer>) -> Self { $name(cbs, parent, queue) }
		}
	}
}
CommandBuffersNewImpl!([Persistent] GraphicsCommandBuffers);
CommandBuffersNewImpl!([Persistent] BundledCommandBuffers);
CommandBuffersNewImpl!([Persistent] TransferCommandBuffers);
CommandBuffersNewImpl!([Transient] TransientTransferCommandBuffers);
CommandBuffersNewImpl!([Transient] TransientGraphicsCommandBuffers);
// Common Destroy Methods for all command buffers //
fn bufobj_common_drop(bufs: &[VkCommandBuffer], par: &vk::CommandPool)
{
	unsafe { vkFreeCommandBuffers(***par.parent(), **par, bufs.len() as u32, bufs.as_ptr()) };
}
impl Drop for GraphicsCommandBuffers { fn drop(&mut self) { bufobj_common_drop(&self.0, &self.1); } }
impl Drop for BundledCommandBuffers { fn drop(&mut self) { bufobj_common_drop(&self.0, &self.1); } }
impl Drop for TransferCommandBuffers { fn drop(&mut self) { bufobj_common_drop(&self.0, &self.1); } }
impl<'a> Drop for TransientTransferCommandBuffers<'a> { fn drop(&mut self) { bufobj_common_drop(&self.0, &self.1); } }
impl<'a> Drop for TransientGraphicsCommandBuffers<'a> { fn drop(&mut self) { bufobj_common_drop(&self.0, &self.1); } }

/// Indicates that `Self` is Primary Command Buffer which is recorded by `Self::Recorder`.
pub trait PrimaryCommandBuffers<'a>
{
	type Recorder: 'a;

	fn begin(&'a self, index: usize) -> Result<Self::Recorder, EngineError>;
	fn begin_all(&'a self) -> Result<std::vec::IntoIter<(usize, Self::Recorder)>, EngineError>;
}
/// Indicates that `Self` is Secondary Command Buffer which is recorded by `Self::Recorder`.
pub trait SecondaryCommandBuffers<'a>
{
	type Recorder: 'a;

	fn begin(&'a self, index: usize, cont_rp: PreciseRenderPass, cont_fb: &Framebuffer) -> Result<Self::Recorder, EngineError>;
}
impl<'a> PrimaryCommandBuffers<'a> for GraphicsCommandBuffers
{
	type Recorder = GraphicsCommandRecorder<'a>;

	fn begin(&'a self, index: usize) -> Result<GraphicsCommandRecorder, EngineError>
	{
		unsafe
		{
			vkBeginCommandBuffer(self.0[index], &VkCommandBufferBeginInfo
			{
				sType: VkStructureType::CommandBufferBeginInfo, pNext: std::ptr::null(),
				flags: 0, pInheritanceInfo: std::ptr::null()
			}).map(|| GraphicsCommandRecorder(&self.0[index])).map_err(EngineError::from)
		}
	}
	fn begin_all(&'a self) -> Result<std::vec::IntoIter<(usize, GraphicsCommandRecorder)>, EngineError>
	{
		self.0.iter().enumerate().map(|(i, x)|
		unsafe {
			vkBeginCommandBuffer(*x, &VkCommandBufferBeginInfo
			{
				sType: VkStructureType::CommandBufferBeginInfo, pNext: std::ptr::null(),
				flags: 0, pInheritanceInfo: std::ptr::null()
			}).map(|| (i, GraphicsCommandRecorder(&x)))
		}).collect::<Result<Vec<_>, _>>().map_err(EngineError::from).map(|x| x.into_iter())
	}
}
impl<'a> SecondaryCommandBuffers<'a> for BundledCommandBuffers
{
	type Recorder = BundleCommandRecorder<'a>;

	fn begin(&'a self, index: usize, cont_rp: PreciseRenderPass, cont_fb: &Framebuffer) -> Result<BundleCommandRecorder, EngineError>
	{
		let inheritance_info = VkCommandBufferInheritanceInfo
		{
			sType: VkStructureType::CommandBufferInheritanceInfo, pNext: std::ptr::null(),
			renderPass: **cont_rp.0.get_internal(), subpass: cont_rp.1, framebuffer: **cont_fb.get_internal(),
			occlusionQueryEnable: false as VkBool32, queryFlags: 0, pipelineStatistics: 0
		};
		unsafe
		{
			vkBeginCommandBuffer(self.0[index], &VkCommandBufferBeginInfo
			{
				sType: VkStructureType::CommandBufferBeginInfo, pNext: std::ptr::null(),
				flags: VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT, pInheritanceInfo: &inheritance_info
			}).map(|| BundleCommandRecorder(&self.0[index])).map_err(EngineError::from)
		}
	}
}
impl<'a> PrimaryCommandBuffers<'a> for TransferCommandBuffers
{
	type Recorder = TransferCommandRecorder<'a>;

	fn begin(&'a self, index: usize) -> Result<TransferCommandRecorder, EngineError>
	{
		unsafe
		{
			vkBeginCommandBuffer(self.0[index], &VkCommandBufferBeginInfo
			{
				sType: VkStructureType::CommandBufferBeginInfo, pNext: std::ptr::null(),
				flags: 0, pInheritanceInfo: std::ptr::null()
			}).map(|| TransferCommandRecorder(&self.0[index])).map_err(EngineError::from)
		}
	}
	fn begin_all(&'a self) -> Result<std::vec::IntoIter<(usize, TransferCommandRecorder)>, EngineError>
	{
		self.0.iter().enumerate().map(|(i, x)|
		unsafe {
			vkBeginCommandBuffer(*x, &VkCommandBufferBeginInfo
			{
				sType: VkStructureType::CommandBufferBeginInfo, pNext: std::ptr::null(),
				flags: 0, pInheritanceInfo: std::ptr::null()
			}).map(|| (i, TransferCommandRecorder(&x)))
		}).collect::<Result<Vec<_>, _>>().map_err(EngineError::from).map(|x| x.into_iter())
	}
}
impl<'a> PrimaryCommandBuffers<'a> for TransientTransferCommandBuffers<'a>
{
	type Recorder = TransferCommandRecorder<'a>;

	fn begin(&'a self, index: usize) -> Result<TransferCommandRecorder, EngineError>
	{
		unsafe
		{
			vkBeginCommandBuffer(self.0[index], &VkCommandBufferBeginInfo
			{
				sType: VkStructureType::CommandBufferBeginInfo, pNext: std::ptr::null(),
				flags: VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT, pInheritanceInfo: std::ptr::null()
			}).map(|| TransferCommandRecorder(&self.0[index])).map_err(EngineError::from)
		}
	}
	fn begin_all(&'a self) -> Result<std::vec::IntoIter<(usize, TransferCommandRecorder)>, EngineError>
	{
		self.0.iter().enumerate().map(|(i, x)|
		unsafe {
			vkBeginCommandBuffer(*x, &VkCommandBufferBeginInfo
			{
				sType: VkStructureType::CommandBufferBeginInfo, pNext: std::ptr::null(),
				flags: VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT, pInheritanceInfo: std::ptr::null()
			}).map(|| (i, TransferCommandRecorder(&x)))
		}).collect::<Result<Vec<_>, _>>().map_err(EngineError::from).map(|x| x.into_iter())
	}
}
impl<'a> PrimaryCommandBuffers<'a> for TransientGraphicsCommandBuffers<'a>
{
	type Recorder = GraphicsCommandRecorder<'a>;
	fn begin(&'a self, index: usize) -> Result<GraphicsCommandRecorder, EngineError>
	{
		unsafe
		{
			vkBeginCommandBuffer(self.0[index], &VkCommandBufferBeginInfo
			{
				sType: VkStructureType::CommandBufferBeginInfo, pNext: std::ptr::null(),
				flags: VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT, pInheritanceInfo: std::ptr::null()
			}).map(|| GraphicsCommandRecorder(&self.0[index])).map_err(EngineError::from)
		}
	}
	fn begin_all(&'a self) -> Result<std::vec::IntoIter<(usize, GraphicsCommandRecorder)>, EngineError>
	{
		self.0.iter().enumerate().map(|(i, x)| unsafe
		{
			vkBeginCommandBuffer(*x, &VkCommandBufferBeginInfo
			{
				sType: VkStructureType::CommandBufferBeginInfo, pNext: std::ptr::null(),
				flags: VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT, pInheritanceInfo: std::ptr::null()
			}).map(move || (i, GraphicsCommandRecorder(&x)))
		}).collect::<Result<Vec<_>, _>>().map_err(EngineError::from).map(|x| x.into_iter())
	}
}

impl <'a> TransientTransferCommandBuffers<'a>
{
	pub fn execute(self) -> Result<(), EngineError>
	{
		let subcmd = VkSubmitInfo
		{
			sType: VkStructureType::SubmitInfo, pNext: std::ptr::null(),
			waitSemaphoreCount: 0, pWaitSemaphores: std::ptr::null(), pWaitDstStageMask: std::ptr::null(),
			commandBufferCount: self.0.len() as u32, pCommandBuffers: self.0.as_ptr(),
			signalSemaphoreCount: 0, pSignalSemaphores: std::ptr::null()
		};
		self.2.submit(&[subcmd], None).and_then(|()| self.2.wait_for_idle()).map_err(EngineError::from)
	}
}
impl<'a> TransientGraphicsCommandBuffers<'a>
{
	pub fn execute(self, wait_semaphore: Option<(&QueueFence, VkPipelineStageFlags)>) -> Result<(), EngineError>
	{
		let (wsem, stage) = wait_semaphore.map(|(x, w)| (vec![qfence_raw(x)], vec![w])).unwrap_or_else(|| (Vec::new(), Vec::new()));
		let subcmd = VkSubmitInfo
		{
			sType: VkStructureType::SubmitInfo, pNext: std::ptr::null(),
			waitSemaphoreCount: wsem.len() as u32, pWaitSemaphores: wsem.as_ptr(), pWaitDstStageMask: stage.as_ptr(),
			commandBufferCount: self.0.len() as u32, pCommandBuffers: self.0.as_ptr(),
			signalSemaphoreCount: 0, pSignalSemaphores: std::ptr::null()
		};
		self.2.submit(&[subcmd], None).and_then(|_| self.2.wait_for_idle()).map_err(EngineError::from)
	}
}

/// An recording state of Graphics Command Buffer
pub struct GraphicsCommandRecorder<'a>(&'a VkCommandBuffer);
/// An recording state of Bundle(Secondary) Command Buffer
pub struct BundleCommandRecorder<'a>(&'a VkCommandBuffer);
/// An recording state of Transfer Command Buffer
pub struct TransferCommandRecorder<'a>(&'a VkCommandBuffer);
// common drops //
impl<'a> Drop for GraphicsCommandRecorder<'a> { fn drop(&mut self) { unsafe { vkEndCommandBuffer(*self.0) }; } }
impl<'a> Drop for BundleCommandRecorder<'a> { fn drop(&mut self) { unsafe { vkEndCommandBuffer(*self.0) }; } }
impl<'a> Drop for TransferCommandRecorder<'a> { fn drop(&mut self) { unsafe { vkEndCommandBuffer(*self.0) }; } }
/// Provides how to record some drawing commands
pub trait DrawingCommandRecorder
{
	fn bind_pipeline(self, pipeline: &GraphicsPipeline) -> Self;
	fn bind_descriptor_sets(self, layout: &shading::PipelineLayout, sets: &DescriptorSetArrayView) -> Self;
	fn bind_descriptor_sets_partial(self, layout: &shading::PipelineLayout, start_set: u32, sets: &DescriptorSetArrayView) -> Self;
	fn push_constants(self, layout: &shading::PipelineLayout, shader_stage: ShaderStage, range: std::ops::Range<u32>, data: &[f32]) -> Self;
	fn bind_vertex_buffers(self, buffer_offsets: &[(&Resource<Type = VkBuffer>, usize)]) -> Self;
	fn bind_vertex_buffers_partial(self, start_binding: u32, buffer_offsets: &[(&Resource<Type = VkBuffer>, usize)]) -> Self;
	fn bind_index_buffer(self, buffer: &Resource<Type = VkBuffer>, offset: usize) -> Self;
	
	fn draw(self, vertex_count: u32, instance_count: u32) -> Self;
	fn draw_with_voffs(self, vertex_count: u32, vertex_offset: u32, instance_count: u32) -> Self;
	fn draw_indexed(self, index_count: u32, instance_count: u32, index_offset: u32) -> Self;
	fn draw_indirect(self, param_buffer: &Resource<Type = VkBuffer>, param_offs: usize) -> Self;
	fn draw_indirect_mult(self, param_buffer: &Resource<Type = VkBuffer>, param_offs: usize, param_count: u32) -> Self;
}
macro_rules! DrawingCommandRecorderDefaultImpl
{
	($implementee: ident) =>
	{
		impl<'a> DrawingCommandRecorder for $implementee<'a>
		{
			fn bind_pipeline(self, pipeline: &GraphicsPipeline) -> Self
			{
				unsafe { vkCmdBindPipeline(*self.0, VkPipelineBindPoint::Graphics, **pipeline.get_internal()) };
				self
			}
			fn bind_descriptor_sets(self, layout: &shading::PipelineLayout, sets: &DescriptorSetArrayView) -> Self
			{
				self.bind_descriptor_sets_partial(layout, 0, sets)
			}
			fn bind_descriptor_sets_partial(self, layout: &shading::PipelineLayout, start_set: u32, sets: &DescriptorSetArrayView) -> Self
			{
				unsafe { vkCmdBindDescriptorSets(*self.0, VkPipelineBindPoint::Graphics, **layout.get_internal(),
					start_set, sets.len() as u32, sets.as_ptr(), 0, std::ptr::null()) };
				self
			}
			fn push_constants(self, layout: &shading::PipelineLayout, shader_stage: ShaderStage, range: std::ops::Range<u32>, data: &[f32]) -> Self
			{
				unsafe { vkCmdPushConstants(*self.0, **layout.get_internal(), shader_stage as VkShaderStageFlags,
					range.start, range.len() as u32, data.as_ptr() as *const c_void) };
				self
			}
			fn bind_vertex_buffers(self, buffer_offsets: &[(&Resource<Type = VkBuffer>, usize)]) -> Self
			{
				self.bind_vertex_buffers_partial(0, buffer_offsets)
			}
			fn bind_vertex_buffers_partial(self, start_binding: u32, buffer_offsets: &[(&Resource<Type = VkBuffer>, usize)]) -> Self
			{
				let (buffer_native, offsets_native): (Vec<_>, Vec<_>) = buffer_offsets.into_iter()
					.map(|&(b, v)| (b.resource(), v as VkDeviceSize)).unzip();
				unsafe { vkCmdBindVertexBuffers(*self.0, start_binding, buffer_native.len() as u32, buffer_native.as_ptr(), offsets_native.as_ptr()) };
				self
			}
			fn bind_index_buffer(self, buffer: &Resource<Type = VkBuffer>, offset: usize) -> Self
			{
				unsafe { vkCmdBindIndexBuffer(*self.0, buffer.resource(), offset as VkDeviceSize, VkIndexType::U16) };
				self
			}
			
			fn draw(self, vertex_count: u32, instance_count: u32) -> Self
			{
				unsafe { vkCmdDraw(*self.0, vertex_count, instance_count, 0, 0) };
				self
			}
			fn draw_with_voffs(self, vertex_count: u32, vertex_offset: u32, instance_count: u32) -> Self
			{
				unsafe { vkCmdDraw(*self.0, vertex_count, instance_count, vertex_offset, 0) };
				self
			}
			fn draw_indexed(self, index_count: u32, instance_count: u32, index_offset: u32) -> Self
			{
				unsafe { vkCmdDrawIndexed(*self.0, index_count, instance_count, 0, index_offset, 0) };
				self
			}
			fn draw_indirect(self, param_buffer: &Resource<Type = VkBuffer>, param_offs: usize) -> Self
			{
				unsafe { vkCmdDrawIndirect(*self.0, param_buffer.resource(), param_offs as VkDeviceSize, 1, 0) };
				self
			}
			fn draw_indirect_mult(self, param_buffer: &Resource<Type = VkBuffer>, param_offs: usize, param_count: u32) -> Self
			{
				unsafe { vkCmdDrawIndirect(*self.0, param_buffer.resource(), param_offs as VkDeviceSize,
					param_count, std::mem::size_of::<VkDrawIndirectCommand>() as u32) };
				self
			}
		}
	}
}
DrawingCommandRecorderDefaultImpl!(GraphicsCommandRecorder);
DrawingCommandRecorderDefaultImpl!(BundleCommandRecorder);
// provide depending commands //
impl<'a> GraphicsCommandRecorder<'a>
{
	pub fn pipeline_barrier(self, src_stage_mask: VkPipelineStageFlags, dst_stage_mask: VkPipelineStageFlags,
		depend_by_region: bool, memory_barriers: &[MemoryBarrier], buffer_barriers: &[BufferMemoryBarrier], image_barriers: &[ImageMemoryBarrier]) -> Self
	{
		let (mbs_native, bbs_native, ibs_native) = (
			memory_barriers.into_iter().map(|x| x.into()).collect::<Vec<_>>(),
			buffer_barriers.into_iter().map(|x| x.into()).collect::<Vec<_>>(),
			image_barriers.into_iter().map(|x| x.into()).collect::<Vec<_>>()
		);
		unsafe { vkCmdPipelineBarrier(*self.0, src_stage_mask, dst_stage_mask,
			if depend_by_region { VK_DEPENDENCY_BY_REGION_BIT } else { 0 },
			mbs_native.len() as u32, mbs_native.as_ptr(),
			bbs_native.len() as u32, bbs_native.as_ptr(),
			ibs_native.len() as u32, ibs_native.as_ptr()) };
		self
	}

	pub fn begin_render_pass(self, framebuffer: &Framebuffer, clear_values: &[AttachmentClearValue], use_bundles: bool) -> Self
	{
		let clear_values_native = clear_values.into_iter().map(|x| x.into()).collect::<Vec<_>>();
		let begin_info = VkRenderPassBeginInfo
		{
			sType: VkStructureType::RenderPassBeginInfo, pNext: std::ptr::null(),
			renderPass: **framebuffer.renderpass().get_internal(), framebuffer: **framebuffer.get_internal(),
			renderArea: VkRect2D(VkOffset2D(0, 0), *framebuffer.area()),
			clearValueCount: clear_values_native.len() as u32, pClearValues: clear_values_native.as_ptr()
		};
		unsafe { vkCmdBeginRenderPass(*self.0, &begin_info,
			if use_bundles { VkSubpassContents::SecondaryCommandBuffers } else { VkSubpassContents::Inline }) };
		self
	}
	pub fn next_subpass(self, use_bundles: bool) -> Self
	{
		unsafe { vkCmdNextSubpass(*self.0, if use_bundles { VkSubpassContents::SecondaryCommandBuffers } else { VkSubpassContents::Inline }) };
		self
	}
	pub fn end_render_pass(self) -> Self
	{
		unsafe { vkCmdEndRenderPass(*self.0) };
		self
	}

	pub fn execute_commands(self, buffers: &BundledCommandBuffersView) -> Self
	{
		unsafe { vkCmdExecuteCommands(*self.0, buffers.len() as u32, buffers.as_ptr()) };
		self
	}
	
	pub fn blit_image(self, src: &Resource<Type = VkImage>, dst: &Resource<Type = VkImage>, src_layout: VkImageLayout, dst_layout: VkImageLayout,
		regions: &[ImageBlitRegion], filter: Filter) -> Self
	{
		let regions_native = regions.into_iter().map(|&x| x.into()).collect::<Vec<_>>();
		unsafe { vkCmdBlitImage(*self.0, src.resource(), src_layout, dst.resource(), dst_layout,
			regions_native.len() as u32, regions_native.as_ptr(), filter.into()) };
		self
	}
}
impl<'a> TransferCommandRecorder<'a>
{
	pub fn pipeline_barrier(self, src_stage_mask: VkPipelineStageFlags, dst_stage_mask: VkPipelineStageFlags,
		depend_by_region: bool, memory_barriers: &[MemoryBarrier], buffer_barriers: &[BufferMemoryBarrier], image_barriers: &[ImageMemoryBarrier]) -> Self
	{
		let (mbs_native, bbs_native, ibs_native) = (
			memory_barriers.into_iter().map(|x| x.into()).collect::<Vec<_>>(),
			buffer_barriers.into_iter().map(|x| x.into()).collect::<Vec<_>>(),
			image_barriers.into_iter().map(|x| x.into()).collect::<Vec<_>>()
		);
		unsafe { vkCmdPipelineBarrier(*self.0, src_stage_mask, dst_stage_mask,
			if depend_by_region { VK_DEPENDENCY_BY_REGION_BIT } else { 0 },
			mbs_native.len() as u32, mbs_native.as_ptr(),
			bbs_native.len() as u32, bbs_native.as_ptr(),
			ibs_native.len() as u32, ibs_native.as_ptr()) };
		self
	}

	pub fn copy_buffer(self, src: &Resource<Type = VkBuffer>, dst: &Resource<Type = VkBuffer>, regions: &[BufferCopyRegion]) -> Self
	{
		let regions_native = regions.into_iter().map(|&x| x.into()).collect::<Vec<_>>();
		unsafe { vkCmdCopyBuffer(*self.0, src.resource(), dst.resource(),
			regions_native.len() as u32, regions_native.as_ptr()) };
		self
	}
	pub fn copy_image(self, src: &Resource<Type = VkImage>, dst: &Resource<Type = VkImage>, regions: &[ImageCopyRegion]) -> Self
	{
		let regions_native = regions.into_iter().map(|&x| x.into()).collect::<Vec<_>>();
		unsafe { vkCmdCopyImage(*self.0, src.resource(), VkImageLayout::TransferSrcOptimal,
			dst.resource(), VkImageLayout::TransferDestOptimal, regions_native.len() as u32, regions_native.as_ptr()) };
		self
	}
}
macro_rules! RecorderCommon
{
	(for $t: ty) =>
	{
		impl<'a> $t
		{
			pub fn end(self) -> Result<(), EngineError>
			{
				unsafe { vkEndCommandBuffer(*self.0) }.and_then(|| { std::mem::forget(self); Ok(()) }).map_err(From::from)
			}
			pub fn inject_commands<F>(self, f: F) -> Self where F: FnOnce(Self) -> Self
			{
				f(self)
			}
		}
	}
}
RecorderCommon!(for GraphicsCommandRecorder<'a>);
RecorderCommon!(for BundleCommandRecorder<'a>);
RecorderCommon!(for TransferCommandRecorder<'a>);

#[derive(Clone, Copy)]
pub struct BufferCopyRegion(pub usize, pub usize, pub usize);		// src, dst, size
impl std::convert::Into<VkBufferCopy> for BufferCopyRegion
{
	fn into(self) -> VkBufferCopy
	{
		let BufferCopyRegion(src, dst, size) = self;
		VkBufferCopy(src as VkDeviceSize, dst as VkDeviceSize, size as VkDeviceSize)
	}
}
#[derive(Clone, Copy)]
// src_layers, src_offset, dst_layers, dst_offset, extent
pub struct ImageCopyRegion(pub ImageSubresourceLayers, pub VkOffset3D, pub ImageSubresourceLayers, pub VkOffset3D, pub VkExtent3D);
impl std::convert::Into<VkImageCopy> for ImageCopyRegion
{
	fn into(self) -> VkImageCopy
	{
		let ImageCopyRegion(sl, so, dl, _do, ex) = self;
		VkImageCopy(sl.into(), so, dl.into(), _do, ex)
	}
}
impl ImageCopyRegion
{
	pub fn entire_colorbits(size: VkExtent3D) -> Self
	{
		ImageCopyRegion(ImageSubresourceLayers::base_color(), VkOffset3D(0, 0, 0),
			ImageSubresourceLayers::base_color(), VkOffset3D(0, 0, 0), size)
	}
}
#[derive(Clone, Copy)]
pub struct ImageBlitRegion(pub ImageSubresourceLayers, pub [VkOffset3D; 2], pub ImageSubresourceLayers, pub [VkOffset3D; 2]);
impl std::convert::Into<VkImageBlit> for ImageBlitRegion
{
	fn into(self) -> VkImageBlit
	{
		let ImageBlitRegion(sl, so, dl, _do) = self;
		VkImageBlit
		{
			srcSubresource: sl.into(), dstSubresource: dl.into(),
			srcOffsets: so, dstOffsets: _do
		}
	}
}
impl ImageBlitRegion
{
	pub fn same_region(src_subres: ImageSubresourceLayers, dst_subres: ImageSubresourceLayers, offs: VkOffset3D, size: VkOffset3D) -> Self
	{
		ImageBlitRegion(src_subres, [offs, size], dst_subres, [offs, size])
	}
}

//! Interlude: Command Pool and Buffers

use interlude_vk_defs::*;
use interlude_vk_funport::*;
use device::Device;
use {
	ImageSubresourceLayers, EngineResult, ImageResource, BufferResource, Filter, AttachmentClearValue, GraphicsInterface,
	Framebuffer, PipelineLayout, DescriptorSetArrayView, GraphicsPipeline, QueueFence, Fence, PreciseRenderPass,
	ImageSubresourceRange, Size3, Offset3, UnrecoverableExt, AccessFlags, ImageLayout, PipelineStageFlag
};
use shading::ShaderStageSet;
use std::ops::{Deref, Range};
use std::mem::{forget, size_of, uninitialized as reserved, zeroed, transmute};
use std::ptr::null;
use subsystem_layer::{NativeResultValueHandler, NativeHandleProvider};
use std::rc::Rc;

use subsystem_layer::NativeCommandPool;
pub struct CommandPoolPair { persistent: Rc<NativeCommandPool>, transient: Rc<NativeCommandPool> }
pub struct CommandPool { graphics: CommandPoolPair, transfer: CommandPoolPair }
impl CommandPool
{
	pub fn new(device: &Rc<Device>) -> EngineResult<Self>
	{
		let (mut g, mut t, mut gt, mut tt) = unsafe { reserved() };
		unsafe
		{
			let create = |cinfo, t| vkCreateCommandPool(device.native(), &cinfo, null(), t).into_result();
			let transient = VkCommandPoolCreateInfo { flags: VK_COMMAND_POOL_CREATE_TRANSIENT_BIT, .. Default::default() };
			create(VkCommandPoolCreateInfo { queueFamilyIndex: device.graphics_qf_index, .. Default::default() }, &mut g)?;
			create(VkCommandPoolCreateInfo { queueFamilyIndex: device.transfer_qf_index, .. Default::default() }, &mut t)?;
			create(VkCommandPoolCreateInfo { queueFamilyIndex: device.graphics_qf_index, .. transient }, &mut gt)?;
			create(VkCommandPoolCreateInfo { queueFamilyIndex: device.transfer_qf_index, .. transient }, &mut tt)?;
		}
		Ok(CommandPool
		{
			graphics: CommandPoolPair { persistent: Rc::new(NativeCommandPool(g, device.clone())), transient: Rc::new(NativeCommandPool(gt, device.clone())) },
			transfer: CommandPoolPair { persistent: Rc::new(NativeCommandPool(t, device.clone())), transient: Rc::new(NativeCommandPool(tt, device.clone())) }
		})
	}
}

// Memory Barriers //
/// Defines a Memory Barrier
#[derive(Clone)]
pub struct MemoryBarrier { pub src_access: AccessFlags, pub dst_access: AccessFlags }
/// Defines a Buffer Memory Barrier
#[derive(Clone)]
pub struct BufferMemoryBarrier<'a>
{
	pub src_access: AccessFlags, pub dst_access: AccessFlags,
	pub src_queue_family_index: u32, pub dst_queue_family_index: u32,
	pub buffer: &'a BufferResource, pub range: Range<usize>
}
impl<'a> Default for BufferMemoryBarrier<'a>
{
	/// No queue release operation, No accesses in first and seconds access scopes
	fn default() -> Self
	{
		BufferMemoryBarrier
		{
			src_queue_family_index: VK_QUEUE_FAMILY_IGNORED, dst_queue_family_index: VK_QUEUE_FAMILY_IGNORED,
			.. unsafe { zeroed() }
		}
	}
}
impl<'a> BufferMemoryBarrier<'a>
{
	/// Flip Access Mask and Queue Family Index
	pub fn flip(&self) -> Self
	{
		BufferMemoryBarrier
		{
			src_access: self.dst_access, dst_access: self.src_access,
			src_queue_family_index: self.src_queue_family_index, dst_queue_family_index: self.dst_queue_family_index,
			buffer: self.buffer, range: self.range.clone()
		}
	}
}
/// Defines a Image Memory Barrier
#[derive(Clone)]
pub struct ImageMemoryBarrier<'a>
{
	pub src_access: AccessFlags, pub dst_access: AccessFlags,
	pub src_layout: ImageLayout, pub dst_layout: ImageLayout,
	pub src_queue_family_index: u32, pub dst_queue_family_index: u32,
	pub image: &'a ImageResource, pub subresource_range: ImageSubresourceRange
}
impl<'a> Default for ImageMemoryBarrier<'a>
{
	/// Undefined to Undefined Layout, No queue release operation, No accesses in first and seconds access scopes
	fn default() -> Self
	{
		ImageMemoryBarrier
		{
			src_queue_family_index: VK_QUEUE_FAMILY_IGNORED, dst_queue_family_index: VK_QUEUE_FAMILY_IGNORED,
			.. unsafe { zeroed() }
		}
	}
}
impl<'a> ImageMemoryBarrier<'a>
{
	/// Initialize operation(set image layout from Preinitialized)
	pub fn initialize(image: &'a ImageResource, subresource_range: ImageSubresourceRange, dst_access: AccessFlags, dst_layout: ImageLayout) -> Self
	{
		ImageMemoryBarrier
		{
			dst_access, src_layout: ImageLayout::Preinitialized, dst_layout, image, subresource_range, .. Default::default()
		}
	}
	/// Initialize operation(set image layout from Undefined)
	pub fn initialize_undef(image: &'a ImageResource, subresource_range: ImageSubresourceRange, dst_access: AccessFlags, dst_layout: ImageLayout) -> Self
	{
		ImageMemoryBarrier
		{
			dst_access, src_layout: ImageLayout::Undefined, dst_layout, image, subresource_range, .. Default::default()
		}
	}
}
// NativeForms //
impl<'a> Into<VkMemoryBarrier> for &'a MemoryBarrier
{
	fn into(self) -> VkMemoryBarrier
	{
		VkMemoryBarrier
		{
			srcAccessMask: self.src_access.into(), dstAccessMask: self.dst_access.into(), .. Default::default()
		}
	}
}
impl<'a> Into<VkBufferMemoryBarrier> for &'a BufferMemoryBarrier<'a>
{
	fn into(self) -> VkBufferMemoryBarrier
	{
		VkBufferMemoryBarrier
		{
			srcAccessMask: self.src_access.into(), dstAccessMask: self.dst_access.into(),
			srcQueueFamilyIndex: self.src_queue_family_index, dstQueueFamilyIndex: self.dst_queue_family_index,
			buffer: self.buffer.internal() as _, offset: self.range.start as _, size: (self.range.end - self.range.start) as _,
			.. Default::default()
		}
	}
}
impl<'a> Into<VkImageMemoryBarrier> for &'a ImageMemoryBarrier<'a>
{
	fn into(self) -> VkImageMemoryBarrier
	{
		VkImageMemoryBarrier
		{
			srcAccessMask: self.src_access.into(), dstAccessMask: self.dst_access.into(),
			oldLayout: self.src_layout as _, newLayout: self.dst_layout as _,
			srcQueueFamilyIndex: self.src_queue_family_index, dstQueueFamilyIndex: self.dst_queue_family_index,
			image: self.image.internal() as _, subresourceRange: (&self.subresource_range).into(), .. Default::default()
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
pub struct GraphicsCommandBuffers(Vec<VkCommandBuffer>, Rc<NativeCommandPool>);
/// A set of command buffers which will be used in other command buffer.
pub struct BundledCommandBuffers(Vec<VkCommandBuffer>, Rc<NativeCommandPool>);
/// A set of command buffers which contains Transfer Commands and has to be dispatched to Transfer Queue.
pub struct TransferCommandBuffers(Vec<VkCommandBuffer>, Rc<NativeCommandPool>);
/// A set of command buffers which contains Transfer Commands and has to be sent once only.
pub struct TransientTransferCommandBuffers<'a>(Vec<VkCommandBuffer>, &'a NativeCommandPool, VkQueue);
/// A set of command buffers which contains Graphics Commands and has to be sent once only.
pub struct TransientGraphicsCommandBuffers<'a>(Vec<VkCommandBuffer>, &'a NativeCommandPool, VkQueue);
impl GraphicsCommandBuffers
{
	/// Creates some command buffers
	pub fn new(engine: &GraphicsInterface, count: usize) -> EngineResult<Self>
	{
		let mut buffers = vec![unsafe { zeroed() }; count];
		unsafe { vkAllocateCommandBuffers(engine.device().native(), &VkCommandBufferAllocateInfo
		{
			commandPool: engine.pools().graphics.persistent.native(), level: VK_COMMAND_BUFFER_LEVEL_PRIMARY, commandBufferCount: count as _,
			.. Default::default()
		}, buffers.as_mut_ptr()) }.make_result_with(|| GraphicsCommandBuffers(buffers, engine.pools().graphics.persistent.clone()))
	}
}
impl BundledCommandBuffers
{
	/// Creates some command buffers
	pub fn new(engine: &GraphicsInterface, count: usize) -> EngineResult<Self>
	{
		let mut buffers = vec![unsafe { zeroed() }; count];
		unsafe { vkAllocateCommandBuffers(engine.device().native(), &VkCommandBufferAllocateInfo
		{
			commandPool: engine.pools().graphics.persistent.native(), level: VK_COMMAND_BUFFER_LEVEL_SECONDARY, commandBufferCount: count as _,
			.. Default::default()
		}, buffers.as_mut_ptr()) }.make_result_with(|| BundledCommandBuffers(buffers, engine.pools().graphics.persistent.clone()))
	}
}
impl TransferCommandBuffers
{
	/// Creates some command buffers
	pub fn new(engine: &GraphicsInterface, count: usize) -> EngineResult<Self>
	{
		let mut buffers = vec![unsafe { zeroed() }; count];
		unsafe { vkAllocateCommandBuffers(engine.device().native(), &VkCommandBufferAllocateInfo
		{
			commandPool: engine.pools().transfer.persistent.native(), level: VK_COMMAND_BUFFER_LEVEL_PRIMARY, commandBufferCount: count as _,
			.. Default::default()
		}, buffers.as_mut_ptr()) }.make_result_with(|| TransferCommandBuffers(buffers, engine.pools().transfer.persistent.clone()))
	}
}
impl<'a> TransientTransferCommandBuffers<'a>
{
	/// Creates some command buffers
	pub fn new(engine: &'a GraphicsInterface, count: usize) -> EngineResult<Self>
	{
		let mut buffers = vec![unsafe { zeroed() }; count];
		unsafe { vkAllocateCommandBuffers(engine.device().native(), &VkCommandBufferAllocateInfo
		{
			commandPool: engine.pools().transfer.transient.native(), level: VK_COMMAND_BUFFER_LEVEL_PRIMARY, commandBufferCount: count as _,
			.. Default::default()
		}, buffers.as_mut_ptr()) }.make_result_with(|| TransientTransferCommandBuffers(buffers, &engine.pools().transfer.transient, engine.device().transfer_queue))
	}
}
impl<'a> TransientGraphicsCommandBuffers<'a>
{
	/// Creates some command buffers
	pub fn new(engine: &'a GraphicsInterface, count: usize) -> EngineResult<Self>
	{
		let mut buffers = vec![unsafe { zeroed() }; count];
		unsafe { vkAllocateCommandBuffers(engine.device().native(), &VkCommandBufferAllocateInfo
		{
			commandPool: engine.pools().graphics.transient.native(), level: VK_COMMAND_BUFFER_LEVEL_PRIMARY, commandBufferCount: count as _,
			.. Default::default()
		}, buffers.as_mut_ptr()) }.make_result_with(|| TransientGraphicsCommandBuffers(buffers, &engine.pools().graphics.transient, engine.device().graphics_queue))
	}
}
// Dereferencing Operations(Provides slice of command buffers) //
impl Deref for GraphicsCommandBuffers { type Target = GraphicsCommandBuffersView; fn deref(&self) -> &Self::Target { &self.0 } }
impl Deref for BundledCommandBuffers { type Target = BundledCommandBuffersView;  fn deref(&self) -> &Self::Target { &self.0 } }
impl Deref for TransferCommandBuffers { type Target = TransferCommandBuffersView; fn deref(&self) -> &Self::Target { &self.0 } }
impl<'a> Deref for TransientGraphicsCommandBuffers<'a> { type Target = GraphicsCommandBuffersView; fn deref(&self) -> &Self::Target { &self.0 } }
impl<'a> Deref for TransientTransferCommandBuffers<'a> { type Target = TransferCommandBuffersView; fn deref(&self) -> &Self::Target { &self.0 } }
// Concurrency Supports //
unsafe impl Sync for GraphicsCommandBuffers {}
unsafe impl Send for GraphicsCommandBuffers {}
unsafe impl Sync for TransferCommandBuffers {}
unsafe impl Send for TransferCommandBuffers {}
// Common Destroy Methods for all command buffers //
fn bufobj_common_drop(bufs: &[VkCommandBuffer], cp: &NativeCommandPool)
{
	unsafe { vkFreeCommandBuffers(cp.1.native(), cp.native(), bufs.len() as u32, bufs.as_ptr()) };
}
impl Drop for GraphicsCommandBuffers { fn drop(&mut self) { bufobj_common_drop(&self.0, &self.1); } }
impl Drop for BundledCommandBuffers  { fn drop(&mut self) { bufobj_common_drop(&self.0, &self.1); } }
impl Drop for TransferCommandBuffers { fn drop(&mut self) { bufobj_common_drop(&self.0, &self.1); } }
impl<'a> Drop for TransientTransferCommandBuffers<'a> { fn drop(&mut self) { bufobj_common_drop(&self.0, &self.1); } }
impl<'a> Drop for TransientGraphicsCommandBuffers<'a> { fn drop(&mut self) { bufobj_common_drop(&self.0, &self.1); } }

fn begin_graphics_command_recording<'a>(cb: &'a VkCommandBuffer) -> EngineResult<GraphicsCommandRecorder<'a>>
{
	unsafe { vkBeginCommandBuffer(*cb, &Default::default()) }.make_result_with(|| GraphicsCommandRecorder(cb))
}
fn begin_transfer_command_recording<'a>(cb: &'a VkCommandBuffer) -> EngineResult<TransferCommandRecorder<'a>>
{
	unsafe { vkBeginCommandBuffer(*cb, &Default::default()) }.make_result_with(|| TransferCommandRecorder(cb))
}
fn begin_graphics_command_recording_onetime<'a>(cb: &'a VkCommandBuffer) -> EngineResult<GraphicsCommandRecorder<'a>>
{
	unsafe { vkBeginCommandBuffer(*cb, &VkCommandBufferBeginInfo { flags: VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT, .. Default::default() }) }
		.make_result_with(|| GraphicsCommandRecorder(cb))
}
fn begin_transfer_command_recording_onetime<'a>(cb: &'a VkCommandBuffer) -> EngineResult<TransferCommandRecorder<'a>>
{
	unsafe { vkBeginCommandBuffer(*cb, &VkCommandBufferBeginInfo { flags: VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT, .. Default::default() }) }
		.make_result_with(|| TransferCommandRecorder(cb))
}
use std::slice::Iter as SliceIter;
use std::iter::Map;
pub type BeginningCommandBuffersIter<'a, RecT: 'a> = Map<SliceIter<'a, VkCommandBuffer>, fn(&'a VkCommandBuffer) -> EngineResult<RecT>>;
/// Indicates that `Self` is Primary Command Buffer which is recorded by `Self::Recorder`.
pub trait PrimaryCommandBuffers<'a>
{
	type Recorder: 'a;

	fn begin(&'a self, index: usize) -> EngineResult<Self::Recorder>;
	fn begin_all(&'a self) -> BeginningCommandBuffersIter<'a, Self::Recorder>;
}
/// Indicates that `Self` is Secondary Command Buffer which is recorded by `Self::Recorder`.
pub trait SecondaryCommandBuffers<'a>
{
	type Recorder: 'a;

	fn begin(&'a self, index: usize, cont_rp: PreciseRenderPass, cont_fb: &Framebuffer) -> EngineResult<Self::Recorder>;
}
impl<'a> PrimaryCommandBuffers<'a> for GraphicsCommandBuffers
{
	type Recorder = GraphicsCommandRecorder<'a>;

	fn begin(&'a self, index: usize) -> EngineResult<Self::Recorder>
	{
		unsafe { vkBeginCommandBuffer(self.0[index], &Default::default()) }.make_result_with(|| GraphicsCommandRecorder(&self.0[index]))
	}
	fn begin_all(&'a self) -> BeginningCommandBuffersIter<'a, Self::Recorder>
	{
		self.0.iter().map(begin_graphics_command_recording)
	}
}
impl<'a> SecondaryCommandBuffers<'a> for BundledCommandBuffers
{
	type Recorder = BundleCommandRecorder<'a>;

	fn begin(&'a self, index: usize, cont_rp: PreciseRenderPass, cont_fb: &Framebuffer) -> EngineResult<BundleCommandRecorder>
	{
		let inheritance_info = VkCommandBufferInheritanceInfo
		{
			renderPass: cont_rp.0.native(), subpass: cont_rp.1, framebuffer: cont_fb.native(), .. Default::default()
		};
		unsafe
		{
			vkBeginCommandBuffer(self.0[index], &VkCommandBufferBeginInfo
			{
				flags: VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT, pInheritanceInfo: &inheritance_info, .. Default::default()
			}).make_result_with(|| BundleCommandRecorder(&self.0[index]))
		}
	}
}
impl<'a> PrimaryCommandBuffers<'a> for TransferCommandBuffers
{
	type Recorder = TransferCommandRecorder<'a>;

	fn begin(&'a self, index: usize) -> EngineResult<TransferCommandRecorder>
	{
		unsafe
		{
			vkBeginCommandBuffer(self.0[index], &Default::default())
				.make_result_with(|| TransferCommandRecorder(&self.0[index]))
		}
	}
	fn begin_all(&'a self) -> BeginningCommandBuffersIter<'a, Self::Recorder>
	{
		self.0.iter().map(begin_transfer_command_recording)
	}
}
impl<'a> PrimaryCommandBuffers<'a> for TransientTransferCommandBuffers<'a>
{
	type Recorder = TransferCommandRecorder<'a>;

	fn begin(&'a self, index: usize) -> EngineResult<TransferCommandRecorder>
	{
		unsafe
		{
			vkBeginCommandBuffer(self.0[index], &VkCommandBufferBeginInfo
			{
				flags: VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT, .. Default::default()
			}).make_result_with(|| TransferCommandRecorder(&self.0[index]))
		}
	}
	fn begin_all(&'a self) -> BeginningCommandBuffersIter<'a, Self::Recorder>
	{
		self.0.iter().map(begin_transfer_command_recording_onetime)
	}
}
impl<'a> PrimaryCommandBuffers<'a> for TransientGraphicsCommandBuffers<'a>
{
	type Recorder = GraphicsCommandRecorder<'a>;
	fn begin(&'a self, index: usize) -> EngineResult<GraphicsCommandRecorder>
	{
		unsafe
		{
			vkBeginCommandBuffer(self.0[index], &VkCommandBufferBeginInfo
			{
				flags: VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT, .. Default::default()
			}).make_result_with(|| GraphicsCommandRecorder(&self.0[index]))
		}
	}
	fn begin_all(&'a self) -> BeginningCommandBuffersIter<'a, Self::Recorder>
	{
		self.0.iter().map(begin_graphics_command_recording_onetime)
	}
}

impl<'a> TransientTransferCommandBuffers<'a>
{
	pub fn execute(self) -> EngineResult<()>
	{
		unsafe { vkQueueSubmit(self.2, 1, &VkSubmitInfo
		{
			commandBufferCount: self.0.len() as _, pCommandBuffers: self.0.as_ptr(), .. Default::default()
		}, zeroed()) }.into_result()?;
		unsafe { vkQueueWaitIdle(self.2) }.into_result()
	}
}
impl<'a> TransientGraphicsCommandBuffers<'a>
{
	pub fn execute(self, wait_semaphore: Option<(&QueueFence, &PipelineStageFlag)>) -> EngineResult<()>
	{
		let (wsem, stage) = wait_semaphore.map(|(x, w)| (vec![x.native()], vec![w.into_flag()])).unwrap_or_else(|| (Vec::new(), Vec::new()));
		unsafe { vkQueueSubmit(self.2, 1, &VkSubmitInfo
		{
			waitSemaphoreCount: wsem.len() as _, pWaitSemaphores: wsem.as_ptr(), pWaitDstStageMask: stage.as_ptr(),
			commandBufferCount: self.0.len() as _, pCommandBuffers: self.0.as_ptr(), .. Default::default()
		}, zeroed()) }.into_result()?;
		unsafe { vkQueueWaitIdle(self.2) }.into_result()
	}
}

/// Immediately submitted commands for transfer operations
pub struct ImmediateTransferCommandSubmission<'a>(VkCommandBuffer, &'a NativeCommandPool, VkQueue);
/// Immediately submitted commands for graphics operations
pub struct ImmediateGraphicsCommandSubmission<'a>(VkCommandBuffer, &'a NativeCommandPool, VkQueue);
impl<'a> ImmediateTransferCommandSubmission<'a>
{
	/// Begin recording commands for immediate submission
	pub fn begin(engine: &'a GraphicsInterface) -> EngineResult<Self>
	{
		let mut v = unsafe { reserved() };
		let cs = unsafe { vkAllocateCommandBuffers(engine.device().native(), &VkCommandBufferAllocateInfo
		{
			commandPool: engine.pools().transfer.transient.native(), level: VK_COMMAND_BUFFER_LEVEL_PRIMARY, commandBufferCount: 1,
			.. Default::default()
		}, &mut v) }.make_result_with(|| ImmediateTransferCommandSubmission(v, &engine.pools().transfer.transient, engine.device().transfer_queue))?;
		unsafe { vkBeginCommandBuffer(cs.0, &Default::default()) }.make_result(cs)
	}
}
impl<'a> ImmediateGraphicsCommandSubmission<'a>
{
	/// Begin recording commands for immediate submission
	pub fn begin(engine: &'a GraphicsInterface) -> EngineResult<Self>
	{
		let mut v = unsafe { reserved() };
		let cs = unsafe { vkAllocateCommandBuffers(engine.device().native(), &VkCommandBufferAllocateInfo
		{
			commandPool: engine.pools().graphics.transient.native(), level: VK_COMMAND_BUFFER_LEVEL_PRIMARY, commandBufferCount: 1,
			.. Default::default()
		}, &mut v) }.make_result_with(|| ImmediateGraphicsCommandSubmission(v, &engine.pools().graphics.transient, engine.device().graphics_queue))?;
		unsafe { vkBeginCommandBuffer(cs.0, &Default::default()) }.make_result(cs)
	}
}
impl<'a> Drop for ImmediateTransferCommandSubmission<'a>
{
	/// Close recording state and execute
	fn drop(&mut self)
	{
		unsafe { vkEndCommandBuffer(self.0) }.into_result().or_crash();
		unsafe { vkQueueSubmit(self.2, 1, &VkSubmitInfo
		{
			commandBufferCount: 1, pCommandBuffers: &self.0, .. Default::default()
		}, zeroed()) }.into_result().or_crash();
		unsafe { vkQueueWaitIdle(self.2) }.into_result().or_crash();
		unsafe { vkFreeCommandBuffers((self.1).1.native(), self.1.native(), 1, &self.0) };
	}
}
impl<'a> Drop for ImmediateGraphicsCommandSubmission<'a>
{
	/// Close recording state and execute
	fn drop(&mut self)
	{
		unsafe { vkEndCommandBuffer(self.0) }.into_result().or_crash();
		unsafe { vkQueueSubmit(self.2, 1, &VkSubmitInfo
		{
			commandBufferCount: 1, pCommandBuffers: &self.0, .. Default::default()
		}, zeroed()) }.into_result().or_crash();
		unsafe { vkQueueWaitIdle(self.2) }.into_result().or_crash();
		unsafe { vkFreeCommandBuffers((self.1).1.native(), self.1.native(), 1, &self.0) };
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
/// Record an command to its owner buffer
pub trait CommandRecorder
{
	fn buffer(&self) -> VkCommandBuffer;
}
impl<'a> CommandRecorder for GraphicsCommandRecorder<'a> { fn buffer(&self) -> VkCommandBuffer { *self.0 } }
impl<'a> CommandRecorder for BundleCommandRecorder<'a> { fn buffer(&self) -> VkCommandBuffer { *self.0 } }
impl<'a> CommandRecorder for TransferCommandRecorder<'a> { fn buffer(&self) -> VkCommandBuffer { *self.0 } }
impl<'a> CommandRecorder for ImmediateGraphicsCommandSubmission<'a> { fn buffer(&self) -> VkCommandBuffer { self.0 } }
impl<'a> CommandRecorder for ImmediateTransferCommandSubmission<'a> { fn buffer(&self) -> VkCommandBuffer { self.0 } }
/// Provides how to record some drawing commands
pub trait DrawingCommandRecorder: CommandRecorder + Sized
{
	fn bind_pipeline(self, pipeline: &GraphicsPipeline) -> Self
	{
		unsafe { vkCmdBindPipeline(self.buffer(), VK_PIPELINE_BIND_POINT_GRAPHICS, pipeline.native()) }; self
	}
	fn bind_descriptor_sets(self, layout: &PipelineLayout, sets: &DescriptorSetArrayView) -> Self { self.bind_descriptor_sets_partial(layout, 0, sets) }
	fn bind_descriptor_sets_partial(self, layout: &PipelineLayout, start_set: u32, sets: &DescriptorSetArrayView) -> Self
	{
		unsafe { vkCmdBindDescriptorSets(self.buffer(), VK_PIPELINE_BIND_POINT_GRAPHICS, layout.native(), start_set, sets.len() as _, sets.as_ptr(), 0, null()) };
		self
	}
	fn push_constants<T>(self, layout: &PipelineLayout, shader_stage: ShaderStageSet, range: Range<u32>, data: &T) -> Self
	{
		unsafe { vkCmdPushConstants(self.buffer(), layout.native(), shader_stage.into(), range.start, range.len() as _, transmute(data)) };
		self
	}
	fn bind_vertex_buffers(self, buffer_offsets: &[(&BufferResource, usize)]) -> Self
	{
		self.bind_vertex_buffers_partial(0, buffer_offsets)
	}
	fn bind_vertex_buffers_partial(self, start_binding: u32, buffer_offsets: &[(&BufferResource, usize)]) -> Self
	{
		let (buffer_native, offsets_native): (Vec<_>, Vec<_>) = buffer_offsets.into_iter()
			.map(|&(b, v)| (unsafe { transmute::<_, VkBuffer>(b.internal()) }, v as VkDeviceSize)).unzip();
		unsafe { vkCmdBindVertexBuffers(self.buffer(), start_binding, buffer_native.len() as _, buffer_native.as_ptr(), offsets_native.as_ptr()) };
		self
	}
	fn bind_index_buffer(self, buffer: &BufferResource, offset: usize) -> Self
	{
		unsafe { vkCmdBindIndexBuffer(self.buffer(), transmute(buffer.internal()), offset as _, VK_INDEX_TYPE_UINT16) };
		self
	}
	
	fn draw(self, vertex_count: u32, instance_count: u32) -> Self
	{
		unsafe { vkCmdDraw(self.buffer(), vertex_count, instance_count, 0, 0) };
		self
	}
	fn draw_with_voffs(self, vertex_count: u32, vertex_offset: u32, instance_count: u32) -> Self
	{
		unsafe { vkCmdDraw(self.buffer(), vertex_count, instance_count, vertex_offset, 0) };
		self
	}
	fn draw_indexed(self, index_count: u32, instance_count: u32, index_offset: i32) -> Self
	{
		unsafe { vkCmdDrawIndexed(self.buffer(), index_count, instance_count, 0, index_offset, 0) };
		self
	}
	fn draw_indirect(self, param_buffer: &BufferResource, param_offs: usize) -> Self
	{
		unsafe { vkCmdDrawIndirect(self.buffer(), transmute(param_buffer.internal()), param_offs as _, 1, 0) };
		self
	}
	fn draw_indirect_mult(self, param_buffer: &BufferResource, param_offs: usize, param_count: u32) -> Self
	{
		unsafe { vkCmdDrawIndirect(self.buffer(), transmute(param_buffer.internal()), param_offs as _, param_count, size_of::<VkDrawIndirectCommand>() as _) };
		self
	}
}
pub trait QueueSyncOperationCommandRecorder : CommandRecorder + Sized
{
	fn pipeline_barrier<PSs: Into<VkPipelineStageFlags>, PSd: Into<VkPipelineStageFlags>>(self, src_stage_mask: PSs, dst_stage_mask: PSd,
		depend_by_region: bool, memory_barriers: &[MemoryBarrier], buffer_barriers: &[BufferMemoryBarrier], image_barriers: &[ImageMemoryBarrier]) -> Self
	{
		let (mbs_native, bbs_native, ibs_native) = (
			memory_barriers.into_iter().map(|x| x.into()).collect::<Vec<_>>(),
			buffer_barriers.into_iter().map(|x| x.into()).collect::<Vec<_>>(),
			image_barriers.into_iter().map(|x| x.into()).collect::<Vec<_>>()
		);
		unsafe { vkCmdPipelineBarrier(self.buffer(), src_stage_mask.into(), dst_stage_mask.into(),
			if depend_by_region { VK_DEPENDENCY_BY_REGION_BIT } else { 0 },
			mbs_native.len() as u32, mbs_native.as_ptr(),
			bbs_native.len() as u32, bbs_native.as_ptr(),
			ibs_native.len() as u32, ibs_native.as_ptr()) };
		self
	}
	fn pipeline_barrier_on<PS: Into<VkPipelineStageFlags> + Copy>(self, stage_mask: PS, depend_by_region: bool,
		memory_barriers: &[MemoryBarrier], buffer_barriers: &[BufferMemoryBarrier], image_barriers: &[ImageMemoryBarrier]) -> Self
	{
		self.pipeline_barrier(stage_mask, stage_mask, depend_by_region, memory_barriers, buffer_barriers, image_barriers)
	}
}
pub trait CommandInjection : Sized
{
	fn inject_commands<F>(self, f: F) -> Self where F: FnOnce(Self) -> Self { f(self) }
}
pub trait PrimaryGraphicsCommandRecorder: DrawingCommandRecorder + Sized
{
	fn begin_render_pass(self, framebuffer: &Framebuffer, clear_values: &[AttachmentClearValue], use_bundles: bool) -> Self
	{
		let clear_values_native = clear_values.into_iter().map(|x| x.into()).collect::<Vec<_>>();
		let begin_info = VkRenderPassBeginInfo
		{
			renderPass: framebuffer.renderpass().native(), framebuffer: framebuffer.native(),
			renderArea: VkRect2D { extent: framebuffer.area().clone(), .. Default::default() },
			clearValueCount: clear_values_native.len() as _, pClearValues: clear_values_native.as_ptr(), .. Default::default()
		};
		unsafe { vkCmdBeginRenderPass(self.buffer(), &begin_info,
			if use_bundles { VK_SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS } else { VK_SUBPASS_CONTENTS_INLINE }) };
		self
	}
	fn next_subpass(self, use_bundles: bool) -> Self
	{
		unsafe { vkCmdNextSubpass(self.buffer(), if use_bundles { VK_SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS } else { VK_SUBPASS_CONTENTS_INLINE }) };
		self
	}
	fn end_render_pass(self) -> Self { unsafe { vkCmdEndRenderPass(self.buffer()) }; self }

	fn execute_commands(self, buffers: &BundledCommandBuffersView) -> Self
	{
		unsafe { vkCmdExecuteCommands(self.buffer(), buffers.len() as _, buffers.as_ptr()) };
		self
	}
	
	fn blit_image(self, src: &ImageResource, dst: &ImageResource, src_layout: VkImageLayout, dst_layout: VkImageLayout,
		regions: &[ImageBlitRegion], filter: Filter) -> Self
	{
		let regions_native = regions.into_iter().map(Into::into).collect::<Vec<_>>();
		unsafe { vkCmdBlitImage(self.buffer(), transmute(src.internal()), src_layout, transmute(dst.internal()), dst_layout,
			regions_native.len() as _, regions_native.as_ptr(), filter as _) };
		self
	}
}
pub trait PrimaryTransferCommandRecorder : CommandRecorder + Sized
{
	fn copy_buffer(self, src: &BufferResource, dst: &BufferResource, regions: &[BufferCopyRegion]) -> Self
	{
		let regions_native = regions.into_iter().map(Into::into).collect::<Vec<_>>();
		unsafe { vkCmdCopyBuffer(self.buffer(), transmute(src.internal()), transmute(dst.internal()), regions_native.len() as _, regions_native.as_ptr()) };
		self
	}
	fn copy_image(self, src: &ImageResource, dst: &ImageResource, regions: &[ImageCopyRegion]) -> Self
	{
		let regions_native = regions.into_iter().map(Into::into).collect::<Vec<_>>();
		unsafe { vkCmdCopyImage(self.buffer(), transmute(src.internal()), VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
			transmute(dst.internal()), VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL, regions_native.len() as _, regions_native.as_ptr()) };
		self
	}
}
pub trait ClosableCommandRecorder : CommandRecorder + Sized
{
	fn end(self) -> EngineResult<()> { unsafe { vkEndCommandBuffer(self.buffer()) }.into_result()?; forget(self); Ok(()) }
}
pub trait ImmediateSubmissionCommands<'a> : CommandRecorder + Sized
{
	fn submit_opt(self, wait_info: &[(&QueueFence, &PipelineStageFlag)], sync_qf: Option<&QueueFence>, sync_fence: Option<&Fence>)
		-> EngineResult<CommandBufferRef<'a>>;
}

impl<'a> DrawingCommandRecorder for GraphicsCommandRecorder<'a> {}
impl<'a> PrimaryGraphicsCommandRecorder for GraphicsCommandRecorder<'a> {}
impl<'a> QueueSyncOperationCommandRecorder for GraphicsCommandRecorder<'a> {}
impl<'a> CommandInjection for GraphicsCommandRecorder<'a> {}
impl<'a> ClosableCommandRecorder for GraphicsCommandRecorder<'a> {}

impl<'a> DrawingCommandRecorder for ImmediateGraphicsCommandSubmission<'a> {}
impl<'a> PrimaryGraphicsCommandRecorder for ImmediateGraphicsCommandSubmission<'a> {}
impl<'a> QueueSyncOperationCommandRecorder for ImmediateGraphicsCommandSubmission<'a> {}
impl<'a> CommandInjection for ImmediateGraphicsCommandSubmission<'a> {}

impl<'a> DrawingCommandRecorder for BundleCommandRecorder<'a> {}
impl<'a> CommandInjection for BundleCommandRecorder<'a> {}
impl<'a> ClosableCommandRecorder for BundleCommandRecorder<'a> {}

impl<'a> PrimaryTransferCommandRecorder for TransferCommandRecorder<'a> {}
impl<'a> QueueSyncOperationCommandRecorder for TransferCommandRecorder<'a> {}
impl<'a> CommandInjection for TransferCommandRecorder<'a> {}
impl<'a> ClosableCommandRecorder for TransferCommandRecorder<'a> {}

impl<'a> PrimaryTransferCommandRecorder for ImmediateTransferCommandSubmission<'a> {}
impl<'a> QueueSyncOperationCommandRecorder for ImmediateTransferCommandSubmission<'a> {}
impl<'a> CommandInjection for ImmediateTransferCommandSubmission<'a> {}

/// Garbage Command Buffer(only reference)
pub struct CommandBufferRef<'a>(VkCommandBuffer, &'a NativeCommandPool);
impl<'a> ImmediateSubmissionCommands<'a> for ImmediateTransferCommandSubmission<'a>
{
	fn submit_opt(self, wait_info: &[(&QueueFence, &PipelineStageFlag)], sync_qf: Option<&QueueFence>, sync_fence: Option<&Fence>)
		-> EngineResult<CommandBufferRef<'a>>
	{
		unsafe { vkEndCommandBuffer(self.0) }.into_result()?;

		let (wsem, stage): (Vec<_>, Vec<_>) = wait_info.into_iter().map(|&(x, w)| (x.native(), w.into_flag())).unzip();
		let ssem = sync_qf.into_iter().map(NativeHandleProvider::native).collect::<Vec<_>>();
		unsafe { vkQueueSubmit(self.2, 1, &VkSubmitInfo
		{
			waitSemaphoreCount: wsem.len() as _, pWaitSemaphores: wsem.as_ptr(), pWaitDstStageMask: stage.as_ptr(),
			signalSemaphoreCount: ssem.len() as _, pSignalSemaphores: ssem.as_ptr(), commandBufferCount: 1, pCommandBuffers: &self.0, .. Default::default()
		}, sync_fence.map(NativeHandleProvider::native).unwrap_or(zeroed())) }.into_result()?;
		if sync_qf.is_none() && sync_fence.is_none() { unsafe { vkQueueWaitIdle(self.2) }.into_result()?; }
		let ImmediateTransferCommandSubmission(buf, cp, _) = self; forget(self);
		Ok(CommandBufferRef(buf, cp))
	}
}
impl<'a> ImmediateSubmissionCommands<'a> for ImmediateGraphicsCommandSubmission<'a>
{
	fn submit_opt(self, wait_info: &[(&QueueFence, &PipelineStageFlag)], sync_qf: Option<&QueueFence>, sync_fence: Option<&Fence>)
		-> EngineResult<CommandBufferRef<'a>>
	{
		unsafe { vkEndCommandBuffer(self.0) }.into_result()?;
		let (wsem, stage): (Vec<_>, Vec<_>) = wait_info.into_iter().map(|&(x, w)| (x.native(), w.into_flag())).unzip();
		let ssem = sync_qf.into_iter().map(NativeHandleProvider::native).collect::<Vec<_>>();
		unsafe { vkQueueSubmit(self.2, 1, &VkSubmitInfo
		{
			waitSemaphoreCount: wsem.len() as _, pWaitSemaphores: wsem.as_ptr(), pWaitDstStageMask: stage.as_ptr(),
			signalSemaphoreCount: ssem.len() as _, pSignalSemaphores: ssem.as_ptr(), commandBufferCount: 1, pCommandBuffers: &self.0, .. Default::default()
		}, sync_fence.map(NativeHandleProvider::native).unwrap_or(zeroed())) }.into_result()?;
		if sync_qf.is_none() && sync_fence.is_none() { unsafe { vkQueueWaitIdle(self.2) }.into_result()?; }
		let ImmediateGraphicsCommandSubmission(buf, cp, _) = self; forget(self);
		Ok(CommandBufferRef(buf, cp))
	}
}
impl<'a> Drop for CommandBufferRef<'a>
{
	fn drop(&mut self) { unsafe { vkFreeCommandBuffers((self.1).1.native(), self.1.native(), 1, &self.0) }; }
}

#[derive(Clone)]
pub struct BufferCopyRegion(pub usize, pub usize, pub usize);		// src, dst, size
impl<'a> Into<VkBufferCopy> for &'a BufferCopyRegion
{
	fn into(self) -> VkBufferCopy
	{
		let &BufferCopyRegion(src, dst, size) = self;
		VkBufferCopy { srcOffset: src as _, dstOffset: dst as _, size: size as _ }
	}
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImageCopyRegion
{
	pub src_layers: ImageSubresourceLayers, pub src_offset: VkOffset3D,
	pub dst_layers: ImageSubresourceLayers, pub dst_offset: VkOffset3D,
	pub extent: VkExtent3D
}
impl Default for ImageCopyRegion
{
	/// Entire Bits
	fn default() -> Self
	{
		ImageCopyRegion
		{
			src_layers: Default::default(), src_offset: Default::default(),
			dst_layers: Default::default(), dst_offset: Default::default(), extent: Default::default()
		}
	}
}
impl<'a> Into<VkImageCopy> for &'a ImageCopyRegion
{
	fn into(self) -> VkImageCopy
	{
		VkImageCopy
		{
			srcSubresource: (&self.src_layers).into(), srcOffset: self.src_offset.clone(), 
			dstSubresource: (&self.dst_layers).into(), dstOffset: self.dst_offset.clone(), extent: self.extent.clone()
		}
	}
}
#[derive(Debug, PartialEq, Eq)]
pub struct ImageBlitRegion
{
	pub src_layers: ImageSubresourceLayers, pub dst_layers: ImageSubresourceLayers,
	pub src_offsets: [VkOffset3D; 2], pub dst_offsets: [VkOffset3D; 2]
}
impl<'a> Into<VkImageBlit> for &'a ImageBlitRegion
{
	fn into(self) -> VkImageBlit
	{
		VkImageBlit
		{
			srcSubresource: (&self.src_layers).into(), dstSubresource: (&self.dst_layers).into(),
			srcOffsets: [self.src_offsets[0].clone(), self.src_offsets[1].clone()],
			dstOffsets: [self.dst_offsets[0].clone(), self.dst_offsets[1].clone()]
		}
	}
}
impl ImageBlitRegion
{
	/// Blit Same Region(preferred to use Copy)
	pub fn same_region(src_subres: ImageSubresourceLayers, dst_subres: ImageSubresourceLayers, offs: &Offset3, size: &Size3) -> Self
	{
		let offs2 = VkOffset3D { x: offs.0 + size.0 as i32, y: offs.1 + size.1 as i32, z: offs.2 + size.2 as i32 };
		ImageBlitRegion
		{
			src_layers: src_subres, src_offsets: [offs.as_ref().clone(), offs2.clone()],
			dst_layers: dst_subres, dst_offsets: [offs.as_ref().clone(), offs2]
		}
	}
}
impl Clone for ImageBlitRegion
{
	fn clone(&self) -> Self
	{
		ImageBlitRegion
		{
			src_layers: self.src_layers.clone(), dst_layers: self.dst_layers.clone(),
			src_offsets: [self.src_offsets[0].clone(), self.src_offsets[1].clone()],
			dst_offsets: [self.dst_offsets[0].clone(), self.dst_offsets[1].clone()]
		}
	}
}

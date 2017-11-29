// Interlude: Synchronize Primitives(Fence and QueueFence(Semaphore))

use {GraphicsInterface, EngineResult};
use interlude_vk_defs::*;
use interlude_vk_funport::*;
use device::Device;
use subsystem_layer::{NativeHandleProvider, NativeResultValueHandler};
use std::u64;
use std::ptr::null;
use std::rc::Rc;
use std::mem::uninitialized as reserved;

/// Semaphore: Synchronize primitive between queues
pub struct QueueFence(VkSemaphore, Rc<Device>);
/// Fence: Synchronize primitive between host and device
pub struct Fence(VkFence, Rc<Device>);

impl Fence
{
	pub fn new(engine: &GraphicsInterface) -> EngineResult<Self>
	{
		let mut f = unsafe { reserved() };
		unsafe { vkCreateFence(engine.device().native(), &Default::default(), null(), &mut f) }
			.make_result_with(|| Fence(f, engine.device().clone()))
	}
	pub fn clear(&self) -> EngineResult<()> { unsafe { vkResetFences(self.1.native(), 1, &self.0) }.into_result() }
	pub fn wait(&self, timeout: Option<u64>) -> EngineResult<()>
	{
		unsafe { vkWaitForFences(self.1.native(), 1, &self.0, false as _, timeout.unwrap_or(u64::MAX)) }.into_result()
	}
}
impl QueueFence
{
	pub fn new(engine: &GraphicsInterface) -> EngineResult<Self>
	{
		let mut sem = unsafe { reserved() };
		unsafe { vkCreateSemaphore(engine.device().native(), &Default::default(), null(), &mut sem) }
			.make_result_with(|| QueueFence(sem, engine.device().clone()))
	}
}
impl Drop for Fence { fn drop(&mut self) { unsafe { vkDestroyFence(self.1.native(), self.0, null()) }; } }
impl Drop for QueueFence { fn drop(&mut self) { unsafe { vkDestroySemaphore(self.1.native(), self.0, null()) }; } }
unsafe impl Send for Fence {}
unsafe impl Send for QueueFence {}
impl NativeHandleProvider for Fence
{
	type NativeT = VkFence;
	fn native(&self) -> VkFence { self.0 }
}
impl NativeHandleProvider for QueueFence
{
	type NativeT = VkSemaphore;
	fn native(&self) -> VkSemaphore { self.0 }
}

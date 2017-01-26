// Prelude: Synchronize Primitives(Fence and QueueFence(Semaphore))

use ginterface::GraphicsInterface;
use EngineError;
use vk;
use vk::ffi::*;

pub struct QueueFence(vk::Semaphore);
pub struct Fence(vk::Fence);

impl Fence
{
	pub fn new(engine: &GraphicsInterface) -> Result<Self, EngineError>
	{
		vk::Fence::new(engine.device()).map(Fence).map_err(From::from)
	}
	pub fn clear(&self) -> Result<(), EngineError>
	{
		self.0.reset().map_err(EngineError::from)
	}
	pub fn wait(&self) -> Result<(), EngineError>
	{
		self.0.wait().map_err(EngineError::from)
	}
}
impl QueueFence
{
	pub fn new(engine: &GraphicsInterface) -> Result<Self, EngineError>
	{
		vk::Semaphore::new(engine.device()).map(QueueFence).map_err(From::from)
	}
}

unsafe impl Send for Fence {}
unsafe impl Send for QueueFence {}
pub fn fence_raw(f: &Fence) -> VkFence { *f.0 }
pub fn qfence_raw(f: &QueueFence) -> VkSemaphore { *f.0 }

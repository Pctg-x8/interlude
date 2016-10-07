// Prelude: Synchronize Primitives(Fence and QueueFence(Semaphore))

use super::internals::*;
use {vk, std};

pub trait QueueFenceInternals
{
	fn new(sem: vk::Semaphore) -> Self;
}
pub trait FenceInternals
{
	fn new(fen: vk::Fence) -> Self;
}

pub struct QueueFence { internal: vk::Semaphore }
pub struct Fence { internal: vk::Fence }
pub struct FenceRef<'a> { internal: &'a Fence }

impl InternalExports<vk::Semaphore> for QueueFence { fn get_internal(&self) -> &vk::Semaphore { &self.internal } }
impl InternalExports<vk::Fence> for Fence { fn get_internal(&self) -> &vk::Fence { &self.internal } }

unsafe impl<'a> Send for Fence {}
unsafe impl<'a> Send for QueueFence {}
unsafe impl<'a> Send for FenceRef<'a> {}

impl QueueFenceInternals for QueueFence
{
	fn new(sem: vk::Semaphore) -> Self { QueueFence { internal: sem } }
}
impl FenceInternals for Fence
{
	fn new(fen: vk::Fence) -> Self { Fence { internal: fen } }
}
impl<'a> std::ops::Deref for FenceRef<'a>
{
	type Target = Fence;
	fn deref(&self) -> &Fence { self.internal }
}

impl Fence
{
	pub fn get_status(&self) -> Result<(), EngineError>
	{
		self.internal.get_status().map_err(EngineError::from)
	}
	pub fn clear(&self) -> Result<(), EngineError>
	{
		self.internal.reset().map_err(EngineError::from)
	}
	pub fn wait(&self) -> Result<(), EngineError>
	{
		self.internal.wait().map_err(EngineError::from)
	}
	pub fn clone_ref<'a>(&'a self) -> FenceRef<'a>
	{
		FenceRef { internal: self }
	}
}

//! Graphics Subsystem Layer

use interlude_vk_defs::*;
use interlude_vk_funport::*;
use std::ptr::{null, null_mut};
use std::mem::transmute;
use {EngineResult, EngineError};
use device::Device;
use std::rc::Rc;

/// VkResult Handler
pub trait NativeResultValueHandler: Sized
{
	fn make_result<T>(self, v: T) -> EngineResult<T> { self.make_result_with(|| v) }
	fn make_result_with<F: FnOnce() -> T, T>(self, f: F) -> EngineResult<T>;
	fn into_result(self) -> EngineResult<()>;
}
impl NativeResultValueHandler for VkResult
{
	fn make_result_with<F: FnOnce() -> T, T>(self, f: F) -> EngineResult<T>
	{
		if self == VK_SUCCESS { Ok(f()) } else { Err(EngineError::DeviceError(self)) }
	}
	fn into_result(self) -> EngineResult<()> { if self == VK_SUCCESS { Ok(()) } else { Err(EngineError::DeviceError(self)) } }
}
/// Native Handle Provider
pub trait NativeHandleProvider
{
	type NativeT;
	fn native(&self) -> Self::NativeT;
}

/// VkInstance
pub struct NativeInstance(VkInstance);
impl NativeInstance
{
	/// Build
	pub fn new(cinfo: &VkInstanceCreateInfo) -> EngineResult<Self>
	{
		let mut instance = null_mut();
		unsafe { vkCreateInstance(cinfo, null(), &mut instance) }.make_result_with(|| NativeInstance(instance))
	}
}
impl Drop for NativeInstance
{
	fn drop(&mut self) { unsafe { vkDestroyInstance(self.0, null()) } }
}
impl AsRef<VkInstance> for NativeInstance
{
	fn as_ref(&self) -> &VkInstance { unsafe { transmute(self.0) } }
}
impl NativeHandleProvider for NativeInstance
{
	type NativeT = VkInstance;
	fn native(&self) -> VkInstance { self.0 }
}

/// Render Pass
pub struct NativeRenderPass(pub VkRenderPass, pub Rc<Device>);
impl Drop for NativeRenderPass { fn drop(&mut self) { unsafe { vkDestroyRenderPass(self.1.native(), self.0, null()) }; } }
impl NativeHandleProvider for NativeRenderPass { type NativeT = VkRenderPass; fn native(&self) -> VkRenderPass { self.0 } }
/// Framebuffer
pub struct NativeFramebuffer(pub VkFramebuffer, pub Rc<Device>);
impl Drop for NativeFramebuffer { fn drop(&mut self) { unsafe { vkDestroyFramebuffer(self.1.native(), self.0, null()) }; } }
impl NativeHandleProvider for NativeFramebuffer { type NativeT = VkFramebuffer; fn native(&self) -> VkFramebuffer { self.0 } }

/// Image Resource
pub struct NativeImage<SizeT>(pub VkImage, pub Rc<Device>, pub SizeT);
impl<SizeT> Drop for NativeImage<SizeT> { fn drop(&mut self) { unsafe { vkDestroyImage(self.1.native(), self.0, null()) }; } }
impl<SizeT> NativeHandleProvider for NativeImage<SizeT> { type NativeT = VkImage; fn native(&self) -> VkImage { self.0 } }

/// DescriptorSet Layout
pub struct NativeDescriptorSetLayout(pub VkDescriptorSetLayout, pub Rc<Device>);
impl Drop for NativeDescriptorSetLayout { fn drop(&mut self) { unsafe { vkDestroyDescriptorSetLayout(self.1.native(), self.0, null()) }; } }
impl NativeHandleProvider for NativeDescriptorSetLayout { type NativeT = VkDescriptorSetLayout; fn native(&self) -> VkDescriptorSetLayout { self.0 } }
/// Pipeline Layout
pub struct NativePipelineLayout(pub VkPipelineLayout, pub Rc<Device>);
impl Drop for NativePipelineLayout { fn drop(&mut self) { unsafe { vkDestroyPipelineLayout(self.1.native(), self.0, null()) }; } }
impl NativeHandleProvider for NativePipelineLayout { type NativeT = VkPipelineLayout; fn native(&self) -> VkPipelineLayout { self.0 } }

/// Command Pool
pub struct NativeCommandPool(pub VkCommandPool, pub Rc<Device>);
impl Drop for NativeCommandPool { fn drop(&mut self) { unsafe { vkDestroyCommandPool(self.1.native(), self.0, null()) }; } }
impl NativeHandleProvider for NativeCommandPool { type NativeT = VkCommandPool; fn native(&self) -> VkCommandPool { self.0 } }

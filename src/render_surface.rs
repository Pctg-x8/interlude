// Interlude: Window and RenderWindow(Traits Provider)

use std::rc::Rc;
use {EngineResult, EngineError, GraphicsInterface, QueueFence, ImageView, ImageResource};
use interlude_vk_defs::*;
use interlude_vk_funport::*;
use data::Size2;
use std::ops::Deref;
use std::mem::uninitialized as reserved;
use std::ptr::{null, null_mut};
use std::cmp::max;
use std::{u32, u64};
use subsystem_layer::{NativeInstance, NativeHandleProvider, NativeResultValueHandler};
use device::Device;

// Platform dependent selection
#[cfg(windows)] pub use win32::NativeWindow as PlatformWindowType;
#[cfg(feature = "target_xlib")] pub use wsi::target_xlib::NativeWindowWithServer as PlatformWindowType;

pub fn make_render_window(g: &GraphicsInterface, size: &Size2, caption: &str, resizable: bool) -> EngineResult<RenderWindow>
{
	let under = PlatformWindowType::new(size, caption, resizable)?;
	if under.can_vk_present(g.device().adapter(), g.device().graphics_qf_index) { RenderWindow::new(under, g, size) }
	else { Err(EngineError::GenericError("Vulkan Presentation is not supported by platform.")) }
}

pub struct WindowRenderTargetView(VkImage, VkImageView, VkFormat);
impl ImageResource for WindowRenderTargetView
{
	fn internal(&self) -> u64 { self.0 as _ }
}
impl ImageView for WindowRenderTargetView
{
	fn internal(&self) -> u64 { self.1 as _ }
	fn format(&self) -> VkFormat { self.2 }
}
pub struct RenderWindow
{
	underlying: PlatformWindowType, parent: Rc<Device>, apiroot: Rc<NativeInstance>,
	swapchain: VkSwapchainKHR, surface: VkSurfaceKHR, render_targets: Vec<WindowRenderTargetView>,
	format: VkFormat, extent: Size2, #[allow(dead_code)] has_vsync: bool
}
struct SupportedSurface<'a>(VkSurfaceKHR, &'a GraphicsInterface);
impl<'a> SupportedSurface<'a>
{
	fn ensure(s: VkSurfaceKHR, g: &'a GraphicsInterface) -> EngineResult<Self>
	{
		if g.surface_support(s)? { Ok(SupportedSurface(s, g)) }
		else { Err(EngineError::GenericError("Surface is not supported by device")) }
	}
	fn caps(&self) -> EngineResult<VkSurfaceCapabilitiesKHR>
	{
		let mut caps = unsafe { reserved() };
		unsafe { vkGetPhysicalDeviceSurfaceCapabilitiesKHR(self.1.device().adapter(), self.0, &mut caps) }.make_result(caps)
	}
	fn supported_formats(&self) -> EngineResult<Vec<VkSurfaceFormatKHR>>
	{
		let mut format_count = 0;
		unsafe { vkGetPhysicalDeviceSurfaceFormatsKHR(self.1.device().adapter(), self.0, &mut format_count, null_mut()) }.into_result()?;
		let mut formats = vec![unsafe { reserved() }; format_count as usize];
		unsafe { vkGetPhysicalDeviceSurfaceFormatsKHR(self.1.device().adapter(), self.0, &mut format_count, formats.as_mut_ptr()) }
			.make_result(formats)
	}
	fn supported_present_modes(&self) -> EngineResult<Vec<VkPresentModeKHR>>
	{
		let mut mode_count = 0;
		unsafe { vkGetPhysicalDeviceSurfacePresentModesKHR(self.1.device().adapter(), self.0, &mut mode_count, null_mut()) }.into_result()?;
		let mut modes = vec![unsafe { reserved() }; mode_count as usize];
		unsafe { vkGetPhysicalDeviceSurfacePresentModesKHR(self.1.device().adapter(), self.0, &mut mode_count, modes.as_mut_ptr()) }
			.make_result(modes)
	}
}
impl RenderWindow
{
	fn new(under: PlatformWindowType, g: &GraphicsInterface, size: &Size2) -> Result<Self, EngineError>
	{
		let surface = under.make_vk_surface(g.apicontext())?;
		let surface_ref = SupportedSurface::ensure(surface, g)?;
		let surface_caps = surface_ref.caps()?;

		// autodetect parameters //
		let format = surface_ref.supported_formats()?.into_iter().find(|x| x.format == VK_FORMAT_R8G8B8A8_SRGB || x.format == VK_FORMAT_B8G8R8A8_SRGB)
			.ok_or(EngineError::GenericError("Desired Format(32bpp SRGB) is not supported"))?;
		let present_mode =
		{
			let modes = surface_ref.supported_present_modes()?;
			modes.iter().find(|&&x| x == VK_PRESENT_MODE_FIFO_KHR).or_else(|| modes.iter().find(|&&x| x == VK_PRESENT_MODE_MAILBOX_KHR)).map(|&x| x)
				.ok_or(EngineError::GenericError("Desired Present Mode is not supported"))?
		};
		let extent = if surface_caps.currentExtent.width == u32::MAX || surface_caps.currentExtent.height == u32::MAX { size.as_ref() }
		else { &surface_caps.currentExtent };

		// Set information and Create //
		let queue_family_indices = [g.device().graphics_qf_index];
		let mut swapchain = unsafe { reserved() };
		unsafe { vkCreateSwapchainKHR(g.device().native(), &VkSwapchainCreateInfoKHR
		{
			minImageCount: max(surface_caps.minImageCount, 2), imageFormat: format.format, imageColorSpace: format.colorSpace,
			imageExtent: extent.clone(), imageArrayLayers: 1, imageUsage: VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT, compositeAlpha: VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR,
			preTransform: VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR, presentMode: present_mode, clipped: true as VkBool32,
			queueFamilyIndexCount: queue_family_indices.len() as _, pQueueFamilyIndices: queue_family_indices.as_ptr(), surface, .. Default::default()
		}, null(), &mut swapchain) }.into_result()?;
		let mut bi_count = 0;
		unsafe { vkGetSwapchainImagesKHR(g.device().native(), swapchain, &mut bi_count, null_mut()) }.into_result()?;
		let mut back_images = vec![unsafe { reserved() }; bi_count as _];
		unsafe { vkGetSwapchainImagesKHR(g.device().native(), swapchain, &mut bi_count, back_images.as_mut_ptr()) }.into_result()?;
		let back_views = back_images.into_iter().map(|res| unsafe
		{
			let mut iv = reserved();
			vkCreateImageView(g.device().native(), &VkImageViewCreateInfo
			{
				image: res, subresourceRange: VkImageSubresourceRange { aspectMask: VK_IMAGE_ASPECT_COLOR_BIT, layerCount: 1, .. Default::default() },
				format: format.format, viewType: VK_IMAGE_VIEW_TYPE_2D, components: Default::default(), .. Default::default()
			}, null(), &mut iv).make_result_with(|| WindowRenderTargetView(res, iv, format.format))
		}).collect::<EngineResult<Vec<_>>>()?;

		Ok(RenderWindow
		{
			underlying: under, swapchain, surface, parent: g.device().clone(), apiroot: g.apicontext().clone(),
			render_targets: back_views, format: format.format, extent: extent.as_ref().clone(), has_vsync: present_mode == VK_PRESENT_MODE_FIFO_KHR
		})
	}

	// Old RenderWindow implementations //
	pub fn render_targets(&self) -> &[WindowRenderTargetView] { &self.render_targets }
	pub fn format(&self) -> VkFormat { self.format }
	pub fn size(&self) -> &Size2 { &self.extent }
	pub fn acquire_next_target_index(&self, wait_semaphore: &QueueFence) -> EngineResult<u32>
	{
		let mut next_index = 0;
		unsafe { vkAcquireNextImageKHR(self.parent.native(), self.swapchain, u64::MAX, wait_semaphore.native(), 0 as _, &mut next_index) }
			.make_result(next_index)
	}
	pub fn present(&self, engine: &GraphicsInterface, index: u32, wait_semaphore: Option<&QueueFence>) -> EngineResult<()>
	{
		let sem = wait_semaphore.map(NativeHandleProvider::native).into_iter().collect::<Vec<_>>();
		unsafe { vkQueuePresentKHR(engine.device().graphics_queue, &VkPresentInfoKHR
		{
			waitSemaphoreCount: sem.len() as _, pWaitSemaphores: sem.as_ptr(),
			swapchainCount: 1, pSwapchains: &self.swapchain, pImageIndices: &index, .. Default::default()
		}) }.into_result()
	}
}
impl Deref for RenderWindow { type Target = PlatformWindowType; fn deref(&self) -> &PlatformWindowType { &self.underlying } }
unsafe impl Send for RenderWindow {}
unsafe impl Sync for RenderWindow {}
impl Drop for RenderWindow
{
	fn drop(&mut self)
	{
		for WindowRenderTargetView(_, v, _) in self.render_targets.drain(..) { unsafe { vkDestroyImageView(self.parent.native(), v, null()) }; }
		unsafe { vkDestroySwapchainKHR(self.parent.native(), self.swapchain, null()) };
		unsafe { vkDestroySurfaceKHR(self.apiroot.native(), self.surface, null()) };
	}
}

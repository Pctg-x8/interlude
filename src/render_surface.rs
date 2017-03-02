// Interlude: Window and RenderWindow(Traits Provider)

#![allow(dead_code)]

use {std, vk};
use tuple_tools::*;
use vkdefs::*;
use std::rc::Rc;
use {EngineResult, EngineError, GraphicsInterface, QueueFence, Resource, ImageView};
use data::*;
use rawexports::*;
use vk::traits::*;

// Platform dependent selection
#[cfg(windows)] use win32::NativeWindow;
#[cfg(windows)] use win32::Surface;
#[cfg(unix)] use linux::NativeWindowAndServerCon as NativeWindow;
#[cfg(unix)] use linux::Surface;

pub fn make_render_window(under: NativeWindow, g: &GraphicsInterface, size: &Size2) -> EngineResult<RenderWindow>
{
	if under.is_vk_presentation_support(g.device().parent(), g.device().graphics_qf_index)
	{
		RenderWindow::new(under, g, size)
	}
	else { Err(EngineError::GenericError("Vulkan Presentation by Platform is not supported.")) }
}

pub struct WindowRenderTargetView(VkImage, vk::ImageView, VkFormat);
impl Resource for WindowRenderTargetView { type Type = VkImage; fn resource(&self) -> VkImage { self.0 } }
impl ImageView for WindowRenderTargetView
{
	fn get_native(&self) -> VkImageView { *self.1 }
	fn format(&self) -> VkFormat { self.2 }
}
pub struct RenderWindow
{
	underlying: NativeWindow, swapchain: Rc<vk::Swapchain<Surface>>, render_targets: Vec<WindowRenderTargetView>,
	format: VkFormat, extent: Size2, has_vsync: bool
}
struct SupportedSurface<'a>(&'a Surface, &'a GraphicsInterface);
impl<'a> SupportedSurface<'a>
{
	fn ensure(s: &'a Surface, g: &'a GraphicsInterface) -> Result<Self, EngineError>
	{
		g.ensure_surface_support(s).map(|_| SupportedSurface(s, g))
	}
	fn caps(&self) -> VkSurfaceCapabilitiesKHR { self.1.surface_caps(self.0) }
	fn supported_formats(&self) -> Result<Vec<VkSurfaceFormatKHR>, EngineError> { self.1.surface_formats(self.0) }
	fn supported_present_modes(&self) -> Result<Vec<VkPresentModeKHR>, EngineError> { self.1.surface_present_modes(self.0) }
}
impl RenderWindow
{
	fn new(under: NativeWindow, g: &GraphicsInterface, size: &Size2) -> Result<Self, EngineError>
	{
		let surface = try!(under.make_vk_surface(g.apicontext()).map(Rc::new));
		let surface_ref = try!(SupportedSurface::ensure(&surface, g));
		let surface_caps = surface_ref.caps();

		// autodetect parameters //
		let (format, present_mode) = try!((
			surface_ref.supported_formats().map_err(EngineError::from).and_then(|formats|
				formats.into_iter().find(|x| x.format == VkFormat::R8G8B8A8_SRGB || x.format == VkFormat::B8G8R8A8_SRGB)
					.ok_or(EngineError::GenericError("Desired Format(32bpp SRGB) is not supported"))),
			surface_ref.supported_present_modes().map_err(EngineError::from).and_then(|present_modes|
				present_modes.iter().find(|&&x| x == VkPresentModeKHR::FIFO).or_else(||
					present_modes.iter().find(|&&x| x == VkPresentModeKHR::Mailbox)).map(|&x| x)
				.ok_or(EngineError::GenericError("Desired Present Mode is not supported")))
		).flatten());
		let extent = match surface_caps.currentExtent
		{
			VkExtent2D(std::u32::MAX, _) | VkExtent2D(_, std::u32::MAX) => unsafe { std::mem::transmute(size) },
			_ => &surface_caps.currentExtent
		};

		// Set information and Create //
		let sc = try!({
			let queue_family_indices = [g.device().graphics_queue.family_index()];
			vk::Swapchain::new(g.device(), &surface, &VkSwapchainCreateInfoKHR
			{
				sType: VkStructureType::SwapchainCreateInfoKHR, pNext: std::ptr::null(), flags: 0,
				minImageCount: std::cmp::max(surface_caps.minImageCount, 2), imageFormat: format.format, imageColorSpace: format.colorSpace,
				imageExtent: *extent, imageArrayLayers: 1, imageUsage: VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT,
				imageSharingMode: VkSharingMode::Exclusive, compositeAlpha: VK_COMPOSITE_ALPHA_OPAQUE_BIT, preTransform: VK_SURFACE_TRANSFORM_IDENTITY_BIT,
				presentMode: present_mode, clipped: true as VkBool32,
				pQueueFamilyIndices: queue_family_indices.as_ptr(), queueFamilyIndexCount: queue_family_indices.len() as u32,
				oldSwapchain: vk::empty_handle(), surface: **surface
			}).map(Rc::new)
		});
		let backbuffer_renderviews = try!(sc.images().and_then(|backbuffers| backbuffers.into_iter().map(|res|
		{
			vk::ImageView::new(g.device(), &VkImageViewCreateInfo
			{
				sType: VkStructureType::ImageViewCreateInfo, pNext: std::ptr::null(), flags: 0,
				image: res, subresourceRange: vk::ImageSubresourceRange::default_color(),
				format: format.format, viewType: VkImageViewType::Dim2, components: VkComponentMapping::default()
			}).map(|v| WindowRenderTargetView(res, v, format.format))
		}).collect::<Result<Vec<_>, _>>()));

		Ok(RenderWindow
		{
			underlying: under, swapchain: sc, render_targets: backbuffer_renderviews,
			format: format.format, extent: unsafe { std::mem::transmute(*extent) }, has_vsync: present_mode == VkPresentModeKHR::FIFO
		})
	}

	// Old RenderWindow implementations //
	pub fn render_targets(&self) -> &[WindowRenderTargetView] { &self.render_targets }
	pub fn format(&self) -> VkFormat { self.format }
	pub fn size(&self) -> &Size2 { &self.extent }
	pub fn acquire_next_target_index(&self, wait_semaphore: &QueueFence) -> EngineResult<u32>
	{
		self.swapchain.acquire_next(qfence_raw(wait_semaphore)).map_err(EngineError::from)
	}
	pub fn present(&self, engine: &GraphicsInterface, index: u32, wait_semaphore: Option<&QueueFence>) -> EngineResult<()>
	{
		let sem = wait_semaphore.map(qfence_raw);
		self.swapchain.present(&engine.device().graphics_queue, index, &if let Some(s) = sem { vec![s] } else { Vec::new() }).map_err(EngineError::from)
	}
}
impl std::ops::Deref for RenderWindow { type Target = NativeWindow; fn deref(&self) -> &NativeWindow { &self.underlying } }
unsafe impl Send for RenderWindow {}
unsafe impl Sync for RenderWindow {}

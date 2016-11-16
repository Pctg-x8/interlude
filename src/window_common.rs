// Interlude: Window and RenderWindow(Traits Provider)

#![allow(dead_code)]

use super::internals::*;
use {std, vk};
use std::rc::Rc;
use std::sync::Arc;
use vk::ffi::*;
use vk::traits::*;

/// Application State(has exited?)
#[derive(PartialEq)]
pub enum ApplicationState { Continue, Exited }

/// Indicates Native Window
pub trait NativeWindow : std::marker::Sized + 'static
{
	type NativeWindowServerT : WindowServer<NativeWindowT = Self>;
	type SurfaceCreateInfoKHR;

	fn native_show(&self, server: &Self::NativeWindowServerT);
	fn native_surface_create_info(&self, server: &Self::NativeWindowServerT) -> Self::SurfaceCreateInfoKHR;
}
/// Indicates that provides window and processes messages
pub trait WindowServer: std::marker::Sync + std::marker::Send + std::marker::Sized
{
	type NativeWindowT : NativeWindow<NativeWindowServerT = Self> + 'static;

	fn create_unresizable_window(&self, size: VkExtent2D, title: &str) -> Result<Self::NativeWindowT, EngineError>;
	fn show_window(&self, target: &Self::NativeWindowT);
	fn flush(&self);
	fn process_events(&self) -> ApplicationState;
	fn process_all_events(&self);
	fn is_vk_presentation_support(&self, adapter: &vk::PhysicalDevice, qf_index: u32) -> bool;
	fn make_vk_surface(&self, target: &Self::NativeWindowT, instance: &Rc<vk::Instance>) -> Result<vk::Surface, EngineError>;
}
/*
pub struct WaylandServer
{

}
impl WindowServer for WaylandServer
{
	fn connect() -> Result<Self, EngineError>
	{
		Ok(WaylandServer {})
	}
}
*/

pub trait RenderWindow: std::marker::Send
{
	fn get_back_images(&self) -> Vec<&WindowRenderTarget>;
	fn get_format(&self) -> VkFormat;
	fn get_extent(&self) -> VkExtent2D;
	fn acquire_next_backbuffer_index(&self, wait_semaphore: &QueueFence) -> Result<u32, EngineError>;
	fn present(&self, gqueue: &vk::Queue, index: u32) -> Result<(), EngineError>;
}
pub struct WindowRenderTarget { pub resource: VkImage, pub view: vk::ImageView }
impl ImageResource for WindowRenderTarget { fn get_resource(&self) -> VkImage { self.resource } }
impl ImageView for WindowRenderTarget { fn get_native(&self) -> VkImageView { self.view.get() } }
pub struct Window<N: NativeWindow>
{
	#[allow(dead_code)] server: Arc<N::NativeWindowServerT>, #[allow(dead_code)] native: N,
	#[allow(dead_code)] device_obj: Rc<vk::Surface>, swapchain: Rc<vk::Swapchain>, render_targets: Vec<WindowRenderTarget>,
	format: VkFormat, extent: VkExtent2D, has_vsync: bool,
	backbuffer_available_signal: QueueFence, transfer_complete_signal: QueueFence
}
unsafe impl<N: NativeWindow> Send for Window<N> {}
impl<N: NativeWindow> Window<N>
{
	pub fn create_unresizable(engine: &Engine<N::NativeWindowServerT>, size: VkExtent2D, title: &str) -> Result<Box<Self>, EngineError>
	{
		let server = engine.get_window_server();
		let native_w = try!(server.create_unresizable_window(size, title));
		server.show_window(&native_w);
		server.flush();
		let surface = Rc::new(try!(server.make_vk_surface(&native_w, engine.get_instance())));
		let adapter = engine.get_device().get_adapter();

		// caps check //
		if !engine.get_device().is_surface_support(&surface) { return Err(EngineError::GenericError("Unsupported Surface")); }
		let surface_caps = adapter.get_surface_caps(&surface);

		// making desired parameters //
		let format = try!
		{
			adapter.enumerate_surface_formats(&surface).into_iter()
				.find(|x| x.format == VkFormat::R8G8B8A8_SRGB || x.format == VkFormat::B8G8R8A8_SRGB)
				.ok_or(EngineError::GenericError("Desired Format(32bpp SRGB) is not supported"))
		};
		let present_modes = adapter.enumerate_present_modes(&surface);
		let present_mode = try!
		{
			present_modes.iter().find(|&&x| x == VkPresentModeKHR::FIFO)
				.or_else(|| present_modes.iter().find(|&&x| x == VkPresentModeKHR::Mailbox))
				.ok_or(EngineError::GenericError("Desired Present Mode is not found"))
		};
		let extent = match surface_caps.currentExtent
		{
			VkExtent2D(std::u32::MAX, _) | VkExtent2D(_, std::u32::MAX) => size,
			_ => surface_caps.currentExtent
		};

		// set information and create //
		let queue_family_indices = [engine.get_device().get_graphics_queue().family_index];
		let scinfo = VkSwapchainCreateInfoKHR
		{
			sType: VkStructureType::SwapchainCreateInfoKHR, pNext: std::ptr::null(),
			minImageCount: std::cmp::max(surface_caps.minImageCount, 2), imageFormat: format.format, imageColorSpace: format.colorSpace,
			imageExtent: extent, imageArrayLayers: 1, imageUsage: VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT,
			imageSharingMode: VkSharingMode::Exclusive, compositeAlpha: VK_COMPOSITE_ALPHA_OPAQUE_BIT,
			preTransform: VK_SURFACE_TRANSFORM_IDENTITY_BIT, presentMode: *present_mode, clipped: true as VkBool32,
			pQueueFamilyIndices: queue_family_indices.as_ptr(), queueFamilyIndexCount: queue_family_indices.len() as u32,
			oldSwapchain: std::ptr::null_mut(), flags: 0, surface: surface.get()
		};
		let sc = try!(vk::Swapchain::new(engine.get_device().get_internal(), &surface, &scinfo).map(|x| Rc::new(x)));
		let rt_images = try!(sc.get_images());
		let rt = try!(rt_images.iter().map(|&res|
		{
			vk::ImageView::new(engine.get_device().get_internal(), &VkImageViewCreateInfo
			{
				sType: VkStructureType::ImageViewCreateInfo, pNext: std::ptr::null(), flags: 0,
				image: res, subresourceRange: vk::ImageSubresourceRange::default_color(),
				format: format.format, viewType: VkImageViewType::Dim2,
				components: VkComponentMapping::default()
			}).map(|v| WindowRenderTarget { resource: res, view: v })
		}).collect::<Result<Vec<_>, _>>());
		
		engine.create_queue_fence().and_then(|backbuffer_available_signal|
		engine.create_queue_fence().map(|transfer_complete_signal| Box::new(Window
		{
			server: server.clone(), native: native_w, device_obj: surface, swapchain: sc, render_targets: rt,
			format: format.format, extent: extent, has_vsync: *present_mode == VkPresentModeKHR::FIFO,
			backbuffer_available_signal: backbuffer_available_signal,
			transfer_complete_signal: transfer_complete_signal
		})))
	}
}
impl<N: NativeWindow> RenderWindow for Window<N>
{
	fn get_back_images(&self) -> Vec<&WindowRenderTarget> { self.render_targets.iter().collect() }
	fn get_format(&self) -> VkFormat { self.format }
	fn get_extent(&self) -> VkExtent2D { self.extent }
	fn present(&self, gqueue: &vk::Queue, index: u32) -> Result<(), EngineError>
	{
		self.swapchain.present(gqueue, index, &[]).map_err(EngineError::from)
	}
	fn acquire_next_backbuffer_index(&self, wait_semaphore: &QueueFence) -> Result<u32, EngineError>
	{
		self.swapchain.acquire_next_image(wait_semaphore.get_internal()).map_err(EngineError::from)
	}
}

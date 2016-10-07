// Prelude: Window and RenderWindow

#![allow(dead_code)]

use super::internals::*;
use {std, vk};
use std::rc::Rc;
use std::sync::Arc;
use vk::ffi::*;
use vk::traits::*;
use xcb::ffi::*;
use std::os::raw::*;

// Global Shared atomic value
pub enum XInternAtom<'a>
{
	Requesting(&'a *mut xcb_connection_t, xcb_intern_atom_cookie_t), Arrived(xcb_atom_t)
}
impl <'a> XInternAtom<'a>
{
	fn request(con: &'a *mut xcb_connection_t, name: &str) -> Self
	{
		XInternAtom::Requesting(con, unsafe { xcb_intern_atom(*con, false as u8, name.len() as u16, name.as_ptr() as *const c_char) })
	}
	fn wait_reply(self) -> Self
	{
		match self
		{
			XInternAtom::Requesting(con, c) => XInternAtom::Arrived(unsafe { *xcb_intern_atom_reply(*con, c, std::ptr::null_mut()) }.atom),
			_ => self
		}
	}
	fn unwrap(&self) -> xcb_atom_t
	{
		match self { &XInternAtom::Arrived(a) => a, _ => panic!("Unwrapping Unarrived Data") }
	}
}

#[derive(PartialEq)]
pub enum ApplicationState
{
	Continued, Exited
}
pub trait NativeWindow
{
	fn xcb_show(&self, server: *mut xcb_connection_t);
	fn xcb_surface_create_info(&self, server: *mut xcb_connection_t) -> VkXcbSurfaceCreateInfoKHR;
}
impl NativeWindow for xcb_window_t
{
	fn xcb_show(&self, server: *mut xcb_connection_t)
	{
		unsafe { xcb_map_window(server, *self) };
	}
	fn xcb_surface_create_info(&self, server: *mut xcb_connection_t) -> VkXcbSurfaceCreateInfoKHR
	{
		VkXcbSurfaceCreateInfoKHR
		{
			sType: VkStructureType::XcbSurfaceCreateInfoKHR, pNext: std::ptr::null(), flags: 0,
			connection: server, window: *self
		}
	}
}
pub trait WindowServer: std::marker::Sync + std::marker::Send
{
	fn create_unresizable_window(&self, size: VkExtent2D, title: &str) -> Result<Box<NativeWindow>, EngineError>;
	fn show_window(&self, target: &NativeWindow);
	fn flush(&self);
	fn process_events(&self) -> ApplicationState;
	fn process_all_events(&self);
	fn is_vk_presentation_support(&self, adapter: &vk::PhysicalDevice, qf_index: u32) -> bool;
	fn make_vk_surface(&self, target: &NativeWindow, instance: &Rc<vk::Instance>) -> Result<vk::Surface, EngineError>;
}
pub struct XServer
{
	internal: *mut xcb_connection_t,
	root_depth: u8, root_visual: xcb_visualid_t, root_window: xcb_window_t,
	atom_protocols: xcb_atom_t, atom_delete_window: xcb_atom_t
}
unsafe impl Sync for XServer {}
unsafe impl Send for XServer {}
impl InternalExports<*mut xcb_connection_t> for XServer
{
	fn get_internal(&self) -> &*mut xcb_connection_t { &self.internal }
}
impl XServer
{
	pub fn connect() -> Result<Arc<WindowServer>, EngineError>
	{
		let mut screen_num = 0i32;
		let con_ptr = unsafe { xcb_connect(std::ptr::null(), &mut screen_num) };
		let con_err = unsafe { xcb_connection_has_error(con_ptr) };
		if con_err > 0 { return Err(EngineError::XServerError(con_err)); }
		let setup = unsafe { xcb_get_setup(con_ptr) };
		fn recursive(mut iter: xcb_screen_iterator_t, iterate_rest: i32) -> Option<*mut xcb_screen_t>
		{
			if iterate_rest <= 0 { Some(iter.data) }
			else if iter.rem == 0 { None }
			else { recursive(unsafe { xcb_screen_next(&mut iter); iter }, iterate_rest - 1) }
		}
		let root_scr = try!(recursive(unsafe { xcb_setup_roots_iterator(setup) }, screen_num).ok_or(EngineError::GenericError("XServer Root Screen not found")));
		let rd = unsafe { (*root_scr).root_depth };
		let rv = unsafe { (*root_scr).root_visual };
		let rw = unsafe { (*root_scr).root };

		// Register callback on Window Destroy //
		let protocols_atom = XInternAtom::request(&con_ptr, "WM_PROTOCOLS");
		let delete_window_atom = XInternAtom::request(&con_ptr, "WM_DELETE_WINDOW");

		Ok(Arc::new(XServer
		{
			internal: con_ptr,
			root_depth: rd, root_visual: rv, root_window: rw,
			atom_protocols: protocols_atom.wait_reply().unwrap(), atom_delete_window: delete_window_atom.wait_reply().unwrap()
		}))
	}
}
impl WindowServer for XServer
{
	fn create_unresizable_window(&self, size: VkExtent2D, title: &str) -> Result<Box<NativeWindow>, EngineError>
	{
		let object_id = unsafe { xcb_generate_id(self.internal) };
		let VkExtent2D(width, height) = size;
		unsafe { xcb_create_window(self.internal, self.root_depth, object_id, self.root_window,
			0, 0, width as u16, height as u16, 0, XCB_WINDOW_CLASS_INPUT_OUTPUT as u16,
			self.root_visual, 0, std::ptr::null()) };
		let window_size_hint_params = [
			16 | 32, 0, 0, 0, 0, // PMinSize | PMaxSize, pad1, pad2, pad3, pad4
			width as i32, height as i32, 0, 0, 0, 0	// max_width, max_height, width_inc, height_inc, max_aspect[2]
		];
		unsafe { xcb_change_property(self.internal, XCB_PROP_MODE_REPLACE as u8, object_id, XCB_ATOM_WM_NAME, XCB_ATOM_STRING, 8, title.len() as u32, std::mem::transmute(title.as_ptr())) };
		unsafe { xcb_change_property(self.internal, XCB_PROP_MODE_REPLACE as u8, object_id, self.atom_protocols, 4, 32, 1, std::mem::transmute(&self.atom_delete_window)) };
		unsafe { xcb_change_property(self.internal, XCB_PROP_MODE_REPLACE as u8, object_id, XCB_ATOM_WM_NORMAL_HINTS, XCB_ATOM_WM_SIZE_HINTS, 32, window_size_hint_params.len() as u32 * 4, std::mem::transmute(window_size_hint_params.as_ptr())) };

		Ok(Box::new(object_id))
	}
	fn show_window(&self, target: &NativeWindow)
	{
		target.xcb_show(self.internal);
	}
	fn flush(&self)
	{
		unsafe { xcb_flush(self.internal) };
	}
	fn process_events(&self) -> ApplicationState
	{
		fn recursive(this: &XServer, event_obj: *mut xcb_generic_event_t) -> ApplicationState
		{
			if !event_obj.is_null()
			{
				let event = &unsafe { *event_obj };
				match event.response_type & 0x7f
				{
					XCB_CLIENT_MESSAGE =>
					{
						let cm_event: &xcb_client_message_event_t = unsafe { std::mem::transmute(event) };
						let event_data: [u32; 5] = unsafe { std::mem::transmute(cm_event.data) };
						if event_data[0] == this.atom_delete_window
						{
							ApplicationState::Exited
						}
						else { recursive(this, unsafe { xcb_poll_for_event(this.internal) }) }
					},
					34 => recursive(this, unsafe { xcb_poll_for_event(this.internal) })/* keymap change event(ignore) */,
					_ =>
					{
						info!(target: "Prelude <- XServer", "xcb_event_response: {}", event.response_type);
						recursive(this, unsafe { xcb_poll_for_event(this.internal) })
					}
				}
			}
			else { ApplicationState::Continued }
		}

		recursive(self, unsafe { xcb_poll_for_event(self.internal) })
	}
	fn process_all_events(&self)
	{
		loop
		{
			let event_ptr = unsafe { xcb_wait_for_event(self.internal) };
			assert!(!event_ptr.is_null());
			let event = &unsafe { *event_ptr };
			match event.response_type & 0x7f
			{
				XCB_CLIENT_MESSAGE =>
				{
					let cm_event: &xcb_client_message_event_t = unsafe { std::mem::transmute(event) };
					let event_data: [u32; 5] = unsafe { std::mem::transmute(cm_event.data) };
					if event_data[0] == self.atom_delete_window { break; }
				},
				34 => /* keymap change event(ignored) */(),
				_ => info!(target: "Prelude <- XServer", "xcb_event_response: {:02x}", event.response_type)
			}
		}
	}
	fn is_vk_presentation_support(&self, adapter: &vk::PhysicalDevice, qf_index: u32) -> bool
	{
		adapter.is_xcb_presentation_support(qf_index, self.internal, self.root_visual)
	}
	fn make_vk_surface(&self, target: &NativeWindow, instance: &Rc<vk::Instance>) -> Result<vk::Surface, EngineError>
	{
		vk::Surface::new_xcb(instance, &target.xcb_surface_create_info(self.internal)).map_err(EngineError::from)
	}
}
impl std::ops::Drop for XServer
{
	fn drop(&mut self)
	{
		unsafe { xcb_disconnect(self.internal) };
	}
}
pub fn connect_to_window_server() -> Result<Arc<WindowServer>, EngineError>
{
	XServer::connect()
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

pub trait InternalWindow where Self: std::marker::Sized
{
	fn create_unresizable(engine: &Engine, size: VkExtent2D, title: &str) -> Result<Box<Self>, EngineError>;
}
pub trait RenderWindow: std::marker::Send
{
	fn get_back_images(&self) -> Vec<&EntireImage>;
	fn get_format(&self) -> VkFormat;
	fn get_extent(&self) -> VkExtent2D;
	fn acquire_next_backbuffer_index(&self, wait_semaphore: &QueueFence) -> Result<u32, EngineError>;
	fn present(&self, gqueue: &vk::Queue, index: u32) -> Result<(), EngineError>;
}
pub struct EntireImage { pub resource: VkImage, pub view: vk::ImageView }
impl ImageResource for EntireImage { fn get_resource(&self) -> VkImage { self.resource } }
impl ImageView for EntireImage { fn get_native(&self) -> VkImageView { self.view.get() } }
pub struct Window
{
	#[allow(dead_code)] server: Arc<WindowServer>, #[allow(dead_code)] native: Box<NativeWindow>,
	#[allow(dead_code)] device_obj: Rc<vk::Surface>, swapchain: Rc<vk::Swapchain>, render_targets: Vec<EntireImage>,
	format: VkFormat, extent: VkExtent2D, has_vsync: bool,
	backbuffer_available_signal: QueueFence, transfer_complete_signal: QueueFence
}
unsafe impl Send for Window {}
impl Window
{
	pub fn create_unresizable(engine: &Engine, size: VkExtent2D, title: &str) -> Result<Box<Self>, EngineError>
	{
		let server = engine.get_window_server();
		let native_w = try!(server.create_unresizable_window(size, title));
		server.show_window(&*native_w);
		server.flush();
		let surface = Rc::new(try!(server.make_vk_surface(&*native_w, engine.get_instance())));
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
			}).map(|v| EntireImage { resource: res, view: v })
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
impl RenderWindow for Window
{
	fn get_back_images(&self) -> Vec<&EntireImage> { self.render_targets.iter().collect() }
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

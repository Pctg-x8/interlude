// XCB Window Server and Native Window Implementation

use {std, vk};
use xcb::ffi::*;
use super::super::ffi::*;
use std::os::raw::*;
use super::super::internals::*;
use std::sync::Arc;
use std::rc::Rc;
use epoll;
use std::os::unix::io::*;

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

// xcb_window_t as NativeWindow
impl NativeWindow for xcb_window_t
{
	type NativeWindowServerT = XServer;
	type SurfaceCreateInfoKHR = VkXcbSurfaceCreateInfoKHR;

	fn native_show(&self, server: &Self::NativeWindowServerT) { unsafe { xcb_map_window(server.internal, *self); } }
	fn native_surface_create_info(&self, server: &Self::NativeWindowServerT) -> Self::SurfaceCreateInfoKHR
	{
		VkXcbSurfaceCreateInfoKHR
		{
			sType: VkStructureType::XcbSurfaceCreateInfoKHR, pNext: std::ptr::null(), flags: 0, connection: server.internal, window: *self
		}
	}
	fn destroy(&self) { /* nothing to do */ }
}

// XCB(X11) Server
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
	pub fn connect() -> Result<Arc<Self>, EngineError>
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
	type NativeWindowT = xcb_window_t;

	fn create_unresizable_window(&self, size: VkExtent2D, title: &str) -> Result<Self::NativeWindowT, EngineError>
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

		Ok(object_id)
	}
	fn show_window(&self, target: &Self::NativeWindowT)
	{
		target.native_show(self);
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
			else { ApplicationState::Continue }
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
	fn process_events_and_messages(&self, events: &[&Event]) -> ApplicationState
	{
		let mut polling = epoll::EpollInstance::new().unwrap();
		polling.add_interest(epoll::Interest::new(self.as_raw_fd(), epoll::EPOLLIN, std::u64::MAX)).unwrap();
		for (n, &ev) in events.into_iter().enumerate() { polling.add_interest(epoll::Interest::new(ev.as_raw_fd(), epoll::EPOLLIN, n as u64)).unwrap(); }
		if let Ok(events) = polling.wait(-1, 1)
		{
			if events[0].data() == std::u64::MAX
			{
				// xcb events
				self.process_events()
			}
			else { ApplicationState::EventArrived(events[0].data() as u32) }
		}
		else { ApplicationState::Exited }
	}
	fn is_vk_presentation_support(&self, adapter: &vk::PhysicalDevice, qf_index: u32) -> bool
	{
		adapter.is_xcb_presentation_support(qf_index, self.internal, self.root_visual)
	}
	fn make_vk_surface(&self, target: &Self::NativeWindowT, instance: &Rc<vk::Instance>) -> Result<vk::Surface, EngineError>
	{
		vk::Surface::new_xcb(instance, &target.native_surface_create_info(self)).map_err(EngineError::from)
	}
}
impl std::ops::Drop for XServer
{
	fn drop(&mut self)
	{
		unsafe { xcb_disconnect(self.internal) };
	}
}
impl AsRawFd for XServer
{
	fn as_raw_fd(&self) -> RawFd { unsafe { base::xcb_get_file_descriptor(self.internal) } }
}
pub fn connect_xserver() -> Result<Arc<XServer>, EngineError> { XServer::connect() }

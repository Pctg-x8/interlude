// XCB Window Server and Native Window Implementation

use {std, vk};
use {ApplicationState, EngineResult};
use xcb::ffi::*;
use vkdefs::*;
use std::os::raw::*;
use super::super::internals::*;
use std::rc::Rc;
use std::os::unix::io::*;
use mio;
use super::vk_wsi::*;

// xcb_window_t and server connection
pub struct NativeWindowAndServerCon(xcb_window_t, XServer, xcb_atom_t);
impl NativeWindowAndServerCon
{
	pub fn new(size: &Size2, caption: &str, resizable: bool) -> Result<Self, EngineError>
	{
		XServer::connect().map(|srv|
		{
			let (object_id, atom_delete_window) =
			{
				let atom_protocols = srv.request_intern_atom("WM_PROTOCOLS");
				let atom_delete_window = srv.request_intern_atom("WM_DELETE_WINDOW").wait_reply().unwrap();
				let object_id = srv.generate_id();
				let &Size2(width, height) = size;
				srv.create_window(object_id, None, None, 0, 0, width as u16, height as u16, 0, XCB_WINDOW_CLASS_INPUT_OUTPUT as u16, None, 0, &[]);
				srv.replace_property_str(object_id, XCB_ATOM_WM_NAME, caption);
				srv.replace_property_atom(object_id, atom_protocols.wait_reply().unwrap(), atom_delete_window);
				if !resizable
				{
					let window_size_hint_params = [
						16 | 32, 0, 0, 0, 0, // PMinSize | PMaxSize, pad1, pad2, pad3, pad4
						width as u32, height as u32, 0, 0, 0, 0	// max_width, max_height, width_inc, height_inc, max_aspect[2]
					];
					srv.replace_property_typed_u32s(object_id, XCB_ATOM_WM_NORMAL_HINTS, XCB_ATOM_WM_SIZE_HINTS, &window_size_hint_params);
				}

				srv.show(object_id);
				srv.flush();
				(object_id, atom_delete_window)
			};
			NativeWindowAndServerCon(object_id, srv, atom_delete_window)
		})
	}
	pub fn show(&self) { self.1.show(self.0); }
	pub fn make_vk_surface(&self, instance: &Rc<vk::Instance>) -> EngineResult<Surface>
	{
		Surface::new(instance, &VkXcbSurfaceCreateInfoKHR
		{
			sType: VkStructureType::XcbSurfaceCreateInfoKHR, pNext: std::ptr::null(), flags: 0, connection: self.1.ptr, window: self.0
		}).map_err(EngineError::from)
	}

	pub fn process_messages(&self) -> ApplicationState { self.1.process_events(self.2) }
	pub fn process_all_messages(&self) { self.1.process_all_events(self.2); }
	pub fn process_events_and_messages(&self, events: &[&Event]) -> ApplicationState
	{
		self.1.process_events_and_messages(events, self.2)
	}
	pub fn is_vk_presentation_support(&self, adapter: &vk::PhysicalDevice, qf_index: u32) -> bool
	{
		self.1.is_vk_presentation_support(adapter, qf_index)
	}
}

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

// XCB(X11) Server
const T_SERVER: mio::Token = mio::Token(std::usize::MAX - 1);
struct XServer
{
	ptr: *mut xcb_connection_t, root_visual: xcb_visualid_t, root_window: xcb_window_t
}
unsafe impl Sync for XServer {}
unsafe impl Send for XServer {}
impl XServer
{
	pub fn connect() -> EngineResult<Self>
	{
		let mut screen_num = 0;
		let con_ptr = unsafe { xcb_connect(std::ptr::null(), &mut screen_num) };
		let con_err = unsafe { xcb_connection_has_error(con_ptr) };
		if con_err <= 0
		{
			let setup = unsafe { xcb_get_setup(con_ptr) };
			fn recursive(mut iter: xcb_screen_iterator_t, iterate_rest: i32) -> Option<*mut xcb_screen_t>
			{
				if iterate_rest <= 0 { Some(iter.data) }
				else if iter.rem == 0 { None }
				else { recursive(unsafe { xcb_screen_next(&mut iter); iter }, iterate_rest - 1) }
			}
			recursive(unsafe { xcb_setup_roots_iterator(setup) }, screen_num).ok_or(EngineError::GenericError("XServer Root screen could not be found"))
				.map(|root_scr| unsafe { let ref r = *root_scr; (r.root_visual, r.root) })
				.map(|(rv, rw)| XServer { ptr: con_ptr, root_visual: rv, root_window: rw })
		}
		else { Err(EngineError::XServerError(con_err)) }
	}
	pub fn generate_id(&self) -> u32 { unsafe { xcb_generate_id(self.ptr) } }
	pub fn create_window(&self, id: u32, parent: Option<xcb_window_t>, depth: Option<u8>, x: i16, y: i16, w: u16, h: u16,
		border_width: u16, class: u16, visual: Option<xcb_visualid_t>, value_mask: u32, values: &[u32])
	{
		unsafe
		{
			xcb_create_window(self.ptr, depth.unwrap_or(XCB_COPY_FROM_PARENT as u8), id, parent.unwrap_or(self.root_window),
				x, y, w, h, border_width, class, visual.unwrap_or(XCB_COPY_FROM_PARENT as xcb_visualid_t), value_mask, values.as_ptr())
		};
	}
	pub fn request_intern_atom(&self, atom_str: &str) -> XInternAtom { XInternAtom::request(&self.ptr, atom_str) }
	pub fn flush(&self) { unsafe { xcb_flush(self.ptr) }; }
	pub fn show(&self, id: xcb_window_t) { unsafe { xcb_map_window(self.ptr, id) }; }
	
	pub fn replace_property_str(&self, id: xcb_window_t, property: xcb_atom_t, data: &str)
	{
		unsafe { xcb_change_property(self.ptr, XCB_PROP_MODE_REPLACE as u8, id, property, XCB_ATOM_STRING, 8, data.len() as u32, std::mem::transmute(data.as_ptr())) };
	}
	pub fn replace_property_atom(&self, id: xcb_window_t, property: xcb_atom_t, data: xcb_atom_t)
	{
		unsafe { xcb_change_property(self.ptr, XCB_PROP_MODE_REPLACE as u8, id, property, 4, 32, 1, std::mem::transmute(&data)) };
	}
	pub fn replace_property_typed_u32s(&self, id: xcb_window_t, property: xcb_atom_t, _type: xcb_atom_t, data: &[u32])
	{
		unsafe { xcb_change_property(self.ptr, XCB_PROP_MODE_REPLACE as u8, id, property, _type, 32, data.len() as u32, std::mem::transmute(data.as_ptr())) };
	}

	// Old WindowServer implementations //
	fn process_events(&self, atom_delete_window: xcb_atom_t) -> ApplicationState
	{
		fn recursive(this: &XServer, event_obj: *mut xcb_generic_event_t, atom_delete_window: xcb_atom_t) -> ApplicationState
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
						if event_data[0] == atom_delete_window
						{
							ApplicationState::Exited
						}
						else { recursive(this, unsafe { xcb_poll_for_event(this.ptr) }, atom_delete_window) }
					},
					34 => recursive(this, unsafe { xcb_poll_for_event(this.ptr) }, atom_delete_window)/* keymap change event(ignore) */,
					_ =>
					{
						info!(target: "Interlude <- XServer", "xcb_event_response: {}", event.response_type);
						recursive(this, unsafe { xcb_poll_for_event(this.ptr) }, atom_delete_window)
					}
				}
			}
			else { ApplicationState::Continue }
		}

		recursive(self, unsafe { xcb_poll_for_event(self.ptr) }, atom_delete_window)
	}
	fn process_all_events(&self, atom_delete_window: xcb_atom_t)
	{
		loop
		{
			let event_ptr = unsafe { xcb_wait_for_event(self.ptr) };
			assert!(!event_ptr.is_null());
			let event = &unsafe { *event_ptr };
			match event.response_type & 0x7f
			{
				XCB_CLIENT_MESSAGE =>
				{
					let cm_event: &xcb_client_message_event_t = unsafe { std::mem::transmute(event) };
					let event_data: [u32; 5] = unsafe { std::mem::transmute(cm_event.data) };
					if event_data[0] == atom_delete_window { break; }
				},
				34 => /* keymap change event(ignored) */(),
				_ => info!(target: "Interlude <- XServer", "xcb_event_response: {:02x}", event.response_type)
			}
		}
	}
	fn process_events_and_messages(&self, events: &[&Event], atom_delete_window: xcb_atom_t) -> ApplicationState
	{
		use mio::*;
		use mio::unix::EventedFd;

		let polling = Poll::new().unwrap();
		polling.register(&EventedFd(&self.as_raw_fd()), T_SERVER, Ready::readable(), PollOpt::edge()).unwrap();
		for (n, &ev) in events.into_iter().enumerate() { polling.register(&EventedFd(&ev.as_raw_fd()), Token(n), Ready::readable(), PollOpt::edge()).unwrap(); }
		let mut events = Events::with_capacity(1);
		if let Ok(1) = polling.poll(&mut events, None)
		{
			match events.get(0).unwrap().token()
			{
				T_SERVER => self.process_events(atom_delete_window),
				Token(v) => ApplicationState::EventArrived(v as u32)
			}
		}
		else { ApplicationState::Exited }
	}
	fn is_vk_presentation_support(&self, adapter: &vk::PhysicalDevice, qf_index: u32) -> bool
	{
		unsafe { vkGetPhysicalDeviceXcbPresentationSupportKHR(**adapter, qf_index, self.ptr, self.root_visual) == true as VkBool32 }
	}
}
impl std::ops::Drop for XServer { fn drop(&mut self) { unsafe { xcb_disconnect(self.ptr) }; } }
impl AsRawFd for XServer { fn as_raw_fd(&self) -> RawFd { unsafe { xcb_get_file_descriptor(self.ptr) } } }

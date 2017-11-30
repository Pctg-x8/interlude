//! xlib target window system intergration code

use subsystem_layer::{NativeInstance, NativeHandleProvider, NativeResultValueHandler};
use x11::xlib;
use x11::xlib::*;
use std::ffi::CString;
use std::mem::uninitialized as reserved;
use std::ptr::null;
use std::usize;
use std::os::unix::io::AsRawFd;
use interlude_vk_defs::{VkXlibSurfaceCreateInfoKHR, VkBool32, VkPhysicalDevice, VkSurfaceKHR};
use interlude_vk_funport::{vkCreateXlibSurfaceKHR, vkGetPhysicalDeviceXlibPresentationSupportKHR};
use {ApplicationState, Event, EngineResult, EngineError, Size2};
use mio::*;
use libc::c_ulong;

const T_SERVER: Token = Token(usize::MAX - 1);

/// Server connection + Window ID + Closing Message ID
pub struct NativeWindowWithServer
{
	display: *mut Display, window: Window, close_id: Atom, fixed_size: Option<Size2>
}
impl super::NativeWindowBase for NativeWindowWithServer
{
	fn new(size: &Size2, caption: &str, resizable: bool) -> EngineResult<Self>
	{
		let &Size2(w, h) = size;
		let display = unsafe { XOpenDisplay(null()) };
		if display.is_null() { return Err(EngineError::XServerError(-1)); }
		let root = unsafe { XDefaultRootWindow(display) };
		let window = unsafe { XCreateSimpleWindow(display, root, 0, 0, w, h, 1, 0, 0) };
		if window == 0 { return Err(EngineError::XServerError(-1)); }
		let mut close_id = unsafe { XInternAtom(display, "WM_DELETE_WINDOW\x00".as_ptr() as _, false as _) };
		if close_id == 0 { return Err(EngineError::XServerError(-1)); }
		unsafe { XSetWMProtocols(display, window, &mut close_id, 1) };
		unsafe { XStoreName(display, window, CString::new(caption)?.as_ptr()) };

		Ok(NativeWindowWithServer
		{
			display, window, close_id, fixed_size: if resizable { Some(size.clone()) } else { None }
		})
	}
	fn show(&self) { unsafe { XMapWindow(self.display, self.window) }; }
	fn flush(&self) { unsafe { XFlush(self.display) }; }
	n make_vk_surface(&self, instance: &NativeInstance) -> EngineResult<VkSurfaceKHR>
	{
		let cinfo = VkXlibSurfaceCreateInfoKHR { pdy: self.display, window: self.window, .. Default::default() };
		let mut surface = unsafe { reserved() };
		unsafe { vkCreateXlibSurfaceKHR(instance.native(), &cinfo, null(), &mut surface) }.make_result(surface)
	}
	fn can_vk_present(&self, adapter: VkPhysicalDevice, queue_family_index: u32) -> bool
	{
		unsafe { vkGetPhysicalDeviceXlibPresentationSupportKHR(adapter, queue_family_index, self.display, self.window) == true as VkBool32 }
	}

	fn process_events_and_messages(&self, events: &[&Event]) -> ApplicationState
	{
		let polling = Poll::new().expect("Failed to create polling instance");
		polling.register(&unix::EventedFd(unsafe { &XConnectionNumber(self.display) }), T_SERVER, Ready::readable(), PollOpt::level())
			.expect("Failed to register polling event from display server");
		for (n, &e) in events.into_iter().enumerate()
		{
			polling.register(&unix::EventedFd(&e.as_raw_fd()), Token(n), Ready::readable(), PollOpt::edge()).expect("Failed to register user polling event");
		}
		let mut events = Events::with_capacity(1);
		polling.poll(&mut events, None).map(|_| match events.get(0).unwrap().token()
		{
			T_SERVER => self.process_messages(),
			Token(v) => ApplicationState::EventArrived(v as _)
		}).unwrap_or(ApplicationState::Exited)
	}
	fn process_messages(&self) -> ApplicationState
	{
		while unsafe { XPending(self.display) } > 0
		{
			let mut event = unsafe { reserved() };
			unsafe { XNextEvent(self.display, &mut event) };
			match event.get_type()
			{
				xlib::ClientMessage if (&event as &AsRef<XClientMessageEvent>).as_ref().data.get_long(0) as c_ulong == self.close_id =>
					return ApplicationState::Exited,
				ty => info!(target: "Interlude <- X11", "Unhandled Event: {}", ty)
			}
		}
		ApplicationState::Continue
	}
}
impl Drop for NativeWindowWithServer
{
	fn drop(&mut self)
	{
		unsafe { XCloseDisplay(self.display) };
	}
}

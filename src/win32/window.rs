// Win32 Window Server(dummy) and NativeWindow Implementation

use {std, vk};
use super::super::ffi::*;
use super::super::internals::*;
use std::sync::Arc;
use std::rc::Rc;
use winapi::*; use kernel32::*; use user32::*;
use widestring;

unsafe extern "system" fn common_wndproc(hwnd: HWND, msg: UINT, wp: WPARAM, lp: LPARAM) -> LRESULT
{
	match msg
	{
		WM_DESTROY => { hwnd.destroy(); DefWindowProcW(hwnd, msg, wp, lp) },
		WM_INPUT => { info!(target: "interlude::win32::window", "WM_INPUT Arrived."); DefWindowProcW(hwnd, msg, wp, lp) },
		_ => DefWindowProcW(hwnd, msg, wp, lp)
	}
}

// HWND as NativeWindow
impl NativeWindow for HWND
{
	type NativeWindowServerT = Win32Server;
	type SurfaceCreateInfoKHR = VkWin32SurfaceCreateInfoKHR;

	fn native_show(&self, _: &Self::NativeWindowServerT) { unsafe { ShowWindow(*self, SW_SHOWNORMAL); } }
	fn native_surface_create_info(&self, server: &Self::NativeWindowServerT) -> Self::SurfaceCreateInfoKHR
	{
		VkWin32SurfaceCreateInfoKHR
		{
			sType: VkStructureType::Win32SurfaceCreateInfoKHR, pNext: std::ptr::null(), flags: 0,
			hinstance: server.appinstance, hwnd: *self
		}
	}
	fn destroy(&self) { unsafe { PostQuitMessage(0); } }
}

// Win32 Window Server(dummy object)
pub struct Win32Server
{
	appinstance: HINSTANCE, common_class: ATOM
}
unsafe impl Sync for Win32Server {}
unsafe impl Send for Win32Server {}
impl Win32Server
{
	pub fn connect() -> Result<Arc<Self>, EngineError>
	{
		let appinstance = unsafe { GetModuleHandleW(std::ptr::null()) } as HINSTANCE;
		let classname = widestring::WideCString::from_str("InterludeDefaultWnd").unwrap();
		let wce = WNDCLASSEXW
		{
			cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
			cbClsExtra: 0, cbWndExtra: 0,
			hInstance: appinstance,
			hCursor: unsafe { LoadCursorW(std::ptr::null_mut(), IDC_ARROW) },
			style: CS_OWNDC,
			hbrBackground: std::ptr::null_mut(), hIcon: std::ptr::null_mut(),
			hIconSm: std::ptr::null_mut(),
			lpfnWndProc: Some(common_wndproc), lpszClassName: classname.as_ptr(),
			lpszMenuName: std::ptr::null()
		};
		let comclass = unsafe { RegisterClassExW(&wce) };
		if comclass <= 0 { Err(EngineError::GenericError("Unable to register window class")) }
		else
		{
			Ok(Arc::new(Win32Server
			{
				appinstance: appinstance, common_class: comclass
			}))
		}
	}
}
impl WindowServer for Win32Server
{
	type NativeWindowT = HWND;

	fn create_unresizable_window(&self, size: &Size2, title: &str) -> Result<Self::NativeWindowT, EngineError>
	{
		let &Size2(width, height) = size;
		let title_str = widestring::WideCString::from_str(title).unwrap();
		let wstyle = WS_OVERLAPPED | WS_CAPTION | WS_BORDER | WS_SYSMENU | WS_MINIMIZEBOX;
		let mut r = RECT { left: 0, top: 0, right: width as i32, bottom: height as i32 };
		unsafe { AdjustWindowRectEx(&mut r, wstyle, FALSE, 0) };
		let wnd = unsafe { CreateWindowExW(0, std::mem::transmute((self.common_class as usize) & 0x0000ffff), title_str.as_ptr(), wstyle,
			CW_USEDEFAULT, CW_USEDEFAULT, r.right - r.left, r.bottom - r.top, std::ptr::null_mut(), std::ptr::null_mut(), self.appinstance, std::ptr::null_mut()) };
		if wnd.is_null() { Err(EngineError::GenericError("Unable to create win32 window")) } else { Ok(wnd) }
	}
	fn show_window(&self, target: &Self::NativeWindowT) { target.native_show(self); }
	fn flush(&self) {}
	fn process_events(&self) -> ApplicationState
	{
		let mut msg: MSG = unsafe { std::mem::uninitialized() };
		while unsafe { PeekMessageW(&mut msg, std::ptr::null_mut(), 0, 0, PM_REMOVE) } != 0
		{
			if msg.message == WM_QUIT { return ApplicationState::Exited }
			unsafe { TranslateMessage(&mut msg) };
			unsafe { DispatchMessageW(&mut msg) };
		}
		ApplicationState::Continue
	}
	fn process_all_events(&self)
	{
		let mut msg: MSG = unsafe { std::mem::uninitialized() };
		while unsafe { GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) } > 0
		{
			unsafe { TranslateMessage(&mut msg) };
			unsafe { DispatchMessageW(&mut msg) };
		}
	}
	fn process_events_and_messages(&self, events: &[&Event]) -> ApplicationState
	{
		let ev_handles = events.iter().map(|x| x.get_internal()).collect::<Vec<_>>();
		let res = unsafe { MsgWaitForMultipleObjectsEx(events.len() as u32, ev_handles.as_ptr(), INFINITE,
			QS_ALLEVENTS, MWMO_INPUTAVAILABLE) };
		if res == WAIT_OBJECT_0 + events.len() as u32
		{
			let mut msg: MSG = unsafe { std::mem::uninitialized() };
			while unsafe { PeekMessageW(&mut msg, std::ptr::null_mut(), 0, 0, PM_REMOVE) > 0 }
			{
				if msg.message == WM_QUIT { return ApplicationState::Exited }
				unsafe { TranslateMessage(&mut msg); DispatchMessageW(&mut msg); }
			}
			ApplicationState::Continue
		}
		else if WAIT_OBJECT_0 <= res && res < WAIT_OBJECT_0 + events.len() as u32
		{
			ApplicationState::EventArrived(res - WAIT_OBJECT_0)
		}
		else { ApplicationState::Continue }
	}
	fn is_vk_presentation_support(&self, adapter: &vk::PhysicalDevice, qf_index: u32) -> bool
	{
		adapter.is_platform_presentation_support(qf_index)
	}
	fn make_vk_surface(&self, target: &Self::NativeWindowT, instance: &Rc<vk::Instance>) -> Result<vk::Surface, EngineError>
	{
		vk::Surface::new(instance, &target.native_surface_create_info(self)).map_err(EngineError::from)
	}
}
pub fn connect_win32_server() -> Result<Arc<Win32Server>, EngineError> { Win32Server::connect() }

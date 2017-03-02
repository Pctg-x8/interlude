// Win32 Window Server(dummy) and NativeWindow Implementation

use {std, vk, widestring};
use {EngineResult, EngineError, ApplicationState, Event, Size2};
use std::rc::Rc;
use winapi::*; use kernel32::*; use user32::*;
use super::wsi::*;
use ffi::*;

// HWND as NativeWindow
pub struct NativeWindow(HWND);
impl NativeWindow
{
	fn register_class() -> EngineResult<(HINSTANCE, ATOM)>
	{
		let appinstance = unsafe { GetModuleHandleW(std::ptr::null()) as HINSTANCE };
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
			lpfnWndProc: Some(Self::wndproc), lpszClassName: classname.as_ptr(),
			lpszMenuName: std::ptr::null()
		};
		let comclass = unsafe { RegisterClassExW(&wce) };
		if comclass <= 0 { Err(EngineError::GenericError("Unable to register window class")) } else { Ok((appinstance, comclass)) }
	}

	pub fn new(size: &Size2, caption: &str, resizable: bool) -> EngineResult<Self>
	{
		Self::register_class().and_then(|(appinstance, wndclass)|
		{
			let &Size2(width, height) = size;
			let title_str = widestring::WideCString::from_str(caption).unwrap();
			let wstyle = WS_OVERLAPPED | WS_CAPTION | WS_BORDER | WS_SYSMENU | WS_MINIMIZEBOX | WS_VISIBLE | if resizable { WS_THICKFRAME } else { 0 };
			let mut r = RECT { left: 0, top: 0, right: width as i32, bottom: height as i32 };
			unsafe { AdjustWindowRectEx(&mut r, wstyle, FALSE, 0) };
			let wnd = unsafe { CreateWindowExW(0, std::mem::transmute((wndclass as usize) & 0x0000ffff), title_str.as_ptr(), wstyle,
				CW_USEDEFAULT, CW_USEDEFAULT, r.right - r.left, r.bottom - r.top, std::ptr::null_mut(), std::ptr::null_mut(), appinstance, std::ptr::null_mut()) };
			if wnd.is_null() { Err(EngineError::GenericError("Unable to create win32 window")) } else { Ok(NativeWindow(wnd)) }
		})
	}
	pub fn show(&self) { unsafe { ShowWindow(self.0, SW_SHOWNORMAL); } }
	pub fn make_vk_surface(&self, instance: &Rc<vk::Instance>) -> EngineResult<Surface>
	{
		Surface::new(instance, &VkWin32SurfaceCreateInfoKHR
		{
			sType: VkStructureType::Win32SurfaceCreateInfoKHR, pNext: std::ptr::null(), flags: 0,
			hinstance: unsafe { GetModuleHandleW(std::ptr::null()) }, hwnd: self.0
		}).map_err(From::from)
	}
	pub fn process_messages(&self) -> ApplicationState
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
	pub fn process_all_messages(&self)
	{
		let mut msg: MSG = unsafe { std::mem::uninitialized() };
		while unsafe { GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) } > 0
		{
			unsafe { TranslateMessage(&mut msg) };
			unsafe { DispatchMessageW(&mut msg) };
		}
	}
	pub fn process_events_and_messages(&self, events: &[&Event]) -> ApplicationState
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
	pub fn is_vk_presentation_support(&self, adapter: &vk::PhysicalDevice, gqf_index: u32) -> bool
	{
		unsafe { vkGetPhysicalDeviceWin32PresentationSupportKHR(**adapter, gqf_index) == true as VkBool32 }
	}
	
	unsafe extern "system" fn wndproc(hwnd: HWND, msg: UINT, wp: WPARAM, lp: LPARAM) -> LRESULT
	{
		match msg
		{
			WM_DESTROY => { PostQuitMessage(0); DefWindowProcW(hwnd, msg, wp, lp) },
			WM_INPUT => { info!(target: "interlude::win32::window", "WM_INPUT Arrived."); DefWindowProcW(hwnd, msg, wp, lp) },
			_ => DefWindowProcW(hwnd, msg, wp, lp)
		}
	}
}

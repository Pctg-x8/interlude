//! win32 target window system integration code

use subsystem_layer::{NativeInstance, NativeHandleProvider, NativeResultValueHandler};
use winapi::*; use user32::*; use kernel32::*;
use widestring::WideCString;
use std; use std::ptr::{null, null_mut}; use std::mem::uninitialized as reserved;
use {EngineResult, EngineError, ApplicationState, Size2, Event};
use interlude_vk_defs::{VkSurfaceKHR, VkWin32SurfaceCreateInfoKHR, VkPhysicalDevice};
use interlude_vk_funport::{vkGetPhysicalDeviceWin32PresentationSupportKHR, vkCreateWin32SurfaceKHR};

pub struct NativeWindow(HWND);
impl super::NativeWindowBase for NativeWindow
{
    fn new(size: &Size2, caption: &str, resizable: bool) -> EngineResult<Self>
    {
        fn register_class() -> EngineResult<(HINSTANCE, ATOM)>
        {
            let appinstance = unsafe { GetModuleHandleW(std::ptr::null()) as HINSTANCE };
            let classname = WideCString::from_str("InterludeDefaultWnd").unwrap();
            let wce = WNDCLASSEXW
            {
                cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32, cbClsExtra: 0, cbWndExtra: 0,
                hInstance: appinstance,
                hCursor: unsafe { LoadCursorW(null_mut(), IDC_ARROW) },
                style: CS_OWNDC,
                hbrBackground: null_mut(), hIcon: null_mut(), hIconSm: null_mut(),
                lpfnWndProc: Some(NativeWindow::wndproc), lpszClassName: classname.as_ptr(), lpszMenuName: null()
            };
            let comclass = unsafe { RegisterClassExW(&wce) };
            if comclass <= 0 { Err(EngineError::GenericError("Unable to register window class")) } else { Ok((appinstance, comclass)) }
        }

		register_class().and_then(|(appinstance, wndclass)|
		{
			let &Size2(width, height) = size;
			let title_str = WideCString::from_str(caption).unwrap();
			let wstyle = WS_OVERLAPPED | WS_CAPTION | WS_BORDER | WS_SYSMENU | WS_MINIMIZEBOX | WS_VISIBLE | if resizable { WS_THICKFRAME } else { 0 };
			let mut r = RECT { left: 0, top: 0, right: width as i32, bottom: height as i32 };
			unsafe { AdjustWindowRectEx(&mut r, wstyle, FALSE, 0) };
			let wnd = unsafe { CreateWindowExW(0, std::mem::transmute((wndclass as usize) & 0x0000ffff), title_str.as_ptr(), wstyle,
				CW_USEDEFAULT, CW_USEDEFAULT, r.right - r.left, r.bottom - r.top, std::ptr::null_mut(), std::ptr::null_mut(), appinstance, std::ptr::null_mut()) };
			if wnd.is_null() { Err(EngineError::GenericError("Unable to create win32 window")) } else { Ok(NativeWindow(wnd)) }
		})
    }
    fn show(&self) { unsafe { ShowWindow(self.0, SW_SHOWNORMAL); } }
    fn flush(&self) { /* nop */ }

    fn can_vk_present(&self, adapter: VkPhysicalDevice, queue_family_index: u32) -> bool
    {
        unsafe { vkGetPhysicalDeviceWin32PresentationSupportKHR(adapter, queue_family_index) != 0 }
    }
    fn make_vk_surface(&self, instance: &NativeInstance) -> EngineResult<VkSurfaceKHR>
    {
        let cinfo = VkWin32SurfaceCreateInfoKHR { hinstance: unsafe { GetModuleHandleW(null()) }, hwnd: self.0, .. Default::default() };
        let mut surface = unsafe { reserved() };
        unsafe { vkCreateWin32SurfaceKHR(instance.native(), &cinfo, null(), &mut surface) }.make_result(surface)
    }

    fn process_events_and_messages(&self, events: &[&Event]) -> ApplicationState
    {
        let ev_handles = events.iter().map(|x| x.get_internal()).collect::<Vec<_>>();
        let res = unsafe { MsgWaitForMultipleObjectsEx(ev_handles.len() as _, ev_handles.as_ptr(), INFINITE, QS_ALLEVENTS, MWMO_INPUTAVAILABLE) };
        if res == WAIT_OBJECT_0 + ev_handles.len() as u32
        {
            let mut msg = unsafe { reserved() };
            while unsafe { PeekMessageW(&mut msg, null_mut(), 0, 0, PM_REMOVE) != 0 }
            {
                if msg.message == WM_QUIT { return ApplicationState::Exited; }
                unsafe { TranslateMessage(&mut msg); DispatchMessageW(&mut msg); }
            }
            ApplicationState::Continue
        }
        else if WAIT_OBJECT_0 <= res && res < WAIT_OBJECT_0 + ev_handles.len() as u32
        {
            ApplicationState::EventArrived(res - WAIT_OBJECT_0)
        }
        else { ApplicationState::Continue }
    }
    fn process_messages(&self) -> ApplicationState
    {
        let mut msg = unsafe { reserved() };
        while unsafe { PeekMessageW(&mut msg, null_mut(), 0, 0, PM_REMOVE) != 0 }
        {
            if msg.message == WM_QUIT { return ApplicationState::Exited; }
            unsafe { TranslateMessage(&mut msg); DispatchMessageW(&mut msg); }
        }
        ApplicationState::Continue
    }
    fn process_all_messages(&self)
    {
        let mut msg = unsafe { reserved() };
        while unsafe { GetMessageW(&mut msg, null_mut(), 0, 0) > 0 }
        {
            unsafe { TranslateMessage(&mut msg); DispatchMessageW(&mut msg); }
        }
    }
}
impl NativeWindow
{
    unsafe extern "system" fn wndproc(hwnd: HWND, msg: UINT, wp: WPARAM, lp: LPARAM) -> LRESULT
    {
        match msg
        {
            WM_DESTROY => { PostQuitMessage(0); DefWindowProcW(hwnd, msg, wp, lp) },
            WM_INPUT => { info!(target: "interlude::win32::window", "WM_INPUT arrived."); DefWindowProcW(hwnd, msg, wp, lp) },
            _ => DefWindowProcW(hwnd, msg, wp, lp)
        }
    }
}

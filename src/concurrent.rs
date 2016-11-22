// Concurrency Entities

use std;
use std::sync::Arc;
use super::internals::*;
#[cfg(windows)] use winapi::*;
#[cfg(windows)] use kernel32::*;

/// An event object tells the signal to other threads
#[cfg(windows)] pub struct Event
{
	handle: HANDLE
}
unsafe impl Sync for Event {}
unsafe impl Send for Event {}
#[cfg(windows)] impl Event
{
	pub fn new<Str: Into<Vec<u8>>>(name: Str) -> Result<Arc<Self>, EngineError>
	{
		let evname = std::ffi::CString::new(name).unwrap();
		let ev = unsafe { CreateEventA(std::ptr::null_mut(), FALSE, FALSE, evname.as_ptr()) };
		if ev.is_null() { Err(EngineError::from(std::io::Error::last_os_error())) }
		else { Ok(Arc::new(Event { handle: ev })) }
	}
	pub fn set(&self) { unsafe { SetEvent(self.handle); } }
	pub fn reset(&self) { unsafe { ResetEvent(self.handle); } }
	pub fn get_internal(&self) -> HANDLE { self.handle }
}
#[cfg(windows)] impl Drop for Event
{
	fn drop(&mut self) { unsafe { CloseHandle(self.handle); } }
}

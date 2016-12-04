// Concurrency Entities

use std;
use std::sync::Arc;
use super::internals::*;
#[cfg(windows)] use winapi::*;
#[cfg(windows)] use kernel32::*;
#[cfg(unix)] use libc;
#[cfg(unix)] use std::os::unix::io::*;

/// An event object tells the signal to other threads
#[cfg(windows)] pub struct Event
{
	handle: HANDLE
}
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

/// An event object tells the signal to other threads
#[cfg(unix)] pub struct Event(RawFd);
#[cfg(unix)] impl AsRawFd for Event { fn as_raw_fd(&self) -> RawFd { self.0 } }
#[cfg(unix)] impl Event
{
	pub fn new<Str: Into<Vec<u8>>>(_: Str) -> Result<Arc<Self>, EngineError>
	{
		Ok(Arc::new(Event(unsafe { libc::eventfd(0, 0) })))
	}
	pub fn set(&self)
	{
		let value: u64 = 1;
		unsafe { libc::write(self.0, std::mem::transmute(&value), 8) };
	}
	pub fn reset(&self)
	{
		let mut value_store: u64 = 0;
		unsafe { libc::read(self.0, std::mem::transmute(&mut value_store), 8) };
	}
	pub fn get_internal(&self) -> RawFd { self.0 }
}

// Common Implements
impl Drop for Event
{
	#[cfg(windows)] fn drop(&mut self) { unsafe { CloseHandle(self.handle); } }
	#[cfg(unix)]    fn drop(&mut self) { unsafe { libc::close(self.0) }; }
}
unsafe impl Sync for Event {}
unsafe impl Send for Event {}

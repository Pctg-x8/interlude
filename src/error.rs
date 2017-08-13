// Prelude: Error Enums and Crash Handling

use std;
use interlude_vk_defs::VkResult;
use std::os::raw::*;
use freetype_sys::*;

pub enum EngineError
{
	DeviceError(VkResult), IOError(std::io::Error),
	XServerError(c_int), FreeTypeError(FT_Error),
	GenericError(&'static str), Win32ErrorWith(&'static str, std::io::Error),
	NullError(std::ffi::NulError),
	// Specific Errors //
	AllocateMemoryWithEmptyResources
}
impl std::convert::From<VkResult> for EngineError
{
	fn from(res: VkResult) -> EngineError { EngineError::DeviceError(res) }
}
impl std::convert::From<std::io::Error> for EngineError
{
	fn from(ie: std::io::Error) -> EngineError { EngineError::IOError(ie) }
}
impl From<std::ffi::NulError> for EngineError
{
	fn from(fnl: std::ffi::NulError) -> EngineError { EngineError::NullError(fnl) }
}
impl std::fmt::Debug for EngineError
{
	fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error>
	{
		match self
		{
			&EngineError::DeviceError(ref r) => write!(formatter, "DeviceError: {:?}", r),
			&EngineError::IOError(ref e) => write!(formatter, "IOError: {:?}", e),
			&EngineError::Win32ErrorWith(ref s, ref e) => write!(formatter, "{}: {:?}", s, e),
			&EngineError::XServerError(ref c) => write!(formatter, "XServerError: {:?}", c),
			&EngineError::FreeTypeError(ref f) => write!(formatter, "FreeTypeError: {:?}", f),
			&EngineError::GenericError(ref e) => write!(formatter, "GenericError: {}", e),
			&EngineError::NullError(ref n) => write!(formatter, "NulError: {:?}", n),
			&EngineError::AllocateMemoryWithEmptyResources => write!(formatter, "GenericError: Attempting to allocate device memory with empty resources")
		}
	}
}
pub fn crash(err: EngineError) -> !
{
	error!(target: "Interlude", "Engine crashed!: {:?}", err);
	panic!("Application has exited due to {}", match err
	{
		EngineError::DeviceError(_) => "Device Error",
		EngineError::IOError(_) => "Input/Output Error",
		EngineError::Win32ErrorWith(_, _) => "Win32Error",
		EngineError::XServerError(_) => "XServer Communication Error",
		EngineError::FreeTypeError(_) => "FreeType Internal Error",
		EngineError::NullError(_) => "Internal Error",
		EngineError::GenericError(_) | EngineError::AllocateMemoryWithEmptyResources => "Generic Error"
	})
}

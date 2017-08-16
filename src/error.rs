// Prelude: Error Enums and Crash Handling

use std;
use interlude_vk_defs::*;
use std::os::raw::*;
use freetype_sys::*;
use std::borrow::Cow;

pub enum EngineError
{
	DeviceError(VkResult), IOError(std::io::Error),
	XServerError(c_int), FreeTypeError(FT_Error),
	GenericError(&'static str), Win32ErrorWith(&'static str, std::io::Error),
	NullError(std::ffi::NulError), Utf8Error(std::str::Utf8Error),
	// Specific Errors //
	AllocateMemoryWithEmptyResources, InvalidFormatCombination
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
impl From<std::str::Utf8Error> for EngineError
{
	fn from(u8e: std::str::Utf8Error) -> EngineError { EngineError::Utf8Error(u8e) }
}
impl std::fmt::Debug for EngineError
{
	fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error>
	{
		match self
		{
			&EngineError::DeviceError(ref r) => write!(formatter, "DeviceError: {}", scan_vkresult(*r)),
			&EngineError::IOError(ref e) => write!(formatter, "IOError: {:?}", e),
			&EngineError::Win32ErrorWith(ref s, ref e) => write!(formatter, "{}: {:?}", s, e),
			&EngineError::XServerError(ref c) => write!(formatter, "XServerError: {:?}", c),
			&EngineError::FreeTypeError(ref f) => write!(formatter, "FreeTypeError: {:?}", f),
			&EngineError::GenericError(ref e) => write!(formatter, "GenericError: {}", e),
			&EngineError::NullError(ref n) => write!(formatter, "NulError: {:?}", n),
			&EngineError::Utf8Error(ref e) => write!(formatter, "Utf8Error: {:?}", e),
			&EngineError::AllocateMemoryWithEmptyResources => write!(formatter, "GenericError: Attempting to allocate device memory with empty resources"),
			&EngineError::InvalidFormatCombination => write!(formatter, "GenericError: Invalid format combination")
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
		EngineError::NullError(_) | EngineError::Utf8Error(_) => "Internal Error",
		EngineError::GenericError(_) | EngineError::AllocateMemoryWithEmptyResources | EngineError::InvalidFormatCombination => "Generic Error"
	})
}

fn scan_vkresult(r: VkResult) -> Cow<'static, str>
{
	match r
	{
		VK_SUCCESS => "Success".into(),
		VK_NOT_READY => "Not Ready".into(),
		VK_TIMEOUT => "Timeout".into(),
		VK_EVENT_SET => "Event Set".into(),
		VK_EVENT_RESET => "Event Reset".into(),
		VK_INCOMPLETE => "Incomplete".into(),
		VK_ERROR_OUT_OF_HOST_MEMORY => "Out of Host Memory".into(),
		VK_ERROR_OUT_OF_DEVICE_MEMORY => "Out of Device Memory".into(),
		VK_ERROR_INITIALIZATION_FAILED => "Initialization Failed".into(),
		VK_ERROR_DEVICE_LOST => "Device Lost".into(),
		VK_ERROR_MEMORY_MAP_FAILED => "Memory Map Failed".into(),
		VK_ERROR_LAYER_NOT_PRESENT => "Layer not Present".into(),
		VK_ERROR_EXTENSION_NOT_PRESENT => "Extension not Present".into(),
		VK_ERROR_FEATURE_NOT_PRESENT => "Feature not Present".into(),
		VK_ERROR_INCOMPATIBLE_DRIVER => "Incompatible Driver".into(),
		VK_ERROR_TOO_MANY_OBJECTS => "Too many Objects".into(),
		VK_ERROR_FORMAT_NOT_SUPPORTED => "Format not Supported".into(),
		VK_ERROR_FRAGMENT_POOL => "Fragment Pool".into(),
		VK_ERROR_SURFACE_LOST_KHR => "Surface Lost".into(),
		VK_ERROR_NATIVE_WINDOW_IN_USE_KHR => "Native Window in use".into(),
		VK_SUBOPTIMAL_KHR => "Suboptimal".into(),
		VK_ERROR_OUT_OF_DATE_KHR => "Out of Date".into(),
		VK_ERROR_INCOMPATIBLE_DISPLAY_KHR => "Incompatible Display".into(),
		VK_ERROR_VALIDATION_FAILED_EXT => "Validation Failed".into(),
		VK_ERROR_INVALID_SHADER_NV => "Invalid Shader".into(),
		VK_ERROR_OUT_OF_POOL_MEMORY_KHR => "Out of Pool Memory".into(),
		VK_ERROR_INVALID_EXTERNAL_HANDLE_KHR => "Invalid External Handle".into(),
		_ => format!("Unknown Error: {}", r).into()
	}
}

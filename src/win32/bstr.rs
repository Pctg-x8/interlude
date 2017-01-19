
//! Win32(COM) bstr wrap

use widestring;
use std;
use winapi::*;

#[link(name = "OleAut32")]
extern "system"
{
	fn SysAllocString(psz: *const OLECHAR) -> BSTR;
	fn SysFreeString(bstrString: BSTR);
}

pub struct BStr(BSTR);
impl Drop for BStr { fn drop(&mut self) { unsafe { SysFreeString(self.0); } } }
impl std::ops::Deref for BStr { type Target = BSTR; fn deref(&self) -> &BSTR { &self.0 } }
impl BStr
{
	pub fn alloc(src: &str) -> Result<Self, Box<std::error::Error>>
	{
		std::ffi::CString::new(src).map_err(std::convert::From::from).and_then(|cs| unsafe
		{
			let bs = SysAllocString(cs.as_ptr() as *const OLECHAR);
			if bs.is_null() { Err(std::convert::From::from(std::io::Error::last_os_error())) } else { Ok(BStr(bs)) }
		})
	}
	pub fn owned(p: BSTR) -> Self { BStr(p) }
}
impl ToString for BStr
{
	fn to_string(&self) -> String
	{
		let cw = unsafe { widestring::WideCStr::from_ptr_str(self.0) };
		cw.to_string().unwrap()
	}
}

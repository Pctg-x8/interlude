// libudev

#![allow(dead_code)]

mod ffi
{
/***
  Origianl file(libudev.h) is part of systemd.

  Copyright 2008-2012 Kay Sievers <kay@vrfy.org>

  systemd is free software; you can redistribute it and/or modify it
  under the terms of the GNU Lesser General Public License as published by
  the Free Software Foundation; either version 2.1 of the License, or
  (at your option) any later version.

  systemd is distributed in the hope that it will be useful, but
  WITHOUT ANY WARRANTY; without even the implied warranty of
  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
  Lesser General Public License for more details.

  You should have received a copy of the GNU Lesser General Public License
  along with systemd; If not, see <http://www.gnu.org/licenses/>.
***/
	#![allow(non_camel_case_types)]
	use libc::*;

	pub enum udev {}
	pub enum udev_list_entry {}
	pub enum udev_device {}
	pub enum udev_monitor {}
	pub enum udev_enumerate {}
	
	#[link(name = "udev")]
	extern "C"
	{
		pub fn udev_new() -> *mut udev;
		pub fn udev_unref(udev: *mut udev) -> *mut udev;

		pub fn udev_list_entry_get_next(entry: *mut udev_list_entry) -> *mut udev_list_entry;
		pub fn udev_list_entry_get_name(entry: *mut udev_list_entry) -> *const c_char;
		pub fn udev_list_entry_get_value(entry: *mut udev_list_entry) -> *const c_char;

		pub fn udev_device_new_from_syspath(udev: *mut udev, syspath: *const c_char) -> *mut udev_device;
		pub fn udev_device_unref(dev: *mut udev_device) -> *mut udev_device;
		pub fn udev_device_get_parent(dev: *mut udev_device) -> *mut udev_device;
		pub fn udev_device_get_devtype(dev: *mut udev_device) -> *const c_char;
		pub fn udev_device_get_devnode(dev: *mut udev_device) -> *const c_char;
		pub fn udev_device_get_devpath(dev: *mut udev_device) -> *const c_char;
		pub fn udev_device_get_is_initialized(dev: *mut udev_device) -> c_int;
		pub fn udev_device_get_properties_list_entry(dev: *mut udev_device) -> *mut udev_list_entry;
		pub fn udev_device_get_sysattr_list_entry(dev: *mut udev_device) -> *mut udev_list_entry;
		pub fn udev_device_get_property_value(dev: *mut udev_device, name: *const c_char) -> *const c_char;
		pub fn udev_device_get_sysattr_value(dev: *mut udev_device, name: *const c_char) -> *const c_char;
		pub fn udev_device_get_action(dev: *mut udev_device) -> *const c_char;

		pub fn udev_monitor_new_from_netlink(udev: *mut udev, name: *const c_char) -> *mut udev_monitor;
		pub fn udev_monitor_unref(monitor: *mut udev_monitor) -> *mut udev_monitor;
		pub fn udev_monitor_enable_receiving(monitor: *mut udev_monitor) -> c_int;
		pub fn udev_monitor_get_fd(monitor: *mut udev_monitor) -> c_int;
		pub fn udev_monitor_receive_device(monitor: *mut udev_monitor) -> *mut udev_device;
		pub fn udev_monitor_filter_add_match_subsystem_devtype(monitor: *mut udev_monitor, subsystem: *const c_char, devtype: *const c_char) -> c_int;

		pub fn udev_enumerate_new(udev: *mut udev) -> *mut udev_enumerate;
		pub fn udev_enumerate_unref(enumerate: *mut udev_enumerate) -> *mut udev_enumerate;
		pub fn udev_enumerate_add_match_subsystem(enumerate: *mut udev_enumerate, subsystem: *const c_char) -> c_int;
		pub fn udev_enumerate_scan_devices(enumerate: *mut udev_enumerate) -> c_int;
		pub fn udev_enumerate_get_list_entry(enumerate: *mut udev_enumerate) -> *mut udev_list_entry;
	}
}

use {std, libc};
use std::ffi::{CString, CStr};

// safety objects
pub struct UserspaceDeviceManager
{
	internal: *mut ffi::udev
}
impl UserspaceDeviceManager
{
	pub fn new() -> Result<Self, &'static str>
	{
		let udev_ptr = unsafe { ffi::udev_new() };
		if udev_ptr.is_null() { Err("Unable to create udev object") }
		else
		{
			Ok(UserspaceDeviceManager { internal: udev_ptr })
		}
	}
	pub fn new_monitor(&self) -> Result<UserspaceDeviceMonitor, &'static str>
	{
		let mon_ptr = unsafe { ffi::udev_monitor_new_from_netlink(self.internal, CString::new("udev").unwrap().as_ptr()) };
		if mon_ptr.is_null() { Err("Unable to create udev monitor") }
		else
		{
			Ok(UserspaceDeviceMonitor { internal: mon_ptr })
		}
	}
	pub fn new_enumerator(&self) -> Result<UserspaceDeviceEnumerator, &'static str>
	{
		let e_ptr = unsafe { ffi::udev_enumerate_new(self.internal) };
		if e_ptr.is_null() { Err("Unable to create udev enumerator") }
		else
		{
			Ok(UserspaceDeviceEnumerator { internal: e_ptr })
		}
	}
	pub fn new_device_from_syspath(&self, path: &str) -> UserspaceDevice
	{
		UserspaceDevice
		{
			internal: unsafe { ffi::udev_device_new_from_syspath(self.internal, CString::new(path).unwrap().as_ptr()) }
		}
	}
}
impl std::ops::Drop for UserspaceDeviceManager
{
	fn drop(&mut self)
	{
		unsafe { ffi::udev_unref(self.internal) };
	}
}

pub struct UserspaceDeviceMonitor
{
	internal: *mut ffi::udev_monitor
}
impl UserspaceDeviceMonitor
{
	pub fn enable_receiving(self) -> Self
	{
		let res = unsafe { ffi::udev_monitor_enable_receiving(self.internal) };
		if res != 0 { error!(target: "udev", "Unable to change monitor to receiving state"); }
		self
	}
	pub fn add_filter_subsystem(self, subsystem: &str) -> Self
	{
		let res = unsafe { ffi::udev_monitor_filter_add_match_subsystem_devtype(self.internal,
			CString::new(subsystem).unwrap().as_ptr(), std::ptr::null()) };
		if res != 0 { error!(target: "udev", "Unable to add subsystem filter"); }
		self
	}
	pub fn receive_device(&self) -> Result<UserspaceDevice, &'static str>
	{
		let d = unsafe { ffi::udev_monitor_receive_device(self.internal) };
		if d.is_null()
		{
			error!(target: "udev", "Failed to receive device");
			Err("Failed to receive device")
		}
		else
		{
			Ok(UserspaceDevice { internal: d })
		}
	}
}
impl std::ops::Drop for UserspaceDeviceMonitor
{
	fn drop(&mut self)
	{
		unsafe { ffi::udev_monitor_unref(self.internal) };
	}
}
impl std::os::unix::io::AsRawFd for UserspaceDeviceMonitor
{
	fn as_raw_fd(&self) -> std::os::unix::io::RawFd { unsafe { ffi::udev_monitor_get_fd(self.internal) } }
}

pub struct UserspaceDeviceEnumerator
{
	internal: *mut ffi::udev_enumerate
}
impl UserspaceDeviceEnumerator
{
	pub fn filter_match_subsystem(self, subsystem: &str) -> Self
	{
		unsafe { ffi::udev_enumerate_add_match_subsystem(self.internal, CString::new(subsystem).unwrap().as_ptr()) };
		self
	}
	pub fn get_devices(&self) -> UserspaceDeviceListEntries
	{
		unsafe { ffi::udev_enumerate_scan_devices(self.internal) };
		let ent = unsafe { ffi::udev_enumerate_get_list_entry(self.internal) };
		UserspaceDeviceListEntries { current: ent, ph: std::marker::PhantomData }
	}
}
impl std::ops::Drop for UserspaceDeviceEnumerator
{
	fn drop(&mut self)
	{
		unsafe { ffi::udev_enumerate_unref(self.internal) };
	}
}

pub struct UserspaceDeviceListEntries<'a>
{
	current: *mut ffi::udev_list_entry,
	ph: std::marker::PhantomData<&'a ffi::udev_list_entry>
}
impl <'a> std::iter::Iterator for UserspaceDeviceListEntries<'a>
{
	type Item = UserspaceDeviceListEntry<'a>;
	
	fn next(&mut self) -> Option<Self::Item>
	{
		if self.current.is_null() { None }
		else
		{
			let retval = self.current;
			self.current = unsafe { ffi::udev_list_entry_get_next(self.current) };
			Some(UserspaceDeviceListEntry
			{
				internal: retval, ph: std::marker::PhantomData
			})
		}
	}
}

unsafe fn return_cstr<'a>(inp: *const libc::c_char) -> Option<&'a CStr>
{
	if inp.is_null() { None } else { Some(CStr::from_ptr(inp)) }
}

pub struct UserspaceDeviceListEntry<'a>
{
	internal: *mut ffi::udev_list_entry,
	ph: std::marker::PhantomData<&'a ffi::udev_list_entry>
}
impl <'a> UserspaceDeviceListEntry<'a>
{
	pub fn name(&self) -> Option<&CStr>
	{
		unsafe
		{
			return_cstr(ffi::udev_list_entry_get_name(self.internal))
		}
	}
	pub fn value(&self) -> Option<&CStr>
	{
		unsafe
		{
			return_cstr(ffi::udev_list_entry_get_value(self.internal))
		}
	}
}

pub struct UserspaceDevice
{
	internal: *mut ffi::udev_device
}
impl UserspaceDevice
{
	pub fn parent(&self) -> Option<UserspaceDevice>
	{
		let p = unsafe { ffi::udev_device_get_parent(self.internal) };
		if p.is_null() { None } else { Some(UserspaceDevice { internal: p }) }
	}
	pub fn device_path(&self) -> Option<&CStr>
	{
		unsafe { return_cstr(ffi::udev_device_get_devpath(self.internal)) }
	}
	pub fn device_type(&self) -> Option<&CStr>
	{
		unsafe { return_cstr(ffi::udev_device_get_devtype(self.internal)) }
	}
	pub fn device_node(&self) -> Option<&CStr>
	{
		unsafe { return_cstr(ffi::udev_device_get_devnode(self.internal)) }
	}
	pub fn is_initialized(&self) -> bool
	{
		unsafe { ffi::udev_device_get_is_initialized(self.internal) != 0 }
	}
	pub fn properties(&self) -> UserspaceDeviceListEntries
	{
		let e = unsafe { ffi::udev_device_get_properties_list_entry(self.internal) };
		UserspaceDeviceListEntries { current: e, ph: std::marker::PhantomData }
	}
	pub fn property_value(&self, name: &str) -> Option<&CStr>
	{
		unsafe { return_cstr(ffi::udev_device_get_property_value(self.internal, CString::new(name).unwrap().as_ptr())) }
	}
	pub fn sysattrs(&self) -> UserspaceDeviceListEntries
	{
		let e = unsafe { ffi::udev_device_get_sysattr_list_entry(self.internal) };
		UserspaceDeviceListEntries { current: e, ph: std::marker::PhantomData }
	}
	pub fn sysattr_value(&self, name: &str) -> Option<&CStr>
	{
		unsafe { return_cstr(ffi::udev_device_get_sysattr_value(self.internal, CString::new(name).unwrap().as_ptr())) }
	}
	pub fn action(&self) -> Option<&CStr>
	{
		unsafe { return_cstr(ffi::udev_device_get_action(self.internal)) }
	}
}
impl std::ops::Drop for UserspaceDevice
{
	fn drop(&mut self)
	{
		unsafe { ffi::udev_device_unref(self.internal) };
	}
}

// Win32 Input Module

use std;
use std::thread;
use std::collections::BTreeMap as GenericMap;
use input::*;
use {EngineResult, EngineError};
use winapi::*;
use user32::*;

pub struct NativeInput<InputNames: Eq + Ord + Copy>
{
	keymap: GenericMap<InputNames, Vec<InputType>>,
	aggregate_key_states: GenericMap<InputKeys, u32>,
	aggregate_axis_states: GenericMap<InputAxis, f32>,
	input_states: GenericMap<InputNames, f32>
}
impl<InputNames: Eq + Ord + Copy> NativeInput<InputNames>
{
	pub fn new() -> EngineResult<Self>
	{
		info!(target: "Interlude::Input", "Registering RawInput Devices...");
		let ri_devices = [
			// Mouse
			RAWINPUTDEVICE { usUsagePage: 0x01, usUsage: 0x02, dwFlags: 0, hwndTarget: std::ptr::null_mut() },
			// Keyboard
			RAWINPUTDEVICE { usUsagePage: 0x01, usUsage: 0x06, dwFlags: RIDEV_NOLEGACY | RIDEV_APPKEYS, hwndTarget: std::ptr::null_mut() }
		];
		if unsafe { RegisterRawInputDevices(ri_devices.as_ptr(), ri_devices.len() as u32, std::mem::size_of::<RAWINPUTDEVICE>() as UINT) } == 0
		{
			Err(EngineError::Win32ErrorWith("Failed value returned from RegisterRawInputDevices", std::io::Error::last_os_error()))
		}
		else
		{
			try!(thread::Builder::new().name("Input Thread[Win32::RawInput/XInput]".into()).spawn(move ||
			{
				// No works
			}));
			Ok(NativeInput
			{
				keymap: GenericMap::new(), input_states: GenericMap::new(),
				aggregate_key_states: GenericMap::new(), aggregate_axis_states: GenericMap::new()
			})
		}
	}
	pub fn add_input(&mut self, to: InputNames, from: InputType)
	{
		from.assert_unhandled();
		self.keymap.entry(to).or_insert_with(Vec::new).push(from);
		self.input_states.insert(to, 0.0f32);
	}
	pub fn update(&mut self) {}
}

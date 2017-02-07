// Interlude: Input System

use EngineResult;
use std::hash::Hash;
use std::ops::Index;
#[cfg(windows)] use winapi::*;
#[cfg(windows)] use user32::*;
#[cfg(windows)] use super::win32::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputKeys
{
	/* Keyboard */
	Unhandled,
	Esc, Number(u8), Minus, Equal, Backspace, Tab, Character(char),
	LeftBrace, RightBrace, Enter, Control, Shift, Alt,
	Apostrophe, Grave, Backslash, Comma, Semicolon, Asterisk, Dot, Slash,
	Space, CapsLock, NumLock, ScrollLock, FunctionKey(u8), Plus,
	ZenkakuHankaku, Katakana, Hiragana, Henkan, KatakanaHiragana, Muhenkan, SysRq,
	Home, End, PageUp, PageDown, Up, Left, Right, Down, Insert, Delete,
	/* ButtonJoystick/Gamepad */
	ButtonTrigger, ButtonThumb(u8), ButtonTop(u8), ButtonPinkie, ButtonBase(u8), ButtonDead,
	ButtonA, ButtonB, ButtonC, ButtonX, ButtonY, ButtonZ, ButtonTrigLeft, ButtonTrigRight, ButtonTrigLeft2, ButtonTrigRight2,
	ButtonSelect, ButtonStart, ButtonMode, ButtonThumbL, ButtonThumbR
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputAxis
{
	X, Y, Z, RX, RY, RZ, Hat0x, Hat0y, Hat1x, Hat1y, Unhandled
}

pub enum InputType { Key(InputKeys), Axis(InputAxis), KeyAsAxis(InputKeys, InputKeys) }
impl InputType
{
	pub fn assert_unhandled(&self)
	{
		match self
		{
			&InputType::Key(k) => assert!(k != InputKeys::Unhandled),
			&InputType::Axis(x) => assert!(x != InputAxis::Unhandled),
			&InputType::KeyAsAxis(p, n) => assert!(p != InputKeys::Unhandled && n != InputKeys::Unhandled)
		}
	}
}
pub trait InputSystem<InputNames: PartialEq + Eq + Hash + Copy + Clone> : Sized + Index<InputNames, Output = f32>
{
	fn new() -> EngineResult<Self>;
	fn add_input(&mut self, to: InputNames, from: InputType);
	fn update(&mut self);
}

#[cfg(windows)] pub struct Win32InputSystem<InputNames: PartialEq + Eq + Hash + Copy + Clone>
{
	keymap: HashMap<InputNames, Vec<InputType>>,
	aggregate_key_states: HashMap<InputKeys, u32>,
	aggregate_axis_states: HashMap<InputAxis, f32>,
	input_states: HashMap<InputNames, f32>
}
#[cfg(windows)] impl<InputNames: PartialEq + Eq + Clone + Copy + std::hash::Hash> InputSystem<InputNames> for Win32InputSystem<InputNames>
{
	fn new() -> EngineResult<Self>
	{
		info!(target: "Interlude::Input", "Registering RawInput Devices...");
		let ri_devices = [
			// Mouse //
			RAWINPUTDEVICE { usUsagePage: 0x01, usUsage: 0x02, dwFlags: 0, hwndTarget: std::ptr::null_mut() },
			// Keyboard //
			RAWINPUTDEVICE { usUsagePage: 0x01, usUsage: 0x06, dwFlags: RIDEV_NOLEGACY | RIDEV_APPKEYS, hwndTarget: std::ptr::null_mut() }
		];
		if unsafe { RegisterRawInputDevices(ri_devices.as_ptr(), ri_devices.len() as u32, std::mem::size_of::<RAWINPUTDEVICE>() as UINT) } == 0
		{
			Err(EngineError::Win32ErrorWith("Failed value returned from RegisterrawInputDevices", std::io::Error::last_os_error()))
		}
		else
		{
			try!(std::thread::Builder::new().name("Input Thread[Win32/Xinput]".to_owned()).spawn(move ||
			{
				// No Works
			}));
			Ok(Win32InputSystem
			{
				keymap: HashMap::new(), input_states: HashMap::new(),
				aggregate_key_states: HashMap::new(), aggregate_axis_states: HashMap::new()
			})
		}
	}
	fn add_input(&mut self, to: InputNames, from: InputType)
	{
		from.assert_unhandled();
		self.keymap.entry(to).or_insert(Vec::new()).push(from);
		self.input_states.insert(to, 0.0f32);
	}
	fn update(&mut self)
	{

	}
}
#[cfg(windows)] impl<InputNames: PartialEq + Eq + Clone + Copy + std::hash::Hash> Index<InputNames> for Win32InputSystem<InputNames>
{
	type Output = f32;
	fn index(&self, name: InputNames) -> &f32
	{
		static DEFAULT_F32: f32 = 0.0;
		self.input_states.get(&name).unwrap_or(&DEFAULT_F32)
	}
}

// Interlude: Input System

use {std, epoll};
use super::internals::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use super::linux::evdev::*;
use super::linux::udev::*;
use std::os::unix::io::{RawFd, AsRawFd};
use std::hash::Hash;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputAxis
{
	X, Y, Z, RX, RY, RZ, Hat0x, Hat0y, Hat1x, Hat1y, Unhandled
}
impl std::convert::From<KeyEvents> for InputKeys
{
	fn from(x: KeyEvents) -> Self
	{
		match x
		{
			/* Nums */
			KeyEvents::Num1 | KeyEvents::KP1 => InputKeys::Number(1),
			KeyEvents::Num2 | KeyEvents::KP2 => InputKeys::Number(2),
			KeyEvents::Num3 | KeyEvents::KP3 => InputKeys::Number(3),
			KeyEvents::Num4 | KeyEvents::KP4 => InputKeys::Number(4),
			KeyEvents::Num5 | KeyEvents::KP5 => InputKeys::Number(5),
			KeyEvents::Num6 | KeyEvents::KP6 => InputKeys::Number(6),
			KeyEvents::Num7 | KeyEvents::KP7 => InputKeys::Number(7),
			KeyEvents::Num8 | KeyEvents::KP8 => InputKeys::Number(8),
			KeyEvents::Num9 | KeyEvents::KP9 => InputKeys::Number(9),
			KeyEvents::Num0 | KeyEvents::KP0 => InputKeys::Number(0),
			KeyEvents::F1 => InputKeys::FunctionKey(1),
			KeyEvents::F2 => InputKeys::FunctionKey(2),
			KeyEvents::F3 => InputKeys::FunctionKey(3),
			KeyEvents::F4 => InputKeys::FunctionKey(4),
			KeyEvents::F5 => InputKeys::FunctionKey(5),
			KeyEvents::F6 => InputKeys::FunctionKey(6),
			KeyEvents::F7 => InputKeys::FunctionKey(7),
			KeyEvents::F8 => InputKeys::FunctionKey(8),
			KeyEvents::F9 => InputKeys::FunctionKey(9),
			KeyEvents::F10 => InputKeys::FunctionKey(10),
			KeyEvents::F11 => InputKeys::FunctionKey(11),
			KeyEvents::F12 => InputKeys::FunctionKey(12),
			/* Characters */
			KeyEvents::A => InputKeys::Character('a'),
			KeyEvents::B => InputKeys::Character('b'),
			KeyEvents::C => InputKeys::Character('c'),
			KeyEvents::D => InputKeys::Character('d'),
			KeyEvents::E => InputKeys::Character('e'),
			KeyEvents::F => InputKeys::Character('f'),
			KeyEvents::G => InputKeys::Character('g'),
			KeyEvents::H => InputKeys::Character('h'),
			KeyEvents::I => InputKeys::Character('i'),
			KeyEvents::J => InputKeys::Character('j'),
			KeyEvents::K => InputKeys::Character('k'),
			KeyEvents::L => InputKeys::Character('l'),
			KeyEvents::M => InputKeys::Character('m'),
			KeyEvents::N => InputKeys::Character('n'),
			KeyEvents::O => InputKeys::Character('o'),
			KeyEvents::P => InputKeys::Character('p'),
			KeyEvents::Q => InputKeys::Character('q'),
			KeyEvents::R => InputKeys::Character('r'),
			KeyEvents::S => InputKeys::Character('s'),
			KeyEvents::T => InputKeys::Character('t'),
			KeyEvents::U => InputKeys::Character('u'),
			KeyEvents::V => InputKeys::Character('v'),
			KeyEvents::W => InputKeys::Character('w'),
			KeyEvents::X => InputKeys::Character('x'),
			KeyEvents::Y => InputKeys::Character('y'),
			KeyEvents::Z => InputKeys::Character('z'),
			/* Keyboard Misc */
			KeyEvents::Esc => InputKeys::Esc,
			KeyEvents::Minus | KeyEvents::KPMinus => InputKeys::Minus,
			KeyEvents::KPPlus => InputKeys::Plus,
			KeyEvents::LeftCtrl | KeyEvents::RightCtrl => InputKeys::Control,
			KeyEvents::LeftAlt | KeyEvents::RightAlt => InputKeys::Alt,
			KeyEvents::LeftShift | KeyEvents::RightShift => InputKeys::Shift,
			KeyEvents::Equal | KeyEvents::KPEqual => InputKeys::Equal,
			KeyEvents::Enter | KeyEvents::KPEnter => InputKeys::Enter,
			KeyEvents::Backspace => InputKeys::Backspace,
			KeyEvents::Tab => InputKeys::Tab,
			KeyEvents::LeftBrace => InputKeys::LeftBrace,
			KeyEvents::RightBrace => InputKeys::RightBrace,
			KeyEvents::Semicolon => InputKeys::Semicolon,
			KeyEvents::Apostrophe => InputKeys::Apostrophe,
			KeyEvents::Grave => InputKeys::Grave,
			KeyEvents::Backslash => InputKeys::Backslash,
			KeyEvents::Comma | KeyEvents::KPJPComma => InputKeys::Comma,
			KeyEvents::Dot | KeyEvents::KPDot => InputKeys::Dot,
			KeyEvents::Slash | KeyEvents::KPSlash => InputKeys::Slash,
			KeyEvents::KPAsterisk => InputKeys::Asterisk,
			KeyEvents::Space => InputKeys::Space,
			KeyEvents::CapsLock => InputKeys::CapsLock,
			KeyEvents::NumLock => InputKeys::NumLock,
			KeyEvents::ScrollLock => InputKeys::ScrollLock,
			KeyEvents::ZenkakuHankaku => InputKeys::ZenkakuHankaku,
			KeyEvents::Katakana => InputKeys::Katakana,
			KeyEvents::Hiragana => InputKeys::Hiragana,
			KeyEvents::Henkan => InputKeys::Henkan,
			KeyEvents::KatakanaHiragana => InputKeys::KatakanaHiragana,
			KeyEvents::Muhenkan => InputKeys::Muhenkan,
			KeyEvents::SysRQ => InputKeys::SysRq,
			KeyEvents::Home => InputKeys::Home,
			KeyEvents::End => InputKeys::End,
			KeyEvents::PageUp => InputKeys::PageUp,
			KeyEvents::PageDown => InputKeys::PageDown,
			KeyEvents::Up => InputKeys::Up,
			KeyEvents::Left => InputKeys::Left,
			KeyEvents::Right => InputKeys::Right,
			KeyEvents::Down => InputKeys::Down,
			KeyEvents::Insert => InputKeys::Insert,
			KeyEvents::Delete => InputKeys::Delete,
			/* ButtonJoystick/Gamepad */
			KeyEvents::ButtonTrigger => InputKeys::ButtonTrigger,
			KeyEvents::ButtonThumb => InputKeys::ButtonThumb(1),
			KeyEvents::ButtonThumb2 => InputKeys::ButtonThumb(2),
			KeyEvents::ButtonTop => InputKeys::ButtonTop(1),
			KeyEvents::ButtonTop2 => InputKeys::ButtonTop(2),
			KeyEvents::ButtonPinkie => InputKeys::ButtonPinkie,
			KeyEvents::ButtonBase => InputKeys::ButtonBase(1),
			KeyEvents::ButtonBase2 => InputKeys::ButtonBase(2),
			KeyEvents::ButtonBase3 => InputKeys::ButtonBase(3),
			KeyEvents::ButtonBase4 => InputKeys::ButtonBase(4),
			KeyEvents::ButtonBase5 => InputKeys::ButtonBase(5),
			KeyEvents::ButtonBase6 => InputKeys::ButtonBase(6),
			KeyEvents::ButtonDead => InputKeys::ButtonDead,
			KeyEvents::ButtonA => InputKeys::ButtonA,
			KeyEvents::ButtonB => InputKeys::ButtonB,
			KeyEvents::ButtonC => InputKeys::ButtonC,
			KeyEvents::ButtonX => InputKeys::ButtonX,
			KeyEvents::ButtonY => InputKeys::ButtonY,
			KeyEvents::ButtonZ => InputKeys::ButtonZ,
			KeyEvents::ButtonTrigLeft => InputKeys::ButtonTrigLeft,
			KeyEvents::ButtonTrigRight => InputKeys::ButtonTrigRight,
			KeyEvents::ButtonTrigLeft2 => InputKeys::ButtonTrigLeft2,
			KeyEvents::ButtonTrigRight2 => InputKeys::ButtonTrigRight2,
			KeyEvents::ButtonSelect => InputKeys::ButtonSelect,
			KeyEvents::ButtonStart => InputKeys::ButtonStart,
			KeyEvents::ButtonMode => InputKeys::ButtonMode,
			KeyEvents::ButtonThumbL => InputKeys::ButtonThumbL,
			KeyEvents::ButtonThumbR => InputKeys::ButtonThumbR,
			_ => InputKeys::Unhandled
		}
	}
}
impl std::convert::From<AbsoluteAxisEvents> for InputAxis
{
	fn from(x: AbsoluteAxisEvents) -> Self
	{
		match x
		{
			AbsoluteAxisEvents::X => InputAxis::X,
			AbsoluteAxisEvents::Y => InputAxis::Y,
			AbsoluteAxisEvents::Z => InputAxis::Z,
			AbsoluteAxisEvents::RX => InputAxis::RX,
			AbsoluteAxisEvents::RY => InputAxis::RY,
			AbsoluteAxisEvents::RZ => InputAxis::RZ,
			AbsoluteAxisEvents::Hat0x => InputAxis::Hat0x,
			AbsoluteAxisEvents::Hat0y => InputAxis::Hat0y,
			AbsoluteAxisEvents::Hat1x => InputAxis::Hat1x,
			AbsoluteAxisEvents::Hat1y => InputAxis::Hat1y,
			_ => InputAxis::Unhandled
		}
	}
}

type AsyncExclusiveHashMap<K, V> = Arc<Mutex<HashMap<K, V>>>;
pub enum InputType
{
	Key(InputKeys), Axis(InputAxis), KeyAsAxis(InputKeys, InputKeys)
}
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
pub struct InputDevice
{
	dev: EventDevice,
	key_states: HashMap<InputKeys, bool>,
	axis_prev_values: HashMap<InputAxis, f32>
}
impl InputDevice
{
	pub fn new(node_path: &str) -> Result<InputDevice, EngineError>
	{
		EventDevice::new(node_path).map(|ev| InputDevice
		{
			dev: ev,
			key_states: HashMap::new(),
			axis_prev_values: HashMap::new()
		})
	}
	pub fn update(&mut self, aggregate_key_states: &mut HashMap<InputKeys, u32>, aggregate_axis_states: &mut HashMap<InputAxis, f32>)
	{
		while let Ok(ev) = self.dev.wait_event()
		{
			match ev
			{
				DeviceEvent::Syn(_, _) => break,
				DeviceEvent::Key(_, k, p) => match p
				{
					PressedState::Released =>
					{
						*self.key_states.entry(k.into()).or_insert(false) = false;
						*aggregate_key_states.entry(k.into()).or_insert(1) -= 1;
					},
					PressedState::Pressed =>
					{
						*self.key_states.entry(k.into()).or_insert(true) = true;
						*aggregate_key_states.entry(k.into()).or_insert(0) += 1;
					},
					PressedState::Repeating => ()
				},
				DeviceEvent::Absolute(_, x, v) =>
				{
					let xe = x.into();
					let old_value = *self.axis_prev_values.entry(xe).or_insert(0.0f32);
					*aggregate_axis_states.entry(xe).or_insert(0.0f32) -= old_value;
					*aggregate_axis_states.entry(xe).or_insert(0.0f32) += v;
					*self.axis_prev_values.entry(xe).or_insert(0.0f32) = v;
				},
				_ => ()
			}
		}
	}
	pub fn unplug(self, aggregate_key_states: &mut HashMap<InputKeys, u32>, aggregate_axis_states: &mut HashMap<InputAxis, f32>)
	{
		for (k, v) in self.key_states
		{
			if v { *aggregate_key_states.entry(k).or_insert(1) -= 1; }
		}
		for (x, v) in self.axis_prev_values
		{
			*aggregate_axis_states.entry(x).or_insert(v) -= v;
		}
	}
}
impl AsRawFd for InputDevice
{
	fn as_raw_fd(&self) -> RawFd { self.dev.as_raw_fd() }
}
pub struct InputSystem<InputNames: PartialEq + Eq + Hash + Copy + Clone + Debug>
{
	keymap: HashMap<InputNames, Vec<InputType>>,
	aggregate_key_states: AsyncExclusiveHashMap<InputKeys, u32>,
	aggregate_axis_states: AsyncExclusiveHashMap<InputAxis, f32>,
	input_states: HashMap<InputNames, f32>
}
impl <InputNames: PartialEq + Eq + Hash + Copy + Clone + Debug> InputSystem<InputNames>
{
	fn search_device_name(device: &UserspaceDevice) -> String
	{
		// search device name ascending parent
		fn recursive(device: &UserspaceDevice) -> String
		{
			device.property_value("NAME").map(|x| x.to_str().unwrap().to_owned()).unwrap_or_else(|| device.parent().as_ref().map(recursive).unwrap_or("Unknown Device".to_owned()))
		}
		recursive(device)
	}
	fn insert_device(input_devices: &mut HashMap<u32, InputDevice>, polling: &mut epoll::EpollInstance, device: UserspaceDevice)
	{
		let name = Self::search_device_name(&device);
		let node_path = device.device_node().expect("Unable to get Device Node Path").to_str().unwrap();
		let node_number: u32 = node_path["/dev/input/event".len()..].parse().unwrap();
		let joystick_device = device.property_value("ID_INPUT_JOYSTICK").and_then(|f| f.to_str().ok()).map(|n| n == "1").unwrap_or(false);
		let keyboard_device = device.property_value("ID_INPUT_KEYBOARD").and_then(|f| f.to_str().ok()).map(|n| n == "1").unwrap_or(false);
		if joystick_device || keyboard_device
		{
			info!(target: "Interlude::Input", "Initializing for {} Input: {} [{}]", if joystick_device { "Joystick" } else { "Keyboard" }, name, node_path);
			let idev = InputDevice::new(node_path).unwrap();
			polling.add_interest(epoll::Interest::new(idev.as_raw_fd(), epoll::EPOLLIN, node_number as u64)).unwrap();
			input_devices.insert(node_number, idev);
		}
	}
	pub fn new() -> Result<Self, EngineError>
	{
		let aks = Arc::new(Mutex::new(HashMap::new()));
		let aas = Arc::new(Mutex::new(HashMap::new()));
		let aks_thread = aks.clone();
		let aas_thread = aas.clone();

		try!(std::thread::Builder::new().name("Input Thread".into()).spawn(move ||
		{
			let mut input_devices = HashMap::new();
			info!(target: "Prelude::Input", "Starting udev...");
			let udev = UserspaceDeviceManager::new().unwrap();
			let mut polling = epoll::EpollInstance::new().expect("Unable to create polling object");
			
			info!(target: "Prelude::Input", "Listing Event Devices...");
			let enumerator = udev.new_enumerator().unwrap().filter_match_subsystem("input");
			let event_devices = enumerator.get_devices().filter(|d| d.name().and_then(|x| x.to_str().ok()).and_then(|d| d.split('/').last())
				.map(|final_name| final_name.starts_with("event")).unwrap_or(false));
			for dev in event_devices.filter_map(|dent| dent.name().and_then(|x| x.to_str().ok()).map(|syspath| udev.new_device_from_syspath(&syspath)))
			{
				// event_device
				Self::insert_device(&mut input_devices, &mut polling, dev);
			}

			let udev_monitor = udev.new_monitor().unwrap().add_filter_subsystem("input").enable_receiving();
			polling.add_interest(epoll::Interest::new(udev_monitor.as_raw_fd(), epoll::EPOLLIN, std::u64::MAX)).unwrap();
			while let Ok(events) = polling.wait(-1, input_devices.len())
			{
				let (udev_events, device_events): (Vec<_>, Vec<_>) = events.into_iter().partition(|e| e.data() == std::u64::MAX);
				// udev monitor
				for dev in udev_events.into_iter().filter_map(|_| udev_monitor.receive_device().ok())
				{
					if dev.device_node().and_then(|d| d.to_str().ok()).map(|d| d.starts_with("/dev/input/event")).unwrap_or(false)
					{
						// event device
						let node_number: u32 = dev.device_node().and_then(|d| d.to_str().ok()).and_then(|s| s["/dev/input/event".len()..].parse().ok()).unwrap_or(std::u32::MAX);
						match dev.action().and_then(|a| a.to_str().ok())
						{
							Some("remove") => if let Some(removed_device) = input_devices.remove(&node_number)
							{
								info!(target: "Prelude::Input", "Removed Device {}", removed_device.dev.name());
								polling.del_interest(&epoll::Interest::new(removed_device.as_raw_fd(), epoll::EPOLLIN, node_number as u64)).unwrap();
								removed_device.unplug(&mut aks_thread.lock().unwrap(), &mut aas_thread.lock().unwrap());
							},
							Some("add") => Self::insert_device(&mut input_devices, &mut polling, dev),
							_ => ()
						}
					}
				}
				// inputs
				{
					let (mut aks, mut aas) = (aks_thread.lock().unwrap(), aas_thread.lock().unwrap());
					for event in device_events
					{
						match input_devices.get_mut(&(event.data() as u32))
						{
							Some(input_device) => input_device.update(&mut aks, &mut aas),
							None =>
							{
								warn!(target: "Prelude::Input", "Input Device is not found?");
							}
						}
					}
				}
			}
		}));

		Ok(InputSystem
		{
			keymap: HashMap::new(), input_states: HashMap::new(),
			aggregate_key_states: aks, aggregate_axis_states: aas
		})
	}
	pub fn add_input(mut self, to: InputNames, from: InputType) -> Self
	{
		from.assert_unhandled();
		self.keymap.entry(to).or_insert(Vec::new()).push(from);
		self.input_states.insert(to, 0.0f32);
		self
	}
	pub fn update(&mut self)
	{
		let (mut key_states, mut axis_states) = (self.aggregate_key_states.lock().unwrap(), self.aggregate_axis_states.lock().unwrap());
		for (t, v) in &self.keymap
		{
			let mut total_value = 0.0f32;
			for f in v
			{
				total_value += match f
				{
					&InputType::Axis(x) => *axis_states.entry(x).or_insert(0.0f32),
					&InputType::Key(k) => if *key_states.entry(k).or_insert(0) > 0 { 1.0f32 } else { 0.0f32 },
					&InputType::KeyAsAxis(n, p) =>
						(if *key_states.entry(p).or_insert(0) > 0 { 1.0f32 } else { 0.0f32 }) -
						(if *key_states.entry(n).or_insert(0) > 0 { 1.0f32 } else { 0.0f32 })
				};
			}
			*self.input_states.entry(*t).or_insert(total_value) = total_value.max(-1.0f32).min(1.0f32);
		}
	}
}
impl <InputNames: PartialEq + Eq + Hash + Copy + Clone + Debug> std::ops::Index<InputNames> for InputSystem<InputNames>
{
	type Output = f32;
	fn index(&self, name: InputNames) -> &f32
	{
		static DEFAULT_F32: f32 = 0.0f32;
		self.input_states.get(&name).unwrap_or(&DEFAULT_F32)
	}
}

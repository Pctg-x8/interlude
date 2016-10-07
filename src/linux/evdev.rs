// evdev binding to Rust

/*
 * Original code: Copyright (c) 1999-2002 Vojtech Pavlik
 *                Copyright (c) 2005 Hans de Goede <hdegoede@redhat.com>
 * This file: Copyright (c) 2016 S.Percentage
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License version 2 as published by
 * the Free Software Foundation.
 */

#![allow(non_camel_case_types)]
#![allow(dead_code)]

use super::super::internals::*;
use {std, libc};
use std::io::Read;
use std::collections::HashMap;
use std::os::unix::io::AsRawFd;

// ioctl macros
const _IOC_NRBITS: u64 = 8;
const _IOC_TYPEBITS: u64 = 8;
const _IOC_SIZEBITS: u64 = 14;
const _IOC_DIRBITS: u64 = 2;
const _IOC_NRSHIFT: u64 = 0;
const _IOC_TYPESHIFT: u64 = _IOC_NRSHIFT + _IOC_NRBITS;
const _IOC_SIZESHIFT: u64 = _IOC_TYPESHIFT + _IOC_TYPEBITS;
const _IOC_DIRSHIFT: u64 = _IOC_SIZESHIFT + _IOC_SIZEBITS;
macro_rules! _IOC
{
	($dir: expr, $_type: expr, $nr: expr, $size: expr) =>
	{
		(($dir as u64) << _IOC_DIRSHIFT) | (($_type as u64) << _IOC_TYPESHIFT) | (($nr as u64) << _IOC_NRSHIFT) | (($size as u64) << _IOC_SIZESHIFT)
	}
}
const _IOC_WRITE: u32 = 1;
const _IOC_READ: u32 = 2;
macro_rules! _IOR
{
	($g: expr, $n: expr, $t: ty) =>
	{
		_IOC!(_IOC_READ, $g, $n, std::mem::size_of::<$t>() as u32)
	}
}
macro_rules! _IOW
{
	($g: expr, $n: expr, $t: ty) =>
	{
		_IOC!(_IOC_WRITE, $g, $n, std::mem::size_of::<$t>() as u32)
	}
}

const BUS_PCI: u16 = 0x01;
const BUS_ISAPNP: u16 = 0x02;
const BUS_USB: u16 = 0x03;
const BUS_HIL: u16 = 0x04;
const BUS_BLUETOOTH: u16 = 0x05;
const BUS_VIRTUAL: u16 = 0x06;

const BUS_ISA: u16 = 0x10;
const BUS_I8042: u16 = 0x11;
const BUS_XTKBD: u16 = 0x12;
const BUS_RS232: u16 = 0x13;
const BUS_GAMEPORT: u16 = 0x14;
const BUS_PARPORT: u16 = 0x15;
const BUS_AMIGA: u16 = 0x16;
const BUS_ADB: u16 = 0x17;
const BUS_I2C: u16 = 0x18;
const BUS_HOST: u16 = 0x19;
const BUS_GSC: u16 = 0x1a;
const BUS_ATARI: u16 = 0x1b;
const BUS_SPI: u16 = 0x1c;
const BUS_RMI: u16 = 0x1d;
const BUS_CEC: u16 = 0x1e;

const SYN_CNT: u64 = 0x0f + 1;
const KEY_CNT: u64 = 0x2ff + 1;
const REL_CNT: u64 = 0x0f + 1;
const ABS_CNT: u64 = 0x3f + 1;
const SW_CNT: u64 = 0x0f + 1;
const MSC_CNT: u64 = 0x07 + 1;
const LED_CNT: u64 = 0x0f + 1;
const REP_CNT: u64 = 0x01 + 1;
const SND_CNT: u64 = 0x07 + 1;
const FF_CNT: u64 = 0x7f + 1;
const FFS_CNT: u64 = 0x01 + 1;

#[repr(C)] #[derive(Clone)]
pub struct input_event
{
	pub time: libc::timeval, pub _type: u16, pub code: u16, pub value: i32
}
const INPUT_EVENT_SIZE: usize = (64 + 64 + 16 + 16 + 32) / 8;
const EV_VERSION: u32 = 0x010001;
#[repr(C)]
struct input_id
{
	bustype: u16, vendor: u16, product: u16, version: u16
}
#[repr(C)]
struct input_absinfo
{
	value: i32, minimum: i32, maximum: i32, fuzz: i32, flat: i32, resolution: i32
}
macro_rules! EVIOCGVERSION
{
	() => { _IOR!('E', 0x01, libc::c_int) }
}
macro_rules! EVIOCGID
{
	() => { _IOR!('E', 0x02, input_id) }
}
macro_rules! EVIOCGNAME
{
	($len: expr) => { _IOC!(_IOC_READ, 'E', 0x06, $len) }
}
macro_rules! EVIOCGBIT
{
	($ev: expr, $len: expr) => { _IOC!(_IOC_READ, 'E', 0x20 + $ev, $len) }
}
macro_rules! EVIOCGABS
{
	($axis: expr) => { _IOR!('E', 0x40 + $axis, input_absinfo) }
}
macro_rules! EVIOCGRAB
{
	() => { _IOW!('E', 0x90, libc::c_int) }
}

// safety wrappers //
#[derive(Debug, Clone)]
pub enum BusType
{
	Unknown, PCI, ISAPnP, USB, HIL, Bluetooth, Virtual,
	ISA, I8042, XTKBD, RS232, Gameport, ParallelPort, Amiga, ADB, I2C, Host,
	GSC, Atari, SPI, RMI, CEC
}
#[repr(C)] #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Event
{
	Syn = 0x00,
	Key = 0x01,
	Relative = 0x02,
	Absolute = 0x03,
	Misc = 0x04,
	Switch = 0x05,
	LED = 0x11,
	Sound = 0x12,
	Repeat = 0x14,
	ForceFeedback = 0x15,
	Power = 0x16,
	ForceFeedbackStatus = 0x17
}
#[repr(C)] #[derive(Debug, Clone, Copy)]
pub enum SynEvents
{
	Report = 0, Config, MultitouchReport, Dropped
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum KeyEvents
{
	Reserved = 0,
	Esc, Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9, Num0,
	Minus, Equal, Backspace, Tab,
	Q, W, E, R, T, Y, U, I, O, P, LeftBrace, RightBrace, Enter, LeftCtrl,
	A, S, D, F, G, H, J, K, L, Semicolon, Apostrophe, Grave, LeftShift, Backslash,
	Z, X, C, V, B, N, M, Comma, Dot, Slash, RightShift, KPAsterisk,
	LeftAlt, Space, CapsLock,
	F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, NumLock, ScrollLock,
	KP7, KP8, KP9, KPMinus, KP4, KP5, KP6, KPPlus, KP1, KP2, KP3, KP0, KPDot,
	ZenkakuHankaku = 85, ND102, F11, F12, RO, Katakana, Hiragana, Henkan,
	KatakanaHiragana, Muhenkan, KPJPComma, KPEnter, RightCtrl, KPSlash, SysRQ,
	RightAlt, LineFeed, Home, Up, PageUp, Left, Right, End, Down, PageDown,
	Insert, Delete, Macro, Mute, VolumeDown, VolumeUp, Power, KPEqual, KPPlusMinus,
	Pause, Scale, KPComma, Hangeul, Hanja, Yen, LeftMeta, RightMeta, Compose,
	Stop, Again, Props, Undo, Front, Copy, Open, Paste, Find, Cut, Help, Menu,
	Calc, Setup, Sleep, Wakeup, File, SendFile, DeleteFile, Xfer, Prog1, Prog2, WWW,
	MSDOS, Coffee, RotateDisplay, CycleWindows, Mail, Bookmarks, Computer,
	Back, Forward, CloseCD, EjectCD, EjectCloseCD, NextSong, PlayPause, PreviousSong,
	StopCD, Record, Rewind, Phone, ISO, Config, Homepage, Refresh, Exit, Move,
	Edit, ScrollUp, ScrollDown, KPLeftParen, KPRightParen, New, Redo,
	F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24,
	PlayCD = 200, PauseCD, Prog3, Prog4, Dashboard, Suspend, Close, Play, Fastforward,
	BassBoost, Print, HP, Camera, Sound, Question, Email, Chat, Search, Connect, Finance,
	Sport, SHop, Alterase, Cancel, BrightnessDown, BrightnessUp, Media,
	SwitchVideoMode, KBDIllumToggle, KBDIllumDown, KBDIllumUp,
	Send, Reply, ForwardMail, Save, Documents, Battery, Bluetooth, WLAN, UWB, Unknown,
	VideoNext, VideoPrev, BrightnessCycle, BrightnessAuto, DisplayOff,
	WWAN, RFKill, MicMute,
	MiscButton = 0x100, Button0, Button1, Button2, Button3, Button4, Button5, Button6, Button7, Button8, Button9,
	/* ButtonMouse */ButtonLeft = 0x110, ButtonRight, ButtonMiddle, ButtonSide, ButtonExtra, ButtonForward, ButtonBack, ButtonTask,
	/* ButtonJoystick */ButtonTrigger = 0x120, ButtonThumb, ButtonThumb2, ButtonTop, ButtonTop2, ButtonPinkie, ButtonBase, ButtonBase2, ButtonBase3, ButtonBase4, ButtonBase5, ButtonBase6, ButtonDead,
	/* ButtonGamepad */ButtonA = 0x130, ButtonB, ButtonC, ButtonX, ButtonY, ButtonZ, ButtonTrigLeft, ButtonTrigRight, ButtonTrigLeft2, ButtonTrigRight2, ButtonSelect, ButtonStart, ButtonMode, ButtonThumbL, ButtonThumbR,
	/* ButtonDigi */ButtonToolPen = 0x140, ButtonToolRubber, ButtonToolBrush, ButtonToolPencil, ButtonToolAirbrush, ButtonToolFinger, ButtonToolMouse, ButtonToolLens, ButtonToolQuintTap,
	ButtonTouch, ButtonStylus, ButtonStylus2, ButtonToolDoubleTap, ButtonToolTripleTap, ButtonToolQuadTap,
	/* ButtonWheel */ButtonGearDown = 0x150, ButtonGearUp,
	Ok = 0x160, Select, Goto, Clear, Power2, Option, Info, Time, Vendor, Archive, Program,
	Channel, Favorites, EPG, PVR, MHP, Language, Title, Subtitle, Angle, Zoom, Mode, Keyboard, Screen, PC, TV, TV2, VCR, VCR2,
	SAT, SAT2, CD, Tape, Radio, Tuner, Player, Text, DVD, AUX, MP3, Audio, Video,
	Directory, List, Memo, Calendar, Red, Green, Yellow, Blue, ChannelUp, ChannelDown,
	First, LAst, AB, Next, Restart, Slow, Shuffle, Break, Previous, Digits, Teen, Twen,
	VideoPhone, Games, ZoomIn, ZoomOut, ZoomReset, WordProcessor, Editor, SpreadSheet, GraphicsEditor, Presentation,
	Database, News, VoiceMail, AddressBook, Messenger, DisplayToggle, SpellCheck, Logoff,
	Dollar, Euro,
	FrameBack, FrameForward, ContextMenu, MediaRepeat, ChannelsUp10, ChannelsDown10,
	DelEOL = 0x1c0, DelEOS, InsLine, DelLine,
	Fn = 0x1d0, FnEsc, FnF1, FnF2, FnF3, FnF4, FnF5, FnF6, FnF7, FnF8, FnF9, FnF10, FnF11, FnF12,
	Fn1, FN2, FnD, FnE, FnF, FnS, FnB,
	Braille1 = 0x1f1, Braille2, Braille3, Braille4, Braille5, Braille6, Braille7, Braille8, Braille9, Braille10,
	Numeric0 = 0x200, Numeric1, Numeric2, Numeric3, Numeric4, Numeric5, Numeric6, Numeric7, Numeric8, Numeric9, NumericStar, NumericPound,
	NumericA, NumericB, NumericC, NumericD, CameraFocus, WPSButton,
	TouchpadToggle, TouchpadOn, TouchpadOff, CameraZoomIn, CameraZoomOut, CameraUp, CameraDown, CameraLeft, CameraRight,
	AttendantOn, AttendantOff, AttendantToggle, LightsToggle,
	DPADUp = 0x220, DPADDown, DPADLeft, DPADRight,
	ALSToggle = 0x230,
	ButtonConfig = 0x240, TaskManager, Journal, ControlPanel, AppSelect, ScreenSaver, VoiceCommand,
	BrightnessMin = 0x250, BrightnessMax,
	KBDInputAssistPrev = 0x260, KBDInputAssistNext, KBDInputAssistPrevGroup, KBDInputAssistNextGroup, KBDInputAssistAccept, KBDInputAssistCancel,
	RightUp, RightDown, LeftUp, LeftDown,
	RotMenu, MediaTopMenu, Numeric11, Numerc12,
	AudioDesc, Dynamic3Mode, NextFavorite, StopRecord, PauseRecord, VOD, Unmute, FastReverse, SlowReverse,
	/* TriggerHappy */TriggerHappy1 = 0x2c0, TriggerHappy2, TriggerHappy3, TriggerHappy5, TriggerHappy6, TriggerHappy7, TriggerHappy8, TriggerHappy9, TriggerHappy10,
	TriggerHappy11, TriggerHappy12, TriggerHappy13, TriggerHappy14, TriggerHappy15, TriggerHappy16, TriggerHappy17, TriggerHappy18, TriggerHappy19, TriggerHappy20,
	TriggerHappy21, TriggerHappy22, TriggerHappy23, TriggerHappy24, TriggerHappy25, TriggerHappy26, TriggerHappy27, TriggerHappy28, TriggerHappy29, TriggerHappy30,
	TriggerHappy31, TriggerHappy32, TriggerHappy33, TriggerHappy34, TriggerHappy35, TriggerHappy36, TriggerHappy37, TriggerHappy38, TriggerHappy39, TriggerHappy40
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum AbsoluteAxisEvents
{
	X = 0x00, Y, Z, RX, RY, RZ, Throttle, Rudder, Wheel, Gas, Brake,
	Hat0x = 0x10, Hat0y, Hat1x, Hat1y, Hat2x, Hat2y, Hat3x, Hat3y, Pressure, Distance,
	TiltX, TiltY, ToolWidth,
	Volume = 0x20, Misc = 0x28,
	MTSlot = 0x2f, MTTouchMajor, MTTouchMinor, MTWidthMajor, MTWidthMinor,
	MTOrientation, MTPositionX, MTPositionY, MTToolType, MTBlobID,
	MTTrackingID, MTPressure, MTDistance, MTToolX, MTToolY
}
#[repr(C)] #[derive(Debug, Clone)]
pub enum ForceFeedbackEffectTypes
{
	Rumble = 0x50, Periodic, Constant, Spring, Friction, Damper, Inertia, Ramp
}
#[repr(C)] #[derive(Debug, Clone)]
pub enum ForceFeedbackWaveforms
{
	Square = 0x58, Triangle, Sine, SawUp, SawDown, Custom
}
#[repr(C)] #[derive(Debug, Clone)]
pub enum ForceFeedbackDeviceProperties
{
	Gain = 0x60, AutoCenter
}
#[derive(Debug, Clone)]
pub struct AxisProperties
{
	pub axis: AbsoluteAxisEvents, pub range: std::ops::Range<i32>,
	pub fuzz: i32, pub dead: i32, pub resolution: i32
}

#[derive(Clone)]
pub struct EventDeviceParams
{
	pub fs_location: String,
	pub bus_type: BusType, pub vendor: u16, pub product: u16, pub version: u16, pub name: String,
	pub syn_events: Vec<SynEvents>, pub key_events: Vec<KeyEvents>, pub axis_events: HashMap<AbsoluteAxisEvents, AxisProperties>,
	pub ff_effect_types: Vec<ForceFeedbackEffectTypes>, pub ff_waveforms: Vec<ForceFeedbackWaveforms>,
	pub ff_properties: Vec<ForceFeedbackDeviceProperties>
}
impl EventDeviceParams
{
	pub fn get_from_fd<FileHandle: AsRawFd>(fs_location: &str, file: &FileHandle) -> Self
	{
		let fd = file.as_raw_fd();
		let evdev_id = unsafe
		{
			let mut ret: input_id = std::mem::uninitialized();
			let iores = libc::ioctl(fd, EVIOCGID!(), &mut ret);
			if iores == -1 { panic!("Failed to perform ioctl: {:?}", std::io::Error::last_os_error()); }
			ret
		};
		let evdev_name = unsafe
		{
			let mut ret: [u8; 256] = std::mem::uninitialized();
			let iores = libc::ioctl(fd, EVIOCGNAME!(256), ret.as_mut_ptr());
			if iores == -1 { panic!("Failed to perform ioctl: {:?}", std::io::Error::last_os_error()); }
			Vec::from(&ret[..(iores - 1) as usize])
		};

		fn perform_event_ioctl(fd: std::os::unix::io::RawFd, e: Event, len_bits: usize) -> Vec<u8>
		{
			let mut ret = vec![0u8; (len_bits as f32 / 8.0f32).ceil() as usize];
			let iores = unsafe { libc::ioctl(fd, EVIOCGBIT!(e as u64, ret.len()), ret.as_mut_ptr()) };
			if iores == -1 { warn!(target: "Prelude::evdev", "Failed to perform ioctl or unsupported(Event::{:?}: {:?})", e, std::io::Error::last_os_error()) };
			ret
		}
		let syn_events = {
			let d = perform_event_ioctl(fd, Event::Syn, SYN_CNT as usize);
			(0 .. 3).filter(|&o| ((d[0] >> o) & 0x01) != 0)
				.map(|o| unsafe { std::mem::transmute::<_, SynEvents>(o as u32) }).collect::<Vec<_>>()
		};
		let key_events = {
			let d = perform_event_ioctl(fd, Event::Key, KEY_CNT as usize);
			d.iter().enumerate().flat_map(|(n, &b)| (0 .. 8).filter(move |o| ((b >> o) & 0x01) != 0)
				.map(move |o| unsafe { std::mem::transmute::<_, KeyEvents>((n * 8 + o) as u32) })).collect::<Vec<_>>()
		};
		let axis_events = {
			let d = perform_event_ioctl(fd, Event::Absolute, ABS_CNT as usize);
			d.iter().enumerate().flat_map(|(n, &b)| (0 .. 8).filter(move |o| ((b >> o) & 0x01) != 0)
				.map(move |o|
				{
					let axis = unsafe { std::mem::transmute::<_, AbsoluteAxisEvents>((n * 8 + o) as u32) };
					let info = unsafe
					{
						let mut ret: input_absinfo = std::mem::uninitialized();
						let iores = libc::ioctl(fd, EVIOCGABS!(n * 8 + o), &mut ret);
						if iores == -1 { warn!(target: "Prelude::evdev", "Failed to perform ioctl: {:?}", std::io::Error::last_os_error()); }
						ret
					};
					(axis.clone(), AxisProperties
					{
						axis: axis, range: info.minimum .. info.maximum,
						fuzz: info.fuzz, dead: info.flat, resolution: info.resolution
					})
				})).collect::<HashMap<_, _>>()
		};
		let (ff_effect_types, ff_waveforms, ff_device_props) = {
			let d = perform_event_ioctl(fd, Event::ForceFeedback, FF_CNT as usize);
			(
				(0 .. 8).filter(|&o| ((d[ForceFeedbackEffectTypes::Rumble as usize >> 3] >> o) & 0x01) != 0)
					.map(|o| unsafe { std::mem::transmute::<_, ForceFeedbackEffectTypes>(ForceFeedbackEffectTypes::Rumble as u32 + o as u32) })
					.collect::<Vec<_>>(),
				(0 .. 6).filter(|&o| ((d[ForceFeedbackWaveforms::Square as usize >> 3] >> o) & 0x01) != 0)
					.map(|o| unsafe { std::mem::transmute::<_, ForceFeedbackWaveforms>(ForceFeedbackWaveforms::Square as u32 + o as u32) })
					.collect::<Vec<_>>(),
				(0 .. 2).filter(|&o| ((d[ForceFeedbackDeviceProperties::Gain as usize >> 3] >> o) & 0x01) != 0)
					.map(|o| unsafe { std::mem::transmute::<_, ForceFeedbackDeviceProperties>(ForceFeedbackDeviceProperties::Gain as u32 + o as u32) })
					.collect::<Vec<_>>()
			)
		};

		EventDeviceParams
		{
			fs_location: fs_location.to_string(),
			bus_type: match evdev_id.bustype
			{
				BUS_PCI => BusType::PCI,
				BUS_ISAPNP => BusType::ISAPnP,
				BUS_USB => BusType::USB,
				BUS_HIL => BusType::HIL,
				BUS_BLUETOOTH => BusType::Bluetooth,
				BUS_VIRTUAL => BusType::Virtual,
				BUS_ISA => BusType::ISA,
				BUS_I8042 => BusType::I8042,
				BUS_XTKBD => BusType::XTKBD,
				BUS_RS232 => BusType::RS232,
				BUS_GAMEPORT => BusType::Gameport,
				BUS_PARPORT => BusType::ParallelPort,
				BUS_AMIGA => BusType::Amiga,
				BUS_ADB => BusType::ADB,
				BUS_I2C => BusType::I2C,
				BUS_HOST => BusType::Host,
				BUS_GSC => BusType::GSC,
				BUS_ATARI => BusType::Atari,
				BUS_SPI => BusType::SPI,
				BUS_RMI => BusType::RMI,
				BUS_CEC => BusType::CEC,
				_ => BusType::Unknown
			},
			vendor: evdev_id.vendor, product: evdev_id.product, version: evdev_id.version,
			name: String::from_utf8(evdev_name).unwrap(),
			syn_events: syn_events, key_events: key_events, axis_events: axis_events,
			ff_effect_types: ff_effect_types, ff_waveforms: ff_waveforms, ff_properties: ff_device_props
		}
	}
}

pub enum PressedState
{
	Released, Pressed, Repeating
}
pub enum DeviceEvent
{
	Syn(libc::timeval, SynEvents),
	Key(libc::timeval, KeyEvents, PressedState),
	Absolute(libc::timeval, AbsoluteAxisEvents, f32),
	Generic(input_event)
}

pub struct EventDevice
{
	params: EventDeviceParams, reader: std::io::BufReader<std::fs::File>, data_u8: Vec<u8>
}
impl EventDevice
{
	pub fn new(node_path: &str) -> Result<Self, EngineError>
	{
		std::fs::OpenOptions::new().read(true).open(node_path).map_err(EngineError::from).map(|fp| EventDevice
		{
			params: EventDeviceParams::get_from_fd(node_path, &fp), reader: std::io::BufReader::new(fp),
			data_u8: vec![0u8; std::mem::size_of::<input_event>()]
		})
	}
	pub fn grab_device(&self) -> Result<(), EngineError>
	{
		let grab_flag: libc::c_int = 1;
		let iores = unsafe { libc::ioctl(self.as_raw_fd(), EVIOCGRAB!(), &grab_flag) };
		if iores == -1 { Err(EngineError::GenericError("Failed to grabbing event device")) } else { Ok(()) }
	}
	pub fn wait_event(&mut self) -> Result<DeviceEvent, EngineError>
	{
		self.reader.read_exact(&mut self.data_u8)
			.map(|()| unsafe { std::mem::transmute::<_, &input_event>(self.data_u8.as_ptr()) })
			.map(|ev| match unsafe { std::mem::transmute::<_, Event>(ev._type as u32) }
			{
				Event::Syn => DeviceEvent::Syn(ev.time, unsafe { std::mem::transmute::<_, SynEvents>(ev.code as u32) }),
				Event::Key => DeviceEvent::Key(ev.time, unsafe { std::mem::transmute::<_, KeyEvents>(ev.code as u32) },
					match ev.value { 1 => PressedState::Pressed, 2 => PressedState::Repeating, _ => PressedState::Released }),
				Event::Absolute =>
				{
					let axis = unsafe { std::mem::transmute::<_, AbsoluteAxisEvents>(ev.code as u32) };
					self.params.axis_events.get(&axis).map(|ap| if ev.value.abs() <= ap.dead * 2 { 0.0f32 }
					else
					{
						if ap.range.start == 0
						{
							// unidirectional
							(ev.value as f32 / ap.range.end as f32).min(1.0f32)
						}
						else
						{
							// bidirectional
							(2.0f32 * (ev.value as f32 - ap.range.start as f32)
								/ ap.range.len() as f32 - 1.0f32).max(-1.0f32).min(1.0f32)
						}
					}).map(|value_norm| DeviceEvent::Absolute(ev.time, axis, value_norm))
						.unwrap_or(DeviceEvent::Generic(ev.clone()))
				},
				_ => DeviceEvent::Generic(ev.clone())
			}).map_err(EngineError::from)
	}
	pub fn send_to(&mut self, sender: std::sync::mpsc::Sender<DeviceEvent>)
	{
		while let Ok(e) = self.wait_event()
		{
			if sender.send(e).is_err() { break; }
		}
	}

	pub fn name(&self) -> &str { &self.params.name }
}
impl std::os::unix::io::AsRawFd for EventDevice
{
	fn as_raw_fd(&self) -> std::os::unix::io::RawFd { self.reader.get_ref().as_raw_fd() }
}

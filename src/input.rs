// Interlude: Input System

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

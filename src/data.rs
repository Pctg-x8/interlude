// Interlude: Data Structures

use std;
use vk::*;

// Vulkan DataType Exports
/// Viewport: left, top, width, height, near, far
#[derive(Clone, Debug, PartialEq)] #[repr(C)] pub struct Viewport(pub f32, pub f32, pub f32, pub f32, pub f32, pub f32);
#[derive(Hash, Clone, Debug, PartialEq, Eq)] #[repr(C)] pub struct Size2(pub u32, pub u32);
#[derive(Hash, Clone, Debug, PartialEq, Eq)] #[repr(C)] pub struct Offset2(pub i32, pub i32);
#[derive(Hash, Clone, Debug, PartialEq, Eq)] #[repr(C)] pub struct Offset3(pub i32, pub i32, pub i32);
#[derive(Hash, Clone, Debug, PartialEq, Eq)] #[repr(C)] pub struct Size3(pub u32, pub u32, pub u32);
#[derive(Hash, Clone, Debug, PartialEq, Eq)] #[repr(C)] pub struct Rect2(pub Offset2, pub Size2);
impl std::convert::From<Size2> for Viewport
{
	fn from(s: Size2) -> Viewport
	{
		let Size2(w, h) = s;
		Viewport(0.0, 0.0, w as f32, h as f32, 0.0, 1.0)
	}
}
impl std::convert::From<Size3> for Size2
{
	fn from(s: Size3) -> Size2 { let Size3(x, y, _) = s; Size2(x, y) }
}
impl std::convert::From<Size2> for Size3 { fn from(s: Size2) -> Size3 { let Size2(x, y) = s; Size3(x, y, 1) } }
macro_rules! ConvertByTransmuting
{
	($f: ty => $t: ty) =>
	{
		impl std::convert::From<$f> for $t { fn from(s: $f) -> Self { unsafe { std::mem::transmute(s) } } }
	}
}
ConvertByTransmuting!(VkExtent2D => Size2);
ConvertByTransmuting!(VkExtent3D => Size3);
ConvertByTransmuting!(VkOffset2D => Offset2);
ConvertByTransmuting!(VkOffset3D => Offset3);
impl Viewport
{
	pub fn make_from(s: &Size2) -> Viewport
	{
		let &Size2(w, h) = s;
		Viewport(0.0, 0.0, w as f32, h as f32, 0.0, 1.0)
	}
}
impl Size2
{
	pub fn width(&self) -> u32 { self.0 }
	pub fn height(&self) -> u32 { self.1 }
	pub fn hpart(&self, left: f32, right: f32, gap: u32) -> (Size2, Size2)
	{
		let total = left + right;
		(Size2((self.width() as f32 * left / total) as u32 - gap, self.height()), Size2((self.width() as f32 * right / total) as u32 - gap, self.height()))
	}
	pub fn vpart(&self, top: f32, bottom: f32, gap: u32) -> (Size2, Size2)
	{
		let total = top + bottom;
		(Size2(self.width(), (self.height() as f32 * top / total) as u32 - gap), Size2(self.width(), (self.height() as f32 * bottom / total) as u32 - gap))
	}
}
impl Size3
{
	pub fn width(&self) -> u32 { self.0 }
	pub fn height(&self) -> u32 { self.1 }
	pub fn depth(&self) -> u32 { self.2 }
}
impl Offset2
{
	pub fn x(&self) -> i32 { self.0 }
	pub fn y(&self) -> i32 { self.1 }
}
impl Offset3
{
	pub fn x(&self) -> i32 { self.0 }
	pub fn y(&self) -> i32 { self.1 }
	pub fn z(&self) -> i32 { self.2 }
}

// Extra Data Exports
#[derive(Clone, Debug, PartialEq)] #[repr(C)] pub struct Size2F(pub f32, pub f32);
#[derive(Clone, Debug, PartialEq)] #[repr(C)] pub struct Offset2F(pub f32, pub f32);
#[derive(Clone, Debug, PartialEq)] #[repr(C)] pub struct Size3F(pub f32, pub f32, pub f32);
#[derive(Clone, Debug, PartialEq)] #[repr(C)] pub struct Offset3F(pub f32, pub f32, pub f32);
#[derive(Clone, Debug, PartialEq)] #[repr(C)] pub struct Rect2F(pub Offset2F, pub Size2F);

/// x, y, z, w
#[repr(C)] #[derive(Clone)] pub struct Position(pub f32, pub f32, pub f32, pub f32);
/// x, y, u, v
#[repr(C)] #[derive(Clone)] pub struct PosUV(pub f32, pub f32, pub f32, pub f32);

// C(GLSL)-complatible Vector Types
pub type CVector4 = [f32; 4];
pub type CVector2 = [f32; 2];
pub type CMatrix4 = [CVector4; 4];

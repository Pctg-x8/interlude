// Interlude: Data Structures

use std::mem::transmute;
use interlude_vk_defs::{VkOffset2D, VkOffset3D, VkExtent2D, VkExtent3D, VkViewport, VkRect2D};

// Vulkan DataType Exports
/// Viewport: left, top, width, height, near, far
#[derive(Clone, Debug, PartialEq)] #[repr(C)] pub struct Viewport(pub f32, pub f32, pub f32, pub f32, pub f32, pub f32);
#[derive(Hash, Clone, Debug, PartialEq, Eq)] #[repr(C)] pub struct Size2(pub u32, pub u32);
#[derive(Hash, Clone, Debug, PartialEq, Eq)] #[repr(C)] pub struct Offset2(pub i32, pub i32);
#[derive(Hash, Clone, Debug, PartialEq, Eq)] #[repr(C)] pub struct Offset3(pub i32, pub i32, pub i32);
#[derive(Hash, Clone, Debug, PartialEq, Eq)] #[repr(C)] pub struct Size3(pub u32, pub u32, pub u32);
#[derive(Hash, Clone, Debug, PartialEq, Eq)] #[repr(C)] pub struct Rect2(pub Offset2, pub Size2);
impl<'s> From<&'s Size2> for Viewport
{
	fn from(s: &'s Size2) -> Viewport
	{
		let &Size2(w, h) = s;
		Viewport(0.0, 0.0, w as f32, h as f32, 0.0, 1.0)
	}
}
impl From<Size3> for Size2 { fn from(s: Size3) -> Size2 { let Size3(x, y, _) = s; Size2(x, y) } }
impl From<Size2> for Size3 { fn from(s: Size2) -> Size3 { let Size2(x, y) = s; Size3(x, y, 1) } }
macro_rules! CoerceSameBits
{
	($f: ty = $t: ty) =>
	{
		impl Into<$t> for $f { fn into(self) -> $t { unsafe { transmute(self) } } }
		impl Into<$f> for $t { fn into(self) -> $f { unsafe { transmute(self) } } }
		impl AsRef<$t> for $f { fn as_ref(&self) -> &$t { unsafe { transmute(self) } } }
		impl AsRef<$f> for $t { fn as_ref(&self) -> &$f { unsafe { transmute(self) } } }
	}
}
CoerceSameBits!(VkExtent2D = Size2);
CoerceSameBits!(VkExtent3D = Size3);
CoerceSameBits!(VkOffset2D = Offset2);
CoerceSameBits!(VkOffset3D = Offset3);
CoerceSameBits!(VkRect2D = Rect2);
CoerceSameBits!(VkViewport = Viewport);
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

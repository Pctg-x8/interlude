// Interlude: Data Structures

use std::mem::transmute;
use interlude_vk_defs::*;
use {EngineResult, EngineError};

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

/// Data Format
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Format
{
	Undefined, RG8, RGBA16, BGRA16, RGB16, BGR16, RGB15A1, BGR15A1, A1RGB15,
	Component(usize, PackedPixelOrder, FormatType), Data(usize, FormatType),
	A2RGB30(FormatType), A2BGR30(FormatType),
	B10G11R11UFloat, BGR18Exp5UFloat, X8D24UNormalized, Data2(usize, FormatType, usize, FormatType),
	Compressed(CompressionAlgorithm)
}
/// Data Format Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FormatType { UNormalized, Normalized, UScaled, Scaled, UInt, Int, Float, SRGB }
/// Packed Pixel Order
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PackedPixelOrder { R, RG, RGB, RGBA, BGR, BGRA, ABGR, ARGB }
/// Compression Algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CompressionAlgorithm
{
	BC1Linear, BC1NonLinear, BC1AlphaLinear, BC1AlphaNonLinear, BC2Linear, BC2NonLinear, BC3Linear, BC3NonLinear,
	BC4Signed, BC4Unsigned, BC5Signed, BC5Unsigned, BC6HSigned, BC6HUnsigned, BC7Linear, BC7NonLinear,
	ETC2Linear(usize), ETC2NonLinear(usize), EACSigned, EACUnsigned,
	EACGSigned, EACGUnsigned, ASTCLinear(ASTCPixelPack), ASTCNonLinear(ASTCPixelPack)
}
/// ASTC Pixel Packing Size
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ASTCPixelPack
{
	_4x4, _5x4, _5x5, _6x5, _6x6, _8x5, _8x6, _8x8, _10x5, _10x6, _10x8, _10x10, _12x10, _12x12
}
pub fn format(f: &Format) -> EngineResult<VkFormat>
{
	match f
	{
		&Format::Undefined => Ok(VK_FORMAT_UNDEFINED),
		&Format::RG8 => Ok(VK_FORMAT_R4G4_UNORM_PACK8),
		&Format::RGBA16 => Ok(VK_FORMAT_R4G4B4A4_UNORM_PACK16),
		&Format::BGRA16 => Ok(VK_FORMAT_B4G4R4A4_UNORM_PACK16),
		&Format::RGB15A1 => Ok(VK_FORMAT_R5G5B5A1_UNORM_PACK16),
		&Format::BGR15A1 => Ok(VK_FORMAT_B5G5R5A1_UNORM_PACK16),
		&Format::A1RGB15 => Ok(VK_FORMAT_A1R5G5B5_UNORM_PACK16),
		&Format::B10G11R11UFloat => Ok(VK_FORMAT_B10G11R11_UFLOAT_PACK32),
		&Format::BGR18Exp5UFloat => Ok(VK_FORMAT_E5B9G9R9_UFLOAT_PACK32),
		&Format::X8D24UNormalized => Ok(VK_FORMAT_X8_D24_UNORM_PACK32),
		&Format::Component(8, pp, ft) => match ft
		{
			FormatType::UNormalized => match pp
			{
				PackedPixelOrder::R => Ok(VK_FORMAT_R8_UNORM), PackedPixelOrder::RG => Ok(VK_FORMAT_R8G8_UNORM),
				PackedPixelOrder::RGB => Ok(VK_FORMAT_R8G8B8_UNORM), PackedPixelOrder::RGBA => Ok(VK_FORMAT_R8G8B8A8_UNORM),
				PackedPixelOrder::BGR => Ok(VK_FORMAT_B8G8R8_UNORM), PackedPixelOrder::BGRA => Ok(VK_FORMAT_B8G8R8A8_UNORM),
				PackedPixelOrder::ABGR => Ok(VK_FORMAT_A8B8G8R8_UNORM_PACK32), _ => Err(EngineError::InvalidFormatCombination)
			},
			FormatType::Normalized => match pp
			{
				PackedPixelOrder::R => Ok(VK_FORMAT_R8_SNORM), PackedPixelOrder::RG => Ok(VK_FORMAT_R8G8_SNORM),
				PackedPixelOrder::RGB => Ok(VK_FORMAT_R8G8B8_SNORM), PackedPixelOrder::RGBA => Ok(VK_FORMAT_R8G8B8A8_SNORM),
				PackedPixelOrder::BGR => Ok(VK_FORMAT_B8G8R8_SNORM), PackedPixelOrder::BGRA => Ok(VK_FORMAT_B8G8R8A8_SNORM),
				PackedPixelOrder::ABGR => Ok(VK_FORMAT_A8B8G8R8_SNORM_PACK32), _ => Err(EngineError::InvalidFormatCombination)
			},
			FormatType::UScaled => match pp
			{
				PackedPixelOrder::R => Ok(VK_FORMAT_R8_USCALED), PackedPixelOrder::RG => Ok(VK_FORMAT_R8G8_USCALED),
				PackedPixelOrder::RGB => Ok(VK_FORMAT_R8G8B8_USCALED), PackedPixelOrder::RGBA => Ok(VK_FORMAT_R8G8B8A8_USCALED),
				PackedPixelOrder::BGR => Ok(VK_FORMAT_B8G8R8_USCALED), PackedPixelOrder::BGRA => Ok(VK_FORMAT_B8G8R8A8_USCALED),
				PackedPixelOrder::ABGR => Ok(VK_FORMAT_A8B8G8R8_USCALED_PACK32), _ => Err(EngineError::InvalidFormatCombination)
			},
			FormatType::Scaled => match pp
			{
				PackedPixelOrder::R => Ok(VK_FORMAT_R8_SSCALED), PackedPixelOrder::RG => Ok(VK_FORMAT_R8G8_SSCALED),
				PackedPixelOrder::RGB => Ok(VK_FORMAT_R8G8B8_SSCALED), PackedPixelOrder::RGBA => Ok(VK_FORMAT_R8G8B8A8_SSCALED),
				PackedPixelOrder::BGR => Ok(VK_FORMAT_B8G8R8_SSCALED), PackedPixelOrder::BGRA => Ok(VK_FORMAT_B8G8R8A8_SSCALED),
				PackedPixelOrder::ABGR => Ok(VK_FORMAT_A8B8G8R8_SSCALED_PACK32), _ => Err(EngineError::InvalidFormatCombination)
			},
			FormatType::UInt => match pp
			{
				PackedPixelOrder::R => Ok(VK_FORMAT_R8_UINT), PackedPixelOrder::RG => Ok(VK_FORMAT_R8G8_UINT),
				PackedPixelOrder::RGB => Ok(VK_FORMAT_R8G8B8_UINT), PackedPixelOrder::RGBA => Ok(VK_FORMAT_R8G8B8A8_UINT),
				PackedPixelOrder::BGR => Ok(VK_FORMAT_B8G8R8_UINT), PackedPixelOrder::BGRA => Ok(VK_FORMAT_B8G8R8A8_UINT),
				PackedPixelOrder::ABGR => Ok(VK_FORMAT_A8B8G8R8_UINT_PACK32), _ => Err(EngineError::InvalidFormatCombination)
			},
			FormatType::Int => match pp
			{
				PackedPixelOrder::R => Ok(VK_FORMAT_R8_SINT), PackedPixelOrder::RG => Ok(VK_FORMAT_R8G8_SINT),
				PackedPixelOrder::RGB => Ok(VK_FORMAT_R8G8B8_SINT), PackedPixelOrder::RGBA => Ok(VK_FORMAT_R8G8B8A8_SINT),
				PackedPixelOrder::BGR => Ok(VK_FORMAT_B8G8R8_SINT), PackedPixelOrder::BGRA => Ok(VK_FORMAT_B8G8R8A8_SINT),
				PackedPixelOrder::ABGR => Ok(VK_FORMAT_A8B8G8R8_SINT_PACK32), _ => Err(EngineError::InvalidFormatCombination)
			},
			FormatType::SRGB => match pp
			{
				PackedPixelOrder::R => Ok(VK_FORMAT_R8_SRGB), PackedPixelOrder::RG => Ok(VK_FORMAT_R8G8_SRGB),
				PackedPixelOrder::RGB => Ok(VK_FORMAT_R8G8B8_SRGB), PackedPixelOrder::RGBA => Ok(VK_FORMAT_R8G8B8A8_SRGB),
				PackedPixelOrder::BGR => Ok(VK_FORMAT_B8G8R8_SRGB), PackedPixelOrder::BGRA => Ok(VK_FORMAT_B8G8R8A8_SRGB),
				PackedPixelOrder::ABGR => Ok(VK_FORMAT_A8B8G8R8_SRGB_PACK32), _ => Err(EngineError::InvalidFormatCombination)
			}, _ => Err(EngineError::InvalidFormatCombination)
		},
		&Format::Component(16, pp, ft) => match ft
		{
			FormatType::UNormalized => match pp
			{
				PackedPixelOrder::R => Ok(VK_FORMAT_R16_UNORM), PackedPixelOrder::RG => Ok(VK_FORMAT_R16G16_UNORM),
				PackedPixelOrder::RGB => Ok(VK_FORMAT_R16G16B16_UNORM), PackedPixelOrder::RGBA => Ok(VK_FORMAT_R16G16B16A16_UNORM),
				_ => Err(EngineError::InvalidFormatCombination)
			},
			FormatType::Normalized => match pp
			{
				PackedPixelOrder::R => Ok(VK_FORMAT_R16_SNORM), PackedPixelOrder::RG => Ok(VK_FORMAT_R16G16_SNORM),
				PackedPixelOrder::RGB => Ok(VK_FORMAT_R16G16B16_SNORM), PackedPixelOrder::RGBA => Ok(VK_FORMAT_R16G16B16A16_SNORM),
				_ => Err(EngineError::InvalidFormatCombination)
			},
			FormatType::UScaled => match pp
			{
				PackedPixelOrder::R => Ok(VK_FORMAT_R16_USCALED), PackedPixelOrder::RG => Ok(VK_FORMAT_R16G16_USCALED),
				PackedPixelOrder::RGB => Ok(VK_FORMAT_R16G16B16_USCALED), PackedPixelOrder::RGBA => Ok(VK_FORMAT_R16G16B16A16_USCALED),
				_ => Err(EngineError::InvalidFormatCombination)
			},
			FormatType::Scaled => match pp
			{
				PackedPixelOrder::R => Ok(VK_FORMAT_R16_SSCALED), PackedPixelOrder::RG => Ok(VK_FORMAT_R16G16_SSCALED),
				PackedPixelOrder::RGB => Ok(VK_FORMAT_R16G16B16_SSCALED), PackedPixelOrder::RGBA => Ok(VK_FORMAT_R16G16B16A16_SSCALED),
				_ => Err(EngineError::InvalidFormatCombination)
			},
			FormatType::UInt => match pp
			{
				PackedPixelOrder::R => Ok(VK_FORMAT_R16_UINT), PackedPixelOrder::RG => Ok(VK_FORMAT_R16G16_UINT),
				PackedPixelOrder::RGB => Ok(VK_FORMAT_R16G16B16_UINT), PackedPixelOrder::RGBA => Ok(VK_FORMAT_R16G16B16A16_UINT),
				_ => Err(EngineError::InvalidFormatCombination)
			},
			FormatType::Int => match pp
			{
				PackedPixelOrder::R => Ok(VK_FORMAT_R16_SINT), PackedPixelOrder::RG => Ok(VK_FORMAT_R16G16_SINT),
				PackedPixelOrder::RGB => Ok(VK_FORMAT_R16G16B16_SINT), PackedPixelOrder::RGBA => Ok(VK_FORMAT_R16G16B16A16_SINT),
				_ => Err(EngineError::InvalidFormatCombination)
			},
			FormatType::Float => match pp
			{
				PackedPixelOrder::R => Ok(VK_FORMAT_R16_SFLOAT), PackedPixelOrder::RG => Ok(VK_FORMAT_R16G16_SFLOAT),
				PackedPixelOrder::RGB => Ok(VK_FORMAT_R16G16B16_SFLOAT), PackedPixelOrder::RGBA => Ok(VK_FORMAT_R16G16B16A16_SFLOAT),
				_ => Err(EngineError::InvalidFormatCombination)
			}, _ => Err(EngineError::InvalidFormatCombination)
		},
		&Format::Component(32, pp, ft) => match ft
		{
			FormatType::UInt => match pp
			{
				PackedPixelOrder::R => Ok(VK_FORMAT_R32_UINT), PackedPixelOrder::RG => Ok(VK_FORMAT_R32G32_UINT),
				PackedPixelOrder::RGB => Ok(VK_FORMAT_R32G32B32_UINT), PackedPixelOrder::RGBA => Ok(VK_FORMAT_R32G32B32A32_UINT),
				_ => Err(EngineError::InvalidFormatCombination)
			},
			FormatType::Int => match pp
			{
				PackedPixelOrder::R => Ok(VK_FORMAT_R32_SINT), PackedPixelOrder::RG => Ok(VK_FORMAT_R32G32_SINT),
				PackedPixelOrder::RGB => Ok(VK_FORMAT_R32G32B32_SINT), PackedPixelOrder::RGBA => Ok(VK_FORMAT_R32G32B32A32_SINT),
				_ => Err(EngineError::InvalidFormatCombination)
			},
			FormatType::Float => match pp
			{
				PackedPixelOrder::R => Ok(VK_FORMAT_R32_SFLOAT), PackedPixelOrder::RG => Ok(VK_FORMAT_R32G32_SFLOAT),
				PackedPixelOrder::RGB => Ok(VK_FORMAT_R32G32B32_SFLOAT), PackedPixelOrder::RGBA => Ok(VK_FORMAT_R32G32B32A32_SFLOAT),
				_ => Err(EngineError::InvalidFormatCombination)
			}, _ => Err(EngineError::InvalidFormatCombination)
		},
		&Format::Component(64, pp, ft) => match ft
		{
			FormatType::UInt => match pp
			{
				PackedPixelOrder::R => Ok(VK_FORMAT_R64_UINT), PackedPixelOrder::RG => Ok(VK_FORMAT_R64G64_UINT),
				PackedPixelOrder::RGB => Ok(VK_FORMAT_R64G64B64_UINT), PackedPixelOrder::RGBA => Ok(VK_FORMAT_R64G64B64A64_UINT),
				_ => Err(EngineError::InvalidFormatCombination)
			},
			FormatType::Int => match pp
			{
				PackedPixelOrder::R => Ok(VK_FORMAT_R64_SINT), PackedPixelOrder::RG => Ok(VK_FORMAT_R64G64_SINT),
				PackedPixelOrder::RGB => Ok(VK_FORMAT_R64G64B64_SINT), PackedPixelOrder::RGBA => Ok(VK_FORMAT_R64G64B64A64_SINT),
				_ => Err(EngineError::InvalidFormatCombination)
			},
			FormatType::Float => match pp
			{
				PackedPixelOrder::R => Ok(VK_FORMAT_R64_SFLOAT), PackedPixelOrder::RG => Ok(VK_FORMAT_R64G64_SFLOAT),
				PackedPixelOrder::RGB => Ok(VK_FORMAT_R64G64B64_SFLOAT), PackedPixelOrder::RGBA => Ok(VK_FORMAT_R64G64B64A64_SFLOAT),
				_ => Err(EngineError::InvalidFormatCombination)
			}, _ => Err(EngineError::InvalidFormatCombination)
		},
		&Format::A2RGB30(FormatType::UNormalized) => Ok(VK_FORMAT_A2R10G10B10_UNORM_PACK32),
		&Format::A2RGB30(FormatType::Normalized)  => Ok(VK_FORMAT_A2R10G10B10_SNORM_PACK32),
		&Format::A2RGB30(FormatType::UScaled)     => Ok(VK_FORMAT_A2R10G10B10_USCALED_PACK32),
		&Format::A2RGB30(FormatType::Scaled)      => Ok(VK_FORMAT_A2R10G10B10_SSCALED_PACK32),
		&Format::A2RGB30(FormatType::UInt)        => Ok(VK_FORMAT_A2R10G10B10_UINT_PACK32),
		&Format::A2RGB30(FormatType::Int)         => Ok(VK_FORMAT_A2R10G10B10_SINT_PACK32),
		&Format::A2BGR30(FormatType::UNormalized) => Ok(VK_FORMAT_A2B10G10R10_UNORM_PACK32),
		&Format::A2BGR30(FormatType::Normalized)  => Ok(VK_FORMAT_A2B10G10R10_SNORM_PACK32),
		&Format::A2BGR30(FormatType::UScaled)     => Ok(VK_FORMAT_A2B10G10R10_USCALED_PACK32),
		&Format::A2BGR30(FormatType::Scaled)      => Ok(VK_FORMAT_A2B10G10R10_SSCALED_PACK32),
		&Format::A2BGR30(FormatType::UInt)        => Ok(VK_FORMAT_A2B10G10R10_UINT_PACK32),
		&Format::A2BGR30(FormatType::Int)         => Ok(VK_FORMAT_A2B10G10R10_SINT_PACK32),
		&Format::Data(8, FormatType::UInt) => Ok(VK_FORMAT_S8_UINT),
		&Format::Data(16, FormatType::UNormalized) => Ok(VK_FORMAT_D16_UNORM),
		&Format::Data(32, FormatType::Float) => Ok(VK_FORMAT_D32_SFLOAT),
		&Format::Data2(16, FormatType::UNormalized, 8, FormatType::UInt) => Ok(VK_FORMAT_D16_UNORM_S8_UINT),
		&Format::Data2(24, FormatType::UNormalized, 8, FormatType::UInt) => Ok(VK_FORMAT_D24_UNORM_S8_UINT),
		&Format::Data2(32, FormatType::Float, 8, FormatType::UInt) => Ok(VK_FORMAT_D32_SFLOAT_S8_UINT),
		&Format::Compressed(CompressionAlgorithm::BC1Linear)    => Ok(VK_FORMAT_BC1_RGB_UNORM_BLOCK),
		&Format::Compressed(CompressionAlgorithm::BC1NonLinear) => Ok(VK_FORMAT_BC1_RGB_SRGB_BLOCK),
		&Format::Compressed(CompressionAlgorithm::BC1AlphaLinear)    => Ok(VK_FORMAT_BC1_RGBA_UNORM_BLOCK),
		&Format::Compressed(CompressionAlgorithm::BC1AlphaNonLinear) => Ok(VK_FORMAT_BC1_RGBA_SRGB_BLOCK),
		&Format::Compressed(CompressionAlgorithm::BC2Linear)    => Ok(VK_FORMAT_BC2_UNORM_BLOCK),
		&Format::Compressed(CompressionAlgorithm::BC2NonLinear) => Ok(VK_FORMAT_BC2_SRGB_BLOCK),
		&Format::Compressed(CompressionAlgorithm::BC3Linear)    => Ok(VK_FORMAT_BC3_UNORM_BLOCK),
		&Format::Compressed(CompressionAlgorithm::BC3NonLinear) => Ok(VK_FORMAT_BC3_SRGB_BLOCK),
		&Format::Compressed(CompressionAlgorithm::BC4Unsigned)  => Ok(VK_FORMAT_BC4_UNORM_BLOCK),
		&Format::Compressed(CompressionAlgorithm::BC4Signed)    => Ok(VK_FORMAT_BC4_SNORM_BLOCK),
		&Format::Compressed(CompressionAlgorithm::BC5Unsigned)  => Ok(VK_FORMAT_BC5_UNORM_BLOCK),
		&Format::Compressed(CompressionAlgorithm::BC5Signed)    => Ok(VK_FORMAT_BC5_SNORM_BLOCK),
		&Format::Compressed(CompressionAlgorithm::BC6HUnsigned) => Ok(VK_FORMAT_BC6H_UFLOAT_BLOCK),
		&Format::Compressed(CompressionAlgorithm::BC6HSigned)   => Ok(VK_FORMAT_BC6H_SFLOAT_BLOCK),
		&Format::Compressed(CompressionAlgorithm::BC7Linear)    => Ok(VK_FORMAT_BC7_UNORM_BLOCK),
		&Format::Compressed(CompressionAlgorithm::BC7NonLinear) => Ok(VK_FORMAT_BC7_SRGB_BLOCK),
		&Format::Compressed(CompressionAlgorithm::ETC2Linear(0)) => Ok(VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK),
		&Format::Compressed(CompressionAlgorithm::ETC2Linear(1)) => Ok(VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK),
		&Format::Compressed(CompressionAlgorithm::ETC2Linear(8)) => Ok(VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK),
		&Format::Compressed(CompressionAlgorithm::ETC2NonLinear(0)) => Ok(VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK),
		&Format::Compressed(CompressionAlgorithm::ETC2NonLinear(1)) => Ok(VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK),
		&Format::Compressed(CompressionAlgorithm::ETC2NonLinear(8)) => Ok(VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK),
		&Format::Compressed(CompressionAlgorithm::EACUnsigned)  => Ok(VK_FORMAT_EAC_R11_UNORM_BLOCK),
		&Format::Compressed(CompressionAlgorithm::EACSigned)    => Ok(VK_FORMAT_EAC_R11_SNORM_BLOCK),
		&Format::Compressed(CompressionAlgorithm::EACGUnsigned) => Ok(VK_FORMAT_EAC_R11G11_UNORM_BLOCK),
		&Format::Compressed(CompressionAlgorithm::EACGSigned)   => Ok(VK_FORMAT_EAC_R11G11_SNORM_BLOCK),
		&Format::Compressed(CompressionAlgorithm::ASTCLinear(pp)) => match pp
		{
			ASTCPixelPack::_4x4 => Ok(VK_FORMAT_ASTC_4x4_UNORM_BLOCK),
			ASTCPixelPack::_5x4 => Ok(VK_FORMAT_ASTC_5x4_UNORM_BLOCK),
			ASTCPixelPack::_5x5 => Ok(VK_FORMAT_ASTC_5x4_UNORM_BLOCK),
			ASTCPixelPack::_6x5 => Ok(VK_FORMAT_ASTC_6x5_UNORM_BLOCK),
			ASTCPixelPack::_6x6 => Ok(VK_FORMAT_ASTC_6x6_UNORM_BLOCK),
			ASTCPixelPack::_8x5 => Ok(VK_FORMAT_ASTC_8x5_UNORM_BLOCK),
			ASTCPixelPack::_8x6 => Ok(VK_FORMAT_ASTC_8x6_UNORM_BLOCK),
			ASTCPixelPack::_8x8 => Ok(VK_FORMAT_ASTC_8x8_UNORM_BLOCK),
			ASTCPixelPack::_10x5 => Ok(VK_FORMAT_ASTC_10x5_UNORM_BLOCK),
			ASTCPixelPack::_10x6 => Ok(VK_FORMAT_ASTC_10x6_UNORM_BLOCK),
			ASTCPixelPack::_10x8 => Ok(VK_FORMAT_ASTC_10x8_UNORM_BLOCK),
			ASTCPixelPack::_10x10 => Ok(VK_FORMAT_ASTC_10x10_UNORM_BLOCK),
			ASTCPixelPack::_12x10 => Ok(VK_FORMAT_ASTC_12x10_UNORM_BLOCK),
			ASTCPixelPack::_12x12 => Ok(VK_FORMAT_ASTC_12x12_UNORM_BLOCK)
		},
		&Format::Compressed(CompressionAlgorithm::ASTCNonLinear(pp)) => match pp
		{
			ASTCPixelPack::_4x4 => Ok(VK_FORMAT_ASTC_4x4_SRGB_BLOCK),
			ASTCPixelPack::_5x4 => Ok(VK_FORMAT_ASTC_5x4_SRGB_BLOCK),
			ASTCPixelPack::_5x5 => Ok(VK_FORMAT_ASTC_5x4_SRGB_BLOCK),
			ASTCPixelPack::_6x5 => Ok(VK_FORMAT_ASTC_6x5_SRGB_BLOCK),
			ASTCPixelPack::_6x6 => Ok(VK_FORMAT_ASTC_6x6_SRGB_BLOCK),
			ASTCPixelPack::_8x5 => Ok(VK_FORMAT_ASTC_8x5_SRGB_BLOCK),
			ASTCPixelPack::_8x6 => Ok(VK_FORMAT_ASTC_8x6_SRGB_BLOCK),
			ASTCPixelPack::_8x8 => Ok(VK_FORMAT_ASTC_8x8_SRGB_BLOCK),
			ASTCPixelPack::_10x5 => Ok(VK_FORMAT_ASTC_10x5_SRGB_BLOCK),
			ASTCPixelPack::_10x6 => Ok(VK_FORMAT_ASTC_10x6_SRGB_BLOCK),
			ASTCPixelPack::_10x8 => Ok(VK_FORMAT_ASTC_10x8_SRGB_BLOCK),
			ASTCPixelPack::_10x10 => Ok(VK_FORMAT_ASTC_10x10_SRGB_BLOCK),
			ASTCPixelPack::_12x10 => Ok(VK_FORMAT_ASTC_12x10_SRGB_BLOCK),
			ASTCPixelPack::_12x12 => Ok(VK_FORMAT_ASTC_12x12_SRGB_BLOCK)
		}
		_ => Err(EngineError::InvalidFormatCombination)
	}
}

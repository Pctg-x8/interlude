#![allow(dead_code)]

// Vulkan C to Rust FFI Macros/Compile-Time functions

pub const VK_VERSION_1_0: u32 = 1;

macro_rules! VK_MAKE_VERSION
{
	($major: expr, $minor: expr, $patch: expr) => (($major << 22) | ($minor << 12) | $patch)
}
pub const VK_API_VERSION_1_0: u32 = VK_MAKE_VERSION!(1, 0, 0);
pub const VK_API_VERSION: u32 = VK_MAKE_VERSION!(1, 0, 0);

pub const VK_MAX_PHYSICAL_DEVICE_NAME_SIZE: usize = 256;
pub const VK_UUID_SIZE: usize = 16;
pub const VK_MAX_MEMORY_TYPES: usize = 32;
pub const VK_MAX_MEMORY_HEAPS: usize = 16;

// Zero-Terminated(C compatible) Extension Names
pub const VK_KHR_SURFACE_EXTENSION_NAME: &'static str = "VK_KHR_surface\0";
pub const VK_KHR_SWAPCHAIN_EXTENSION_NAME: &'static str = "VK_KHR_swapchain\0";

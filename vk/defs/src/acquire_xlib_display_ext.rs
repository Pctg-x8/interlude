//! VK_EXT_acquire_xlib_display extensions

pub const VK_EXT_ACQUIRE_XLIB_DISPLAY_SPEC_VERSION: usize = 1;
pub static VK_EXT_ACQUIRE_XLIB_DISPLAY_EXTENSION_NAME: &'static str = "VK_EXT_acquire_xlib_display";

use x11::xlib::*;
use x11::xrandr::*;
use super::*;

pub type PFN_vkAcquireXlibDisplayEXT = extern "system" fn(physicalDevice: VkPhysicalDevice, dpy: *mut Display, display: VkDisplayKHR) -> VkResult;
pub type PFN_vkGetRandROutputDisplayEXT = extern "system" fn(physicalDevice: VkPhysicalDevice, dpy: *mut Display, rrOutput: RROutput, pDisplay: *mut VkDisplayKHR) -> VkResult;

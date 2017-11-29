//! VK_KHR_display extensions

use libc::*;
use super::*;

mod nd_handle_base_ts { pub enum VkDisplayKHR {} pub enum VkDisplayModeKHR {} }
pub type VkDisplayKHR = VK_NON_DISPATCHABLE_HANDLE!(VkDisplayKHR);
pub type VkDisplayModeKHR = VK_NON_DISPATCHABLE_HANDLE!(VkDisplayModeKHR);

pub const VK_KHR_DISPLAY_SPEC_VERSION: usize = 21;
pub const VK_KHR_DISPLAY_EXTENSION_NAME: &'static str = "VK_KHR_display";

pub type VkDisplayPlaneAlphaFlagsKHR = VkFlags;
pub const VK_DISPLAY_PLANE_ALPHA_OPAQUE_BIT_KHR: VkDisplayPlaneAlphaFlagsKHR = 0x01;
pub const VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR: VkDisplayPlaneAlphaFlagsKHR = 0x02;
pub const VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_BIT_KHR: VkDisplayPlaneAlphaFlagsKHR = 0x04;
pub const VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_PREMULTIPLIED_BIT_KHR: VkDisplayPlaneAlphaFlagsKHR = 0x08;
pub type VkDisplayModeCreateFlagsKHR = VkFlags;
pub type VkDisplaySurfaceCreateFlagsKHR = VkFlags;

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDisplayPropertiesKHR
{
    pub display: VkDisplayKHR, pub displayName: *const c_char,
    pub physicalDimensions: VkExtent2D, pub physicalResolution: VkExtent2D,
    pub supportedTransforms: VkSurfaceTransformFlagsKHR, pub planeReorderPossible: VkBool32,
    pub persistentContent: VkBool32
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDisplayModeParametersKHR { pub visibleRegion: VkExtent2D, pub refreshRate: u32 }
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDisplayModePropertiesKHR
{
    pub displayMode: VkDisplayModeKHR, pub parameters: VkDisplayModeParametersKHR
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDisplayModeCreateInfoKHR
{
    pub sType: VkStructureType, pub pNext: *const c_void,
    pub flags: VkDisplayModeCreateFlagsKHR, pub parameters: VkDisplayModeParametersKHR
}
impl Default for VkDisplayModeCreateInfoKHR
{
    fn default() -> Self
    {
        VkDisplayModeCreateInfoKHR { sType: VK_STRUCTURE_TYPE_DISPLAY_MODE_CREATE_INFO_KHR, .. unsafe { std::mem::zeroed() } }
    }
}

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDisplayPlaneCapabilitiesKHR
{
    pub supportedAlpha: VkDisplayPlaneAlphaFlagsKHR,
    pub minSrcPosition: VkOffset2D, pub maxSrcPosition: VkOffset2D,
    pub minSrcExtent: VkExtent2D, pub maxSrcExtent: VkExtent2D,
    pub minDstPosition: VkOffset2D, pub maxDstPosition: VkOffset2D,
    pub minDstExtent: VkExtent2D, pub maxDstExtent: VkExtent2D
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDisplayPlanePropertiesKHR
{
    pub currentDisplay: VkDisplayKHR, pub currentStackIndex: u32
}
#[repr(C)] #[derive(Debug, Clone, PartialEq)]
pub struct VkDisplaySurfaceCreateInfoKHR
{
    pub sType: VkStructureType, pub pNext: *const c_void,
    pub flags: VkDisplaySurfaceCreateFlagsKHR,
    pub displayMode: VkDisplayModeKHR, pub planeIndex: u32, pub planeStackIndex: u32,
    pub transform: VkSurfaceTransformFlagsKHR, pub globalAlpha: c_float,
    pub alphaMode: VkDisplayPlaneAlphaFlagsKHR, pub imageExtent: VkExtent2D
}
impl Default for VkDisplaySurfaceCreateInfoKHR
{
    fn default() -> Self
    {
        VkDisplaySurfaceCreateInfoKHR { sType: VK_STRUCTURE_TYPE_DISPLAY_SURFACE_CREATE_INFO_KHR, .. unsafe { std::mem::zeroed() } }
    }
}

pub type PFN_vkGetPhysicalDeviceDisplayPropertiesKHR = extern "system" fn(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkDisplayPropertiesKHR) -> VkResult;
pub type PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR = extern "system" fn(physicalDevice: VkPhysicalDevice, pProertyCount: *mut u32, pProperties: *mut VkDisplayPlanePropertiesKHR) -> VkResult;
pub type PFN_vkGetDisplayPlaneSupportedDisplayKHR = extern "system" fn(physicalDevice: VkPhysicalDevice, planeIndex: u32, pDisplayCount: *mut u32, pDisplays: *mut VkDisplayKHR) -> VkResult;
pub type PFN_vkGetDisplayModePropertiesKHR = extern "system" fn(physicalDevice: VkPhysicalDevice, display: VkDisplayKHR, pPropertyCount: *mut u32, pProperties: *mut VkDisplayModePropertiesKHR) -> VkResult;
pub type PFN_vkCreateDisplayModeKHR = extern "system" fn(physicalDevice: VkPhysicalDevice, display: VkDisplayKHR, pCreateInfo: *const VkDisplayModeCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pMode: *mut VkDisplayModeKHR) -> VkResult;
pub type PFN_vkGetDisplayPlaneCapabilitiesKHR = extern "system" fn(physicalDevice: VkPhysicalDevice, mode: VkDisplayModeKHR, planeIndex: u32, pCapabilities: *mut VkDisplayPlaneCapabilitiesKHR) -> VkResult;
pub type PFN_vkCreateDisplayPlaneSurfaceKHR = extern "system" fn(instance: VkInstance, pCreateInfo: *const VkDisplaySurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;

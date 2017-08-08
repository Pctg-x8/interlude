//! VK_KHR_incremental_present extensions

pub const VK_KHR_INCREMENTAL_PRESENT_SPEC_VERSION: usize = 1;
pub static VK_KHR_INCREMENTAL_PRESENT_EXTENSION_NAME: &'static str = "VK_KHR_incremental_present";

use libc::*;
use super::*;

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkRectLayerKHR
{
    pub offset: VkOffset2D, pub extent: VkExtent2D, pub layer: u32
}
impl VkRectLayerKHR
{
    pub fn new(x: i32, y: i32, width: u32, height: u32, layer: u32) -> Self
    {
        VkRectLayerKHR { offset: VkOffset2D { x, y }, extent: VkExtent2D { width, height }, layer }
    }
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPresentRegionsKHR
{
    pub sType: VkStructureType, pub pNext: *const c_void,
    pub swapchainCount: u32, pub pRegions: *const VkPresentRegionKHR
}
impl Default for VkPresentRegionsKHR
{
    fn default() -> Self
    {
        VkPresentRegionsKHR
        {
            sType: VK_STRUCTURE_TYPE_PRESENT_REGIONS_KHR,
            .. unsafe { std::mem::zeroed() }
        }
    }
}

//! VK_KHR_get_physical_device_properties2 extensions

use libc::*;
use super::*;

pub const VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_SPEC_VERSION: usize = 1;
pub static VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_EXTENSION_NAME: &'static str = "VK_KHR_get_physical_device_properties2";

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPhysicalDeviceFeatures2KHR
{
    pub sType: VkStructureType, pub pNext: *mut c_void,
    pub features: VkPhysicalDeviceFeatures
}
impl Default for VkPhysicalDeviceFeatures2KHR
{
    fn default() -> Self
    {
        VkPhysicalDeviceFeatures2KHR
        {
            sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2_KHR,
            .. unsafe { std::mem::zeroed() }
        }
    }
}

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPhysicalDeviceProperties2KHR
{
    pub sType: VkStructureType, pub pNext: *mut c_void,
    pub properties: VkPhysicalDeviceProperties
}
impl Default for VkPhysicalDeviceProperties2KHR
{
    fn default() -> Self
    {
        VkPhysicalDeviceProperties2KHR
        {
            sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2_KHR,
            .. unsafe { std::mem::zeroed() }
        }
    }
}

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkFormatProperties2KHR
{
    pub sType: VkStructureType, pub pNext: *mut c_void,
    pub formatProperties: VkFormatProperties
}
impl Default for VkFormatProperties2KHR
{
    fn default() -> Self
    {
        VkFormatProperties2KHR
        {
            sType: VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2_KHR,
            .. unsafe { std::mem::zeroed() }
        }
    }
}

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkImageFormatProperties2KHR
{
    pub sType: VkStructureType, pub pNext: *mut c_void,
    pub imageFormatProperties: VkImageFormatProperties
}
impl Default for VkImageFormatProperties2KHR
{
    fn default() -> Self
    {
        VkImageFormatProperties2KHR
        {
            sType: VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2_KHR,
            .. unsafe { std::mem::zeroed() }
        }
    }
}

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPhysicalDeviceImageFormatInfo2KHR
{
    pub sType: VkStructureType, pub pNext: *const c_void,
    pub format: VkFormat, pub _type: VkImageType, pub tiling: VkImageTiling,
    pub usage: VkImageUsage, pub flags: VkImageCreateFlags
}
impl Default for VkPhysicalDeviceImageFormatInfo2KHR
{
    fn default() -> Self
    {
        VkPhysicalDeviceImageFormatInfo2KHR
        {
            sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2_KHR,
            .. unsafe { std::mem::zeroed() }
        }
    }
}

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkQueueFamilyProperties2KHR
{
    pub sType: VkStrctureType, pub pNext: *mut c_void,
    pub queueFamilyProperties: VkQueueFamilyProperties
}
impl Default for VkQueueFamilyProperties2KHR
{
    fn default() -> Self
    {
        VkQueueFamilyProperties2KHR
        {
            sType: VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2_KHR,
            .. unsafe { std::mem::zeroed() }
        }
    }
}

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPhysicalDeviceMemoryProperies2KHR
{
    pub sType: VkStructureType, pub pNext: *mut c_void,
    pub memoryProperties: VkPhysicalDeviceMemoryProperties
}
impl Default for VkPhysicalDeviceMemoryProperties2KHR
{
    fn default() -> Self
    {
        VkPhysicalDeviceMemoryProperties2KHR
        {
            sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2_KHR,
            .. unsafe { std::mem::zeroed() }
        }
    }
}

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkSparseImageFormatProperties2KHR
{
    pub sType: VkStructureType, pub pNext: *mut c_void,
    pub properties: VkSparseImageFormatProperties
}
impl Default for VkSparseImageFormatProperties2KHR
{
    fn default() -> Self
    {
        VkSparseImageFormatProperties2KHR
        {
            sType: VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2_KHR,
            .. unsafe { std::mem::zeroed() }
        }
    }
}

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPhysicalDeviceSparseImageFormatInfo2KHR
{
    pub sType: VkStructureType, pub pNext: *const c_void,
    pub format: VkFormat, pub _type: VkImageType, pub samples: VkSampleCountFlags,
    pub usage: VkImageUsageFlags, pub tiling: VkImageTiling
}
impl Default for VkPhysicalDeviceSparseImageFormatInfo2KHR
{
    fn default() -> Self
    {
        VkPhysicalDeviceSparseImageFormatInfo2KHR
        {
            sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2_KHR,
            .. unsafe { std::mem::zeroed() }
        }
    }
}

pub type PFN_vkGetPhysicalDeviceFeatures2KHR = extern "system" fn(physicalDevice: VkPhysicalDevice, pFeatures: *mut VkPhysicalDeviceFeatures2KHR);
pub type PFN_vkGetPhysicalDeviceProperties2KHR = extern "system" fn(physicalDevice: VkPhysicalDevice, pProperties: *mut VkPhysicalDeviceProperties2KHR);
pub type PFN_vkGetPhysicalDeviceFormatProperties2KHR = extern "system" fn(physicalDevice: VkPhysicalDevice, format: VkFormat, pFormatProperties: *mut VkFormatProperties2KHR);
pub type PFN_vkGetPhysicalDeviceImageFormatProperties2KHR = extern "system" fn(physicalDevice: VkPhysicalDevice, pImageFormatInfo: *const VkPhysicalDeviceImageFormatInfo2KHR, pImageFormatProperties: *mut VkImageFormatProperties2KHR) -> VkResult;
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties2KHR = extern "system" fn(physicalDevice: VkPhysicalDevice, pQueueFamilyPropertyCount: *mut u32, pQueueFamilyPropeties: *mut VkQueueFamilyProperties2KHR);
pub type PFN_vkGetPhysicalDeviceMemoryProperties2KHR = extern "system" fn(physicalDevice: VkPhysicalDevice, pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties2KHR);
pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties2KHR = extern "system" fn(physicalDevice: VkPhysicalDevice, pFormatInfo: *const VkPhysicalDeviceSparseImageFormatInfo2KHR, pPropertyCount: *mut u32, pProperties: *mut VkSparseImageFormatPropeties2KHR);

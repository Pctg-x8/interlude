//! VK_KHR_external_memory_capabilities extensions

pub const VK_LUID_SIZE_KHR: usize = 8;
pub const VK_KHR_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION: usize = 1;
pub static VK_KHR_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME: &'static str = "VK_KHR_external_memory_capabilities";

pub type VkExternalMemoryHandleTypeFlagsKHR = VkFlags;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT_KHR: VkExternalMemoryHandleTypeFlagsKHR = 0x01;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR: VkExternalMemoryHandleTypeFlagsKHR = 0x02;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR: VkExternalMemoryHandleTypeFlagsKHR = 0x04;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT_KHR: VkExternalMemoryHandleTypeFlagsKHR = 0x08;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT_KHR: VkExternalMemoryHandleTypeFlagsKHR = 0x10;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT_KHR: VkExternalMemoryHandleTypeFlagsKHR = 0x20;
pub cosnt VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT_KHR: VkExternalMemoryHandleTypeFlagsKHr = 0x40;

pub type VkExternalMemoryFeatureFlagsKHR = VkFlags;
pub const VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_KHR: VkExternalMemoryFeatureFlagsKHR = 0x01;
pub const VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_KHR: VkExternalMemoryFeatureFlagsKHR = 0x02;
pub const VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_KHR: VkExternalMemoryFeatureFlagsKHr = 0x04;

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkExternalMemoryPropertiesKHR
{
    pub externalMemoryFeatures: VkExternalMemoryFatureFlagsKHR,
    pub exportFromImportedHandleTypes: VkExternalMemoryHandleTypeFlagsKHR,
    pub compatibleHandleTypes: VkExternalMemoryHandleTypeFlagsKHR
}
#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPhysicalDeviceExternalImageFormatInfoKHR
{
    pub sType: VkStructureType, pub pNext: *const c_void,
    pub handleType: VkExternalMemoryHandleTypeFlagsKHR
}
impl Default for VkPhysicalDeviceExternalImageFormatInfoKHR
{
    fn default() -> Self
    {
        VkPhysicalDeviceExternalImageFormatKHR
        {
            sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO_KHR,
            .. unsafe { std::mem::zeroed() }
        }
    }
}

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkExternalImageFormatPropertiesKHR
{
    pub sType: VkStructureType, pub pNext: *mut c_void,
    pub externalMemoryProperties: VkExternalMemoryPropertiesKHR
}
impl Default for VkExternalImageFormatPropertiesKHR
{
    fn default() -> Self
    {
        VkExternalImageFormatPropertiesKHR
        {
            sType: VK_STRUCTURE_TYPE_EXTERNAL_IMAGE_FORMAT_PROPERTIES_KHR,
            .. unsafe { std::mem::zeroed() }
        }
    }
}

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPhysicalDeviceExternalBufferInfoKHR
{
    pub sType: VkStructureType, pub pNext: *const c_void, pub flags: VkBufferCreateFlags,
    pub usage: VkBufferUsageFlags, pub handleType: VkExternalMemoryHandleTypeFlagsKHR
}
impl Default for VkPhysicalDeviceExternalBufferInfoKHR
{
    fn default() -> Self
    {
        VkPhysicalDeviceExternalBufferInfoKHR
        {
            sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO_KHR,
            .. unsafe { std::mem::zeroed() }
        }
    }
}

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkExternalBufferPropertiesKHR
{
    pub sType: VkStructureType, pub pNext: *mut c_void,
    pub externalMemoryProperties: VkExternalMemoryPropertiesKHR
}
impl Default for VkExternalBufferPropertiesKHR
{
    fn default() -> Self
    {
        VkExternalBufferPropertiesKHR
        {
            sType: VK_STRUCTURE_TYPE_EXTERNAL_BUFFER_PROPERTIES_KHR,
            .. unsafe { std::mem::zeroed() }
        }
    }
}

#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPhysicalDeviceIDPropertiesKHR
{
    pub sType: VkStructureType, pub pNext: *mut c_void,
    pub deviceUUID: [u8; VK_UUID_SIZE], pub driverUUID: [u8; VK_UUID_SIZE],
    pub deviceLUID: [u8; VK_LUID_SIZE_KHR], pub deviceNodeMask: u32, pub deviceLUIDValid: VkBool32
}
impl Default for VkPhysicalDeviceIDPropertiesKHR
{
    fn default() -> Self
    {
        VkPhysicalDeviceIDPropertiesKHR
        {
            sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES_KHR,
            .. unsafe { std::mem::zeroed() }
        }
    }
}

pub type PFN_vkGetPhysicalDeviceExternalBufferPropertiesKHR = extern "system" fn(physicalDevice: VkPhysicalDevice, pExternalBufferInfo: *const VkPhysicalDeviceExternalBufferInfoKHR, pExternalBufferProperties: *mut VkExternalBufferPropertiesKHR);

//! window system integration module

use {EngineResult, ApplicationState, Event, Size2};
use subsystem_layer::NativeInstance;
use interlude_vk_defs::{VkPhysicalDevice, VkSurfaceKHR};

pub trait NativeWindowBase: Sized
{
    /// Create a new native window
    fn new(size: &Size2, caption: &str, resizable: bool) -> EngineResult<Self>;
    /// Show this window
    fn show(&self);
    /// Flush window server commands
    fn flush(&self);

    /// Whether the server is presentable Vulkan surfaces
    fn can_vk_present(&self, adapter: VkPhysicalDevice, queue_family_index: u32) -> bool;
    /// Create a new Vulkan surface
    fn make_vk_surface(&self, instnace: &NativeInstance) -> EngineResult<VkSurfaceKHR>;

    /// Process messages from window system and event signalings
    fn process_events_and_messages(&self, events: &[&Event]) -> ApplicationState;
    /// Process messages from window system
    fn process_messages(&self) -> ApplicationState;
    /// Process messages from window system until the application receives an exit signal
    fn process_all_messages(&self) { while self.process_messages() == ApplicationState::Continue {} }
}

#[cfg(feature = "target_xlib")]  pub mod target_xlib;
#[cfg(feature = "target_win32")] pub mod target_win32;

// Platform dependent selection
#[cfg(feature = "target_xlib")]  pub use self::target_xlib::NativeWindowWithServer as PlatformWindowType;
#[cfg(feature = "target_win32")] pub use self::target_win32::NativeWindow as PlatformWindowType;

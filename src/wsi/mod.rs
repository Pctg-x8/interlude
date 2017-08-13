//! window system integration module

#[cfg(feature = "target_xlib")] pub mod target_xlib;

use interlude_vk_defs::{VkSurfaceKHR, VkPhysicalDevice};
use interlude_vk_funport::vkDestroySurfaceKHR;
use subsystem_layer::{NativeInstance, NativeHandleProvider};
use std::rc::Rc;
use {ApplicationState, Event, EngineResult, Size2};

/// Native Windows must satisfy following trait
pub trait NativeWindow : Sized
{
	fn new(size: &Size2, caption: &str, resizable: bool) -> EngineResult<Self>;
	fn show(&self);
	fn can_vk_present(&self, adapter: VkPhysicalDevice, queue_family_index: u32) -> bool;
	fn make_vk_surface(&self, instance: &NativeInstance) -> EngineResult<VkSurfaceKHR>;
	fn process_events_and_messages(&self, events: &[&Event]) -> ApplicationState;

	/// Process a message from window server, default implementation by process_events_and_messages and may be implemented more efficient way
	fn process_messages(&self) -> ApplicationState { self.process_events_and_messages(&[]) }
	/// Process all messages from window server and never returns until window has closed,
	/// default implementation by process_messages and may be implemented more efficient way
	fn process_all_messages(&self)
	{
		while self.process_messages() == ApplicationState::Continue {}
	}
}

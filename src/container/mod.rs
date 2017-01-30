// empty implementation for container-based automation(CI)

use std::hash::Hash;
use std::marker::PhantomData;
use {EngineResult, ApplicationState, Event, Size2, vk};
use std::rc::Rc;

// dummy
pub struct NativeWindow {}
pub struct NativeInput<N: Eq + Copy + Hash> { ph: PhantomData<N> }

impl NativeWindow
{
	pub fn new(_: &Size2, _: &str, _: bool) -> EngineResult<Self>
	{
		Ok(NativeWindow {})
	}
	pub fn process_messages(&self) -> ApplicationState { ApplicationState::Exited }
	pub fn process_all_messages(&self) {}
	pub fn process_events_and_messages(&self, _: &[&Event]) -> ApplicationState
	{
		ApplicationState::Exited
	}
	pub fn is_vk_presentation_support(&self, _: &vk::PhysicalDevice, _: u32) -> bool
	{
		false
	}
	pub fn make_vk_surface(&self, _: &Rc<vk::Instance>) -> EngineResult<vk::Surface>
	{
		unreachable!("make_vk_surface is not provided for container-based automation");
	}
}
impl<N: Eq + Copy + Hash> NativeInput<N>
{
	pub fn new() -> EngineResult<Self> { Ok(NativeInput { ph: PhantomData }) }
}

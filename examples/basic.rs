
extern crate interlude;
use interlude::*;

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum InputNames { A }

fn main()
{
	let engine = Engine::new("com.cterm2.interlude.examples.basic", 0x01, Some(std::env::current_dir().unwrap()), DeviceFeatures::new()).or_crash();
	let w = engine.create_render_window(&Size2(640, 480), "Basic Sample").or_crash();
	if let Ok(mut isw) = engine.input_system_ref().write()
	{
		isw.add_input(InputNames::A, InputType::Key(InputKeys::Character('z')));
	}
	engine.process_all_messages();
}

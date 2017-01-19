
extern crate interlude;
use interlude::*;

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum InputNames { }

fn main()
{
	let engine = EngineBuilder::<InputNames>::new("com.cterm2.interlude.examples.basic".into(), (0, 0, 1), "Basic Sample".into(), &Size2(640, 480))
		.asset_base(std::env::current_dir().unwrap().into()).launch().or_crash();
	
	engine.process_all_messages();
}

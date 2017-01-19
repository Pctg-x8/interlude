// Interlude: Common Traits

pub trait InternalExports
{
	type InternalT;
	fn get_internal(&self) -> &Self::InternalT;
}

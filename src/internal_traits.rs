// Interlude: Common Traits

pub trait InternalExports<InternalType>
{
	fn get_internal(&self) -> &InternalType;
}

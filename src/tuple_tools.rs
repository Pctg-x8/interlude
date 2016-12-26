
pub trait TupleFlatR<Ts, E>
{
	fn flatten(self) -> Result<Ts, E>;
}
impl<A, B, E> TupleFlatR<(A, B), E> for (Result<A, E>, Result<B, E>)
{
	fn flatten(self) -> Result<(A, B), E>
	{
		match self { (Ok(a), Ok(b)) => Ok((a, b)), (Err(e), _) | (_, Err(e)) => Err(e) }
	}
}
impl<A, B, C, E> TupleFlatR<(A, B, C), E> for (Result<A, E>, Result<B, E>, Result<C, E>)
{
	fn flatten(self) -> Result<(A, B, C), E>
	{
		match (self.0, self.1, self.2)
		{
			(Ok(a), Ok(b), Ok(c)) => Ok((a, b, c)),
			(Err(e), _, _) | (_, Err(e), _) | (_, _, Err(e)) => Err(e)
		}
	}
}
impl<A, B, C, D, E> TupleFlatR<(A, B, C, D), E> for (Result<A, E>, Result<B, E>, Result<C, E>, Result<D, E>)
{
	fn flatten(self) -> Result<(A, B, C, D), E>
	{
		match self
		{
			(Ok(a), Ok(b), Ok(c), Ok(d)) => Ok((a, b, c, d)),
			(Err(e), _, _, _) | (_, Err(e), _, _) | (_, _, Err(e), _) | (_, _, _, Err(e)) => Err(e)
		}
	}
}

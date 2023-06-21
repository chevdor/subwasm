/// This function reports whether the 2 runtimes APIs are compatible for the user.
/// Runtimes maybe not be compatible for instance if some Call signatures have changed.
///
/// See also [RequireTransactionVersionBump]
pub trait Compatible {
	fn compatible(&self) -> bool;
}

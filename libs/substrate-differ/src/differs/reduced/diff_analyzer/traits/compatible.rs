/// This function reports whether the 2 runtimes APIs are compatible for the user.
/// Runtimes maybe not be compatible for instance if the sisgnature of a call has changed.
pub trait Compatible {
	fn compatible(&self) -> bool;
}

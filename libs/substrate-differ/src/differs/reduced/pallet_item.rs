use super::calls::{call::*, constant::*, error::*, event::*, storage::*};
use comparable::Comparable;
use std::fmt::Display;

#[derive(Debug, Comparable)]
/// A [PalletItem] is what [ReducedRuntime](super::reduced_runtime::ReducedRuntime) are made of.
pub enum PalletItem {
	Call(Call),
	Event(Event),
	Error(Error),
	Storage(Storage),
	Constant(Constant),
}

impl Display for PalletItem {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		const WIDTH: usize = 9;
		match self {
			PalletItem::Call(c) => f.write_fmt(format_args!("{:<WIDTH$}: {c}", "Call")),
			PalletItem::Event(e) => f.write_fmt(format_args!("{:<WIDTH$}: {e}", "Event")),
			PalletItem::Error(e) => f.write_fmt(format_args!("{:<WIDTH$}: {e}", "Error")),
			PalletItem::Constant(c) => f.write_fmt(format_args!("{:<WIDTH$}: {c}", "Constant")),
			PalletItem::Storage(s) => f.write_fmt(format_args!("{:<WIDTH$}: {s}", "Storage")),
		}
	}
}

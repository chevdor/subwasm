use comparable::Comparable;
use frame_metadata::PalletCallMetadata;
use scale_info::form::PortableForm;
use std::fmt::Display;

use super::calls::{call::*, constant::*, error::*, event::*, storage::*};

#[derive(Debug, PartialEq, Eq, Hash, Comparable, PartialOrd, Ord)]
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
		match self {
			PalletItem::Call(c) => f.write_fmt(format_args!("{}", c)),
			PalletItem::Event(e) => f.write_fmt(format_args!("{}", e)),
			PalletItem::Error(e) => f.write_fmt(format_args!("{}", e)),
			PalletItem::Storage(s) => f.write_fmt(format_args!("{}", s)),
			PalletItem::Constant(c) => f.write_fmt(format_args!("{}", c)),
		}
	}
}

impl From<&PalletCallMetadata<PortableForm>> for Call {
	fn from(_: &PalletCallMetadata<PortableForm>) -> Self {
		todo!()
	}
}

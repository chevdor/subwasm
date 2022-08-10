use frame_metadata::PalletCallMetadata;
use scale_info::form::PortableForm;
use std::fmt::Display;

use super::call::{self, *};

#[derive(Debug, PartialEq)]
/// Content of a Reduced runtime
pub enum PalletItem {
	// Call(PalletData),
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
			_ => todo!(),
		}
	}
}

impl From<&PalletCallMetadata<PortableForm>> for call::Call {
	fn from(_: &PalletCallMetadata<PortableForm>) -> Self {
		todo!()
	}
}

use serde::Serialize;
use std::fmt::Display;

use super::reduced_pallet::*;

#[derive(Debug, Serialize)]
pub struct ReducedPalletChangeWrapper<'a> {
	pub(crate) changes: &'a ReducedPalletChange,

	// The following are `Option` since one of them could be `None`
	// in the cases where a pallet was newly introduced or removed.
	pub(crate) pallet_a: Option<&'a ReducedPallet>,
	pub(crate) pallet_b: Option<&'a ReducedPallet>,
}

impl<'a> ReducedPalletChangeWrapper<'a> {
	pub fn new(
		changes: &'a ReducedPalletChange,
		pallet_a: Option<&'a ReducedPallet>,
		pallet_b: Option<&'a ReducedPallet>,
	) -> Self {
		Self { changes, pallet_a, pallet_b }
	}

	// /// Return, if found, a call from our [ReducedPallet].
	// fn get_call(&self, id: &ExtrinsicId, side: ComparisonSide) -> Option<&Call> {
	// 	let reduced_pallet = match side {
	// 		ComparisonSide::Left => &self.pallet_a,
	// 		ComparisonSide::Right => &self.pallet_b,
	// 	};
	// 	match reduced_pallet {
	// 		Some(pallet) => pallet.calls.get(id),
	// 		None => None,
	// 	}
	// }
}

/// This macro helps formatting changes for a given pallet field.
macro_rules! fmt_vec_changes {
	( $self:ident, $f:ident, $field:ident, $changes:ident ) => {{
		writeln!($f, "  - {} changes:", stringify!($field))?;
		let get_a = |id| $self.pallet_a.and_then(|p| p.$field.get(id));
		let get_b = |id| $self.pallet_b.and_then(|p| p.$field.get(id));
		for change in $changes {
			let _ = match change {
				comparable::MapChange::Added(_id, desc) => {
					writeln!($f, "    [+] {}", desc)?;
				}
				comparable::MapChange::Changed(id, changes) => {
					let item_a = get_a(id).map(|i| i.to_string()).unwrap_or_default();
					let item_b = get_b(id).map(|i| i.to_string()).unwrap_or_default();
					let indent: usize = 4;
					writeln!($f, "{:indent$}[≠] OLD: {item_a:<20}", " ",)?;
					writeln!($f, "{:indent$}    NEW: {item_b:<20}", " ",)?;
					for change in changes {
						writeln!($f, "{:indent$}    CHANGES: {change}", " ")?;
					}
				}
				comparable::MapChange::Removed(id) => {
					let item_a_name = get_a(id).map(|c| c.name.as_str()).unwrap_or("n/a");
					writeln!($f, "    [-] {}", item_a_name)?;
				}
			};
		}
		Ok(())
	}};
}

impl<'a> Display for ReducedPalletChangeWrapper<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self.changes {
			ReducedPalletChange::Index(c) => writeln!(f, "index: {c:?}"),
			ReducedPalletChange::Name(c) => writeln!(f, "name: {c:?}"),

			ReducedPalletChange::Calls(c) => fmt_vec_changes!(self, f, calls, c),
			ReducedPalletChange::Events(c) => fmt_vec_changes!(self, f, events, c),
			ReducedPalletChange::Errors(c) => fmt_vec_changes!(self, f, errors, c),

			ReducedPalletChange::Constants(c) => fmt_vec_changes!(self, f, constants, c),
			ReducedPalletChange::Storages(c) => fmt_vec_changes!(self, f, storages, c),
		}
	}
}

use std::{cmp::Ordering, fmt::Display};

use crate::differs::reduced::change_type::Change;

use super::{calls::prelude::Index, diff_result::DiffResult, pallet_item::PalletItem};
use comparable::Comparable;
use frame_metadata::PalletMetadata;
use scale_info::form::PortableForm;

/// A [ReducedPallet] is mainly a `Vec` or [PalletItem].
#[derive(Debug, PartialEq, Eq, Hash, Comparable)]
pub struct ReducedPallet {
	/// Index of the pallet
	pub index: Index,

	/// Name of the pallet
	pub name: String,

	/// Vec of all the `PalletItem`
	pub items: Vec<PalletItem>,
	// TODO: no doc ?
}

impl PartialOrd for ReducedPallet {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		match self.index.partial_cmp(&other.index) {
			a @ Some(core::cmp::Ordering::Equal) => a,
			ord => return ord,
		}
		// match self.name.partial_cmp(&other.name) {
		//     Some(core::cmp::Ordering::Equal) => {}
		//     ord => return ord,
		// }
		// self.items.partial_cmp(&other.items)
	}
}

// TODO: impl Iterator
impl ReducedPallet {
	/// Computes the differences between 2 pallets
	pub fn diff<'meta>(
		pallet_a: Option<&'meta Self>,
		pallet_b: Option<&'meta Self>,
	) -> DiffResult<'meta, ReducedPallet> {
		match (pallet_a, pallet_b) {
			(Some(pa), Some(pb)) => {
				// Compare indexes... well kinda...
				assert_eq!(pa.index, pb.index, "Comparing different indexes does not make much sense");

				// Compare names
				if pa.name != pb.name {
					return DiffResult::new(Change::Modified((pa, pb)));
				}

				// Compare items, this is the most important
				if pa.items != pb.items {
					return DiffResult::new(Change::Modified((pa, pb)));
				}
				DiffResult::new(Change::Unchanged)
			}
			(Some(pa), None) => return DiffResult::new(Change::Removed(pa)),
			(None, Some(pb)) => return DiffResult::new(Change::Added(pb)),
			(None, None) => unreachable!(),
		}
	}
}

impl Display for ReducedPallet {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let _ = f.write_fmt(format_args!("pallet #{} {}\n", self.index, self.name));

		self.items.iter().for_each(|pallet_item| {
			let _ = f.write_fmt(format_args!("  - {}\n", pallet_item));
		});

		Ok(())
	}
}

impl From<&PalletMetadata<PortableForm>> for ReducedPallet {
	fn from(pallet: &PalletMetadata<PortableForm>) -> Self {
		let index: Index = pallet.index.into();
		let name = pallet.name.to_string();
		// let items: Vec<PalletItem> = Vec::new();
		// todo!("You are here :)");

		// let calls: Vec<PalletItem> = pallet.calls.as_ref().map(|call| call.into()).unwrap();
		// let registry = pallet.into()

		// not the rigth approach
		// let pallet_meta : PalletMetadata<MetaForm> = pallet.into();

		// let events: Vec<PalletItem> = pallet.event.as_ref().map(|call| call.into()).unwrap();
		// let errors: Vec<PalletItem> = pallet.error.as_ref().map(|call| call.into()).unwrap();
		// let storages: Vec<PalletItem> = pallet.storage.as_ref().map(|call| call.into()).unwrap();
		// let constants: Vec<PalletItem> = pallet.constants.as_ref().map(|call| call.into()).unwrap();
		// TODO: Add others as well
		// let items = calls;
		let items = vec![];

		// let items = Some(calls); // TODO:
		// Self { index, name, items }
		Self { index, name, items }
	}
}

#[cfg(test)]
impl Default for ReducedPallet {
	fn default() -> Self {
		Self { index: 42, name: "Foobar".into(), items: vec![] }
	}
}

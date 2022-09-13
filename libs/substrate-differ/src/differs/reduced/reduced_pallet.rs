use super::calls::{prelude::Index, *};
use comparable::Comparable;
use frame_metadata::PalletMetadata;
use scale_info::form::PortableForm;
use std::{collections::BTreeMap, fmt::Display};

/// A [ReducedPallet] is mainly a `Vec` or [PalletItem].
// TODO: no doc ?
#[derive(Debug, PartialEq, Eq, Hash, Comparable)]
pub struct ReducedPallet {
	/// Index of the pallet
	pub index: Index,

	/// Name of the pallet
	pub name: String,

	pub calls: BTreeMap<Index, Call>,
	pub events: BTreeMap<Index, Event>,
	pub errors: BTreeMap<Index, Error>,

	pub constants: BTreeMap<String, Constant>,
	pub storages: BTreeMap<String, Storage>,
}

impl PartialOrd for ReducedPallet {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		self.index.partial_cmp(&other.index)
	}
}

// TODO: impl Iterator
impl ReducedPallet {
	// /// Computes the differences between 2 pallets
	// pub fn diff<'meta>(
	// 	pallet_a: Option<&'meta Self>,
	// 	pallet_b: Option<&'meta Self>,
	// ) -> DiffResult<'meta, ReducedPallet> {
	// 	match (pallet_a, pallet_b) {
	// 		(Some(pa), Some(pb)) => {
	// 			// Compare indexes... well kinda...
	// 			assert_eq!(pa.index, pb.index, "Comparing different indexes does not make much sense");

	// 			// Compare names
	// 			if pa.name != pb.name {
	// 				return DiffResult::new(Change::Modified((pa, pb)));
	// 			}

	// 			// Compare items, this is the most important
	// 			// TODO: this check goes away, we switch to comparable
	// 			if pa.calls != pb.calls {
	// 				return DiffResult::new(Change::Modified((pa, pb)));
	// 			}
	// 			DiffResult::new(Change::Unchanged)
	// 		}
	// 		(Some(pa), None) => return DiffResult::new(Change::Removed(pa)),
	// 		(None, Some(pb)) => return DiffResult::new(Change::Added(pb)),
	// 		(None, None) => unreachable!(),
	// 	}
	// }
}

impl Display for ReducedPallet {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let _ = f.write_fmt(format_args!("pallet #{} {}\n", self.index, self.name));

		// TODO: Show more than the calls
		self.calls.iter().for_each(|(index, call)| {
			let _ = f.write_fmt(format_args!("  - {} {}\n", index, call));
		});

		Ok(())
	}
}

impl From<&PalletMetadata<PortableForm>> for ReducedPallet {
	fn from(pallet: &PalletMetadata<PortableForm>) -> Self {
		let index: Index = pallet.index.into();
		let name = pallet.name.to_string();

		Self { index, name, ..Default::default() }
	}
}

impl Default for ReducedPallet {
	fn default() -> Self {
		Self {
			index: 42,
			name: String::new(),
			calls: BTreeMap::new(),
			events: BTreeMap::new(),
			errors: BTreeMap::new(),
			constants: BTreeMap::new(),
			storages: BTreeMap::new(),
		}
	}
}

pub enum PalletItemType {
	Call,
	Event,
	Error,

	Constant,
	Storage,
}

impl Display for PalletItemType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			PalletItemType::Call => f.write_str("Call"),
			PalletItemType::Event => f.write_str("Event"),
			PalletItemType::Error => f.write_str("Error"),
			PalletItemType::Constant => f.write_str("Constant"),
			PalletItemType::Storage => f.write_str("Storage"),
		}
	}
}

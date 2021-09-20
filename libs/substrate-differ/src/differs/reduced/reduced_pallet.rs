use super::{diff_result::DiffResult, pallet_item::PalletItem, Index};
use crate::differs::{raw::change_type::ChangeType, utils::convert};
use frame_metadata::{v13, v14};

#[derive(Debug, PartialEq)]
pub struct ReducedPallet {
	/// Index of the pallet
	pub index: Index,

	/// Name of the pallet
	pub name: String,

	/// Vec of all the `PalletItem`
	items: Vec<PalletItem>,
}

// TODO: impl Iterator
impl ReducedPallet {
	/// Computes the differences between 2 pallets
	pub fn diff(pallet_a: &'static Self, pallet_b: &'static Self) -> DiffResult<ReducedPallet> {
		assert_eq!(pallet_a.index, pallet_b.index, "Comparing different indexes does not make much sense");

		if pallet_a.name != pallet_b.name {
			return DiffResult::new(ChangeType::Modified, pallet_a, pallet_b);
		}

		if pallet_a.items != pallet_b.items {
			return DiffResult::new(ChangeType::Modified, pallet_a, pallet_b);
		}

		DiffResult::new(ChangeType::Unchanged, pallet_a, pallet_b)
	}
}

#[cfg(test)]
impl Default for ReducedPallet {
	fn default() -> Self {
		Self { index: 42, name: "Foobar".into(), items: vec![] }
	}
}

impl From<&v13::ModuleMetadata> for ReducedPallet {
	fn from(v13: &v13::ModuleMetadata) -> Self {
		let index = v13.index.into();
		let name = convert(&v13.name).to_string();
		let mut items: Vec<PalletItem> = Vec::new();

		// Calls
		let calls = match &v13.calls.as_ref() {
			Some(items) => {
				let pallet_items: Vec<PalletItem> = convert(items).iter().map(|c| c.into()).collect();
				Some(pallet_items)
			}
			None => None,
		};

		if let Some(mut c) = calls {
			// println!("calls = {:?}", c.len());
			items.append(&mut c);
		}
		// Events
		let events = match &v13.event.as_ref() {
			Some(items) => {
				let pallet_items: Vec<PalletItem> = convert(items).iter().map(|c| c.into()).collect();
				Some(pallet_items)
			}
			None => None,
		};

		if let Some(mut c) = events {
			// println!("events = {:?}", c.len());
			items.append(&mut c);
		}

		// Storage
		let storage = match &v13.storage.as_ref() {
			Some(items) => {
				// let pallet_items: Vec<PalletItem> = convert(items).iter().map(|c| c.into()).collect();
				let pallet_items: Vec<PalletItem> = convert(&convert(items).entries).iter().map(|c| c.into()).collect();
				Some(pallet_items)
			}
			None => None,
		};

		if let Some(mut c) = storage {
			// println!("storage = {:?}", c.len());
			items.append(&mut c);
		}

		// Errors
		let mut errors: Vec<PalletItem> = convert(&v13.errors).iter().map(|c| c.into()).collect();
		// println!("errors = {:?}", errors.len());
		items.append(&mut errors);

		// Constants
		let mut constants: Vec<PalletItem> = convert(&v13.constants).iter().map(|c| c.into()).collect();
		// println!("constants = {:?}", constants.len());
		items.append(&mut constants);

		// let items = if items.is_empty() { None } else { Some(items) };

		Self { index, name, items }
	}
}

impl From<&v14::PalletMetadata> for ReducedPallet {
	fn from(pallet: &v14::PalletMetadata) -> Self {
		let _index: Index = pallet.index.into();
		let _name = pallet.name.to_string();
		let _content: Vec<PalletItem> = Vec::new();

		todo!("You are here :)");
		// let calls = pallet.calls.as_ref().map(|call| call.into()).collect();

		// let items = Some(calls); // TODO:
		// Self { index, name, items }
	}
}

use super::calls::*;
use comparable::Comparable;
use frame_metadata::v14::PalletMetadata;
use serde::Serialize;
use std::{collections::BTreeMap, fmt::Display};

/// A [ReducedPallet] could be a `Vec` or [PalletItem] but it ends
/// but providing a much more useful output after diffing when using
/// separated fields.
#[derive(Debug, PartialEq, Hash, Comparable, Serialize, Clone)]
pub struct ReducedPallet {
	/// Index of the pallet
	pub index: PalletId,

	/// Name of the pallet
	pub name: String,

	/// Storage prefix.
	pub storage_prefix: String,

	pub calls: BTreeMap<PalletId, Call>,
	pub events: BTreeMap<PalletId, Event>,
	pub errors: BTreeMap<PalletId, Error>,

	pub constants: BTreeMap<String, Constant>,
	pub storages: BTreeMap<String, Storage>,
}

impl PartialOrd for ReducedPallet {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		self.index.partial_cmp(&other.index)
	}
}

macro_rules! display_pallet_items {
	( $self:ident, $f:ident, $field:ident ) => {{
		if !$self.$field.is_empty() {
			let _ = $f.write_fmt(format_args!("  {}:\n", stringify!($field)));
			$self.$field.iter().for_each(|(_id, item)| {
				let _ = $f.write_fmt(format_args!("    - {item}\n"));
			});
		}
	}};
}

impl Display for ReducedPallet {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let _ = f.write_fmt(format_args!("Pallet #{}: {}\n", self.index, self.name));

		display_pallet_items!(self, f, calls);
		display_pallet_items!(self, f, events);
		display_pallet_items!(self, f, errors);
		display_pallet_items!(self, f, constants);
		display_pallet_items!(self, f, storages);
		Ok(())
	}
}

impl From<&PalletMetadata<PortableForm>> for ReducedPallet {
	fn from(pallet: &PalletMetadata<PortableForm>) -> Self {
		let index: PalletId = pallet.index.into();
		let name = pallet.name.to_string();

		Self { index, name, ..Default::default() }
	}
}

impl Default for ReducedPallet {
	fn default() -> Self {
		Self {
			index: 42,
			name: String::new(),
			storage_prefix: String::new(),
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

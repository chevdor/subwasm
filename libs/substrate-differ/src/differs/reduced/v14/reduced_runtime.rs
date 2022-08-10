use crate::differs::reduced::{pallet_data::PalletData, reduced_pallet::ReducedPallet};
use frame_metadata::v14;
use std::fmt::Debug;

impl From<&v14::PalletCallMetadata> for PalletData {
	fn from(f: &v14::PalletCallMetadata) -> Self {
		let meta_type = f.ty;
		let _ti = meta_type.type_info();

		let _index = meta_type.type_id();
		let _name = String::new();

		todo!();
		// PalletData::new(name, index, signature, documentation)
	}
}

#[derive(Debug, PartialEq)]
pub struct ReducedRuntime {
	// TODO: remove pub once we have an iterator
	pub pallets: Vec<ReducedPallet>, // TODO: Could use a BTreeMap
}

// impl SliceIndex<[ReducedPallet] for u32 {

// }

impl From<Vec<ReducedPallet>> for ReducedRuntime {
	fn from(pallets: Vec<ReducedPallet>) -> Self {
		Self { pallets }
	}
}

#[cfg(test)]
mod test_reduced_conversion {
	use crate::differs::reduced::{pallet_item::PalletItem, reduced_runtime};

	use super::*;
	use frame_metadata::RuntimeMetadata;
	use std::path::PathBuf;
	use wasm_loader::Source;
	use wasm_testbed::WasmTestBed;

	const RUNTIME_V14: &str = "../../data/polkadot/V14/polkadot_runtime.compact.compressed.wasm";

	#[test]
	fn test_reduce_v14() {
		let testbed = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_V14))).unwrap();
		let metadata = testbed.metadata();

		match metadata {
			RuntimeMetadata::V14(v14) => {
				let rrtm = reduced_runtime::ReducedRuntime::from_v14(v14).unwrap();
				// println!("rrtm = {:#?}", rrtm);

				assert_eq!(rrtm.pallets.len(), 30);

				let first_pallet = &rrtm.pallets[0];
				assert_eq!(0, first_pallet.index);
				assert_eq!("System", first_pallet.name);

				println!(" first_pallet.items = {:#?}", first_pallet.items);
				assert_eq!(45, first_pallet.items.len());
				match &first_pallet.items[1] {
					PalletItem::Call(c) => assert_eq!("remark", c.name),
					_ => unreachable!(),
				}
			}
			_ => unreachable!(),
		}
	}
}

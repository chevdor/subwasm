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

#[derive(Debug, PartialEq, Eq)]
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
	use frame_metadata::RuntimeMetadata;
	use std::path::PathBuf;
	use wasm_loader::Source;
	use wasm_testbed::WasmTestBed;

	// check with:
	// subwasm meta data/polkadot/V14/polkadot_runtime.compact.compressed.wasm -m system
	const RUNTIME_V14: &str = "../../data/polkadot/V14/polkadot_runtime.compact.compressed.wasm";

	#[test]
	#[ignore = "local data"]
	#[cfg(feature = "v14")]
	fn test_reduce_v14_global() {
		let testbed = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_V14))).unwrap();
		let metadata = testbed.metadata();

		match metadata {
			RuntimeMetadata::V14(v14) => {
				let rrtm = reduced_runtime::ReducedRuntime::from_v14(v14).unwrap();
				// println!("rrtm = {:#?}", rrtm);

				assert_eq!(rrtm.pallets.len(), 30);

				let first_pallet = &rrtm.pallets[&0];
				assert_eq!(0, first_pallet.index);
				assert_eq!("System", first_pallet.name);
				// println!(" first_pallet.items = {:#?}", first_pallet.items);
				assert_eq!(43, first_pallet.items.len());

				match &first_pallet.items[1] {
					PalletItem::Call(c) => assert_eq!("remark", c.name),
					_ => unreachable!(),
				}
			}
			_ => unreachable!(),
		}
	}

	#[test]
	#[ignore = "local data"]
	#[cfg(feature = "v14")]
	fn test_reduce_v14_calls() {
		let testbed = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_V14))).unwrap();
		let metadata = testbed.metadata();

		match metadata {
			RuntimeMetadata::V14(v14) => {
				let rrtm = reduced_runtime::ReducedRuntime::from_v14(v14).unwrap();
				// println!("rrtm = {:#?}", rrtm);

				assert_eq!(rrtm.pallets.len(), 30);

				let first_pallet = &rrtm.pallets[&0];
				assert_eq!(0, first_pallet.index);
				assert_eq!("System", first_pallet.name);

				// Check calls
				let calls = first_pallet.items.iter().filter(|&p| matches!(p, PalletItem::Call(_)));
				assert_eq!(10, calls.count());
			}
			_ => unreachable!(),
		}
	}

	#[test]
	#[ignore = "local data"]
	#[cfg(feature = "v14")]
	fn test_reduce_v14_events() {
		let testbed = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_V14))).unwrap();
		let metadata = testbed.metadata();

		match metadata {
			RuntimeMetadata::V14(v14) => {
				let rrtm = reduced_runtime::ReducedRuntime::from_v14(v14).unwrap();
				// println!("rrtm = {:#?}", rrtm);

				assert_eq!(rrtm.pallets.len(), 30);

				let first_pallet = &rrtm.pallets[&0];
				assert_eq!(0, first_pallet.index);
				assert_eq!("System", first_pallet.name);

				// Check events
				let events = first_pallet.items.iter().filter(|&p| matches!(p, PalletItem::Event(_)));
				assert_eq!(6, events.count());
			}
			_ => unreachable!(),
		}
	}

	#[test]
	#[ignore = "local data"]
	#[cfg(feature = "v14")]
	fn test_reduce_v14_errors() {
		let testbed = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_V14))).unwrap();
		let metadata = testbed.metadata();

		match metadata {
			RuntimeMetadata::V14(v14) => {
				let rrtm = reduced_runtime::ReducedRuntime::from_v14(v14).unwrap();
				// println!("rrtm = {:#?}", rrtm);

				assert_eq!(rrtm.pallets.len(), 30);

				let first_pallet = &rrtm.pallets[&0];
				assert_eq!(0, first_pallet.index);
				assert_eq!("System", first_pallet.name);
				// println!(" first_pallet.items = {:#?}", first_pallet.items);
				assert_eq!(43, first_pallet.items.len());

				// Check errors
				let errors = first_pallet.items.iter().filter(|&p| matches!(p, PalletItem::Error(_)));
				assert_eq!(5, errors.count());
				match &first_pallet.items[1] {
					PalletItem::Call(c) => assert_eq!("remark", c.name),
					_ => unreachable!(),
				}
			}
			_ => unreachable!(),
		}
	}

	#[test]
	#[ignore = "local data"]
	#[cfg(feature = "v14")]
	fn test_reduce_v14_storages() {
		let testbed = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_V14))).unwrap();
		let metadata = testbed.metadata();

		match metadata {
			RuntimeMetadata::V14(v14) => {
				let rrtm = reduced_runtime::ReducedRuntime::from_v14(v14).unwrap();
				// println!("rrtm = {:#?}", rrtm);

				assert_eq!(rrtm.pallets.len(), 30);

				let first_pallet = &rrtm.pallets[&0];
				assert_eq!(0, first_pallet.index);
				assert_eq!("System", first_pallet.name);

				// Check storages
				let storages = first_pallet.items.iter().filter(|&p| matches!(p, PalletItem::Storage(_)));
				assert_eq!(16, storages.count());
			}
			_ => unreachable!(),
		}
	}

	#[test]
	#[ignore = "local data"]
	#[cfg(feature = "v14")]
	fn test_reduce_v14_constants() {
		let testbed = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_V14))).unwrap();
		let metadata = testbed.metadata();

		match metadata {
			RuntimeMetadata::V14(v14) => {
				let rrtm = reduced_runtime::ReducedRuntime::from_v14(v14).unwrap();
				// println!("rrtm = {:#?}", rrtm);

				assert_eq!(rrtm.pallets.len(), 30);

				let first_pallet = &rrtm.pallets[&0];
				assert_eq!(0, first_pallet.index);
				assert_eq!("System", first_pallet.name);

				// Check constants
				let constants = first_pallet.items.iter().filter(|&p| matches!(p, PalletItem::Constant(_)));
				assert_eq!(6, constants.count());
			}
			_ => unreachable!(),
		}
	}
}

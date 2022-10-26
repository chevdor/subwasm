use crate::differs::reduced::{pallet_data::PalletData, reduced_pallet::ReducedPallet};
use frame_metadata::v14;
use std::fmt::Debug;

impl From<&v14::PalletCallMetadata> for PalletData {
	fn from(f: &v14::PalletCallMetadata) -> Self {
		let meta_type = f.ty;
		let _ti = meta_type.type_info();

		let _index = meta_type.type_id();
		let _name = String::new();

		todo!("V14");
		// PalletData::new(name, index, signature, documentation)
	}
}

#[derive(Debug, PartialEq)]
pub struct ReducedRuntime {
	pub pallets: Vec<ReducedPallet>,
}

impl From<Vec<ReducedPallet>> for ReducedRuntime {
	fn from(pallets: Vec<ReducedPallet>) -> Self {
		Self { pallets }
	}
}

#[cfg(test)]
mod test_reduced_conversion {
	use crate::differs::reduced::reduced_runtime;
	use frame_metadata::RuntimeMetadata;
	use std::path::PathBuf;
	use wasm_loader::Source;
	use wasm_testbed::WasmTestBed;

	// check with:
	// subwasm meta data/polkadot/V14/polkadot_runtime.compact.compressed.wasm -m system
	// const RUNTIME_V14: &str = "../../data/polkadot/V14/polkadot_runtime.compact.compressed.wasm";

	#[test]
	#[ignore = "local data"]
	#[cfg(feature = "v14")]
	fn test_reduce_v14_first_pallet_first_call() {
    use crate::differs::test_runtimes::{get_runtime_file, Chain};
		let rtm1 = get_runtime_file(Chain::Polkadot, 14, 9260).expect("Runtime file should exist");
		let testbed = WasmTestBed::new(&Source::File(PathBuf::from(rtm1))).unwrap();
		let metadata = testbed.metadata();

		match metadata {
			RuntimeMetadata::V14(v14) => {
				let rrtm = reduced_runtime::ReducedRuntime::from_v14(v14).unwrap();
				assert_eq!(rrtm.pallets.len(), 51);

				let first_pallet = &rrtm.pallets[&0];
				assert_eq!(0, first_pallet.index);
				assert_eq!("System", first_pallet.name);
				assert_eq!(9, first_pallet.calls.len());
				assert_eq!("remark", &first_pallet.calls.get(&1).unwrap().name);
			}
			_ => unreachable!(),
		}
	}

	#[test]
	#[ignore = "local data"]
	#[cfg(feature = "v14")]
	fn test_reduce_v14_calls() {
    use crate::differs::test_runtimes::{get_runtime_file, Chain};
		let rtm1 = get_runtime_file(Chain::Polkadot, 14, 9260).expect("Runtime file should exist");

		let testbed = WasmTestBed::new(&Source::File(PathBuf::from(rtm1))).unwrap();
		let metadata = testbed.metadata();

		match metadata {
			RuntimeMetadata::V14(v14) => {
				let rrtm = reduced_runtime::ReducedRuntime::from_v14(v14).unwrap();
				assert_eq!(rrtm.pallets.len(), 51);

				let first_pallet = &rrtm.pallets[&0];
				assert_eq!(0, first_pallet.index);
				assert_eq!("System", first_pallet.name);

				assert_eq!(9, first_pallet.calls.len());
			}
			_ => unreachable!(),
		}
	}

	#[test]
	#[ignore = "local data"]
	#[cfg(feature = "v14")]
	fn test_reduce_v14_events() {
    use crate::differs::test_runtimes::{get_runtime_file, Chain};

		let rtm1 = get_runtime_file(Chain::Polkadot, 14, 9260).expect("Runtime file should exist");
		let testbed = WasmTestBed::new(&Source::File(PathBuf::from(rtm1))).unwrap();
		let metadata = testbed.metadata();

		match metadata {
			RuntimeMetadata::V14(v14) => {
				let rrtm = reduced_runtime::ReducedRuntime::from_v14(v14).unwrap();
				assert_eq!(rrtm.pallets.len(), 51);

				let first_pallet = &rrtm.pallets[&0];
				assert_eq!(0, first_pallet.index);
				assert_eq!("System", first_pallet.name);

				assert_eq!(6, first_pallet.events.len());
			}
			_ => unreachable!(),
		}
	}

	#[test]
	#[ignore = "local data"]
	#[cfg(feature = "v14")]
	fn test_reduce_v14_global() {
    use crate::differs::test_runtimes::{get_runtime_file, Chain};
		let rtm1 = get_runtime_file(Chain::Polkadot, 14, 9260).expect("Runtime file should exist");
		let testbed = WasmTestBed::new(&Source::File(PathBuf::from(rtm1))).unwrap();
		let metadata = testbed.metadata();

		match metadata {
			RuntimeMetadata::V14(v14) => {
				let rrtm = reduced_runtime::ReducedRuntime::from_v14(v14).unwrap();
				assert_eq!(rrtm.pallets.len(), 51);

				let first_pallet = &rrtm.pallets[&0];
				assert_eq!(0, first_pallet.index);
				assert_eq!("System", first_pallet.name);

				assert_eq!(9, first_pallet.calls.len());
				assert_eq!(6, first_pallet.events.len());
				assert_eq!(6, first_pallet.errors.len());
				assert_eq!(6, first_pallet.constants.len());
				assert_eq!(16, first_pallet.storages.len());

				assert_eq!("remark", &first_pallet.calls.get(&1).unwrap().name)
			}
			_ => unreachable!(),
		}
	}

	#[test]
	#[ignore = "local data"]
	#[cfg(feature = "v14")]
	fn test_reduce_v14_storages() {
    use crate::differs::test_runtimes::{get_runtime_file, Chain};
		let rtm1 = get_runtime_file(Chain::Polkadot, 14, 9260).expect("Runtime file should exist");
		let testbed = WasmTestBed::new(&Source::File(PathBuf::from(rtm1))).unwrap();
		let metadata = testbed.metadata();

		match metadata {
			RuntimeMetadata::V14(v14) => {
				let rrtm = reduced_runtime::ReducedRuntime::from_v14(v14).unwrap();
				assert_eq!(rrtm.pallets.len(), 51);

				let first_pallet = &rrtm.pallets[&0];
				assert_eq!(0, first_pallet.index);
				assert_eq!("System", first_pallet.name);

				assert_eq!(16, first_pallet.storages.len());
			}
			_ => unreachable!(),
		}
	}

	#[test]
	#[ignore = "local data"]
	#[cfg(feature = "v14")]
	fn test_reduce_v14_constants() {
    use crate::differs::test_runtimes::{get_runtime_file, Chain};
		let rtm1 = get_runtime_file(Chain::Polkadot, 14, 9260).expect("Runtime file should exist");
		let testbed = WasmTestBed::new(&Source::File(PathBuf::from(rtm1))).unwrap();
		let metadata = testbed.metadata();

		match metadata {
			RuntimeMetadata::V14(v14) => {
				let rrtm = reduced_runtime::ReducedRuntime::from_v14(v14).unwrap();
				assert_eq!(rrtm.pallets.len(), 51);

				let first_pallet = &rrtm.pallets[&0];
				assert_eq!(0, first_pallet.index);
				assert_eq!("System", first_pallet.name);

				assert_eq!(6, first_pallet.constants.len());
			}
			_ => unreachable!(),
		}
	}
}

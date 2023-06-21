#[cfg(feature = "v13")]
use crate::differs::reduced::pallet_data::PalletData;

#[cfg(test)]
mod test_reduced_conversion {
	use crate::differs::{
		reduced::reduced_runtime,
		test_runtimes::{Chain, RuntimeFile},
	};
	use frame_metadata::RuntimeMetadata;
	use wasm_loader::Source;
	use wasm_testbed::WasmTestBed;

	#[test]
	#[ignore = "local data"]
	#[cfg(feature = "v14")]
	fn test_reduce_v14_first_pallet_first_call() {
		let rtm1 = RuntimeFile::new(Chain::Polkadot, 14, 9260).try_into().expect("Runtime file should exist");
		let testbed = WasmTestBed::new(&Source::File(rtm1)).expect("Failed loading runtime");
		let metadata = testbed.metadata();

		match metadata {
			RuntimeMetadata::V14(v14) => {
				let rrtm = reduced_runtime::ReducedRuntime::from_v14(&v14).expect("Failed reducing runtime");
				assert_eq!(rrtm.pallets.len(), 51);

				let first_pallet = &rrtm.pallets[&0];
				assert_eq!(0, first_pallet.index);
				assert_eq!("System", first_pallet.name);
				assert_eq!(9, first_pallet.calls.len());
				assert_eq!("remark", &first_pallet.calls.get(&1).expect("Failed getting call").name);
			}
			_ => unreachable!(),
		}
	}

	#[test]
	#[ignore = "local data"]
	#[cfg(feature = "v14")]
	fn test_reduce_v14_calls() {
		let rtm1 = RuntimeFile::new(Chain::Polkadot, 14, 9260).try_into().expect("Runtime file should exist");

		let testbed = WasmTestBed::new(&Source::File(rtm1)).expect("Failed loading runtime");
		let metadata = testbed.metadata();

		match metadata {
			RuntimeMetadata::V14(v14) => {
				let rrtm = reduced_runtime::ReducedRuntime::from_v14(&v14).expect("Failed reducing runtime");
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
		let rtm1 = RuntimeFile::new(Chain::Polkadot, 14, 9260).try_into().expect("Runtime file should exist");
		let testbed = WasmTestBed::new(&Source::File(rtm1)).expect("Failed loading runtime");
		let metadata = testbed.metadata();

		match metadata {
			RuntimeMetadata::V14(v14) => {
				let rrtm = reduced_runtime::ReducedRuntime::from_v14(v14).expect("Failed reducing runtime");
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
		let rtm1 = RuntimeFile::new(Chain::Polkadot, 14, 9260).try_into().expect("Runtime file should exist");
		let testbed = WasmTestBed::new(&Source::File(rtm1)).expect("Failed loading wasm");
		let metadata = testbed.metadata();

		match metadata {
			RuntimeMetadata::V14(v14) => {
				let rrtm = reduced_runtime::ReducedRuntime::from_v14(v14).expect("Failed reducing runtime");
				assert_eq!(rrtm.pallets.len(), 51);

				let first_pallet = &rrtm.pallets[&0];
				assert_eq!(0, first_pallet.index);
				assert_eq!("System", first_pallet.name);

				assert_eq!(9, first_pallet.calls.len());
				assert_eq!(6, first_pallet.events.len());
				assert_eq!(6, first_pallet.errors.len());
				assert_eq!(6, first_pallet.constants.len());
				assert_eq!(16, first_pallet.storages.len());

				assert_eq!("remark", &first_pallet.calls.get(&1).expect("Failed getting pallet").name)
			}
			_ => unreachable!(),
		}
	}

	#[test]
	#[ignore = "local data"]
	#[cfg(feature = "v14")]
	fn test_reduce_v14_storages() {
		let rtm1 = RuntimeFile::new(Chain::Polkadot, 14, 9260).try_into().expect("Runtime file should exist");
		let testbed = WasmTestBed::new(&Source::File(rtm1)).expect("Failed loading wasm");
		let metadata = testbed.metadata();

		match metadata {
			RuntimeMetadata::V14(v14) => {
				let rrtm = reduced_runtime::ReducedRuntime::from_v14(v14).expect("Failed reducing runtime");
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
		let rtm1 = RuntimeFile::new(Chain::Polkadot, 14, 9260).try_into().expect("Runtime file should exist");
		let testbed = WasmTestBed::new(&Source::File(rtm1)).expect("Failed loading wasm");
		let metadata = testbed.metadata();

		match metadata {
			RuntimeMetadata::V14(v14) => {
				let rrtm = reduced_runtime::ReducedRuntime::from_v14(v14).expect("Failed reducing runtime");
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

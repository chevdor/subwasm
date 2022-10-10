use comparable::MapChange;
use super::{calls::PalletId, changed_wapper::ChangedWrapper, reduced_pallet::*, reduced_runtime::*};

/// This struct holds both the ReducedRuntime and its changes.
/// It allows computing stats about the amount of changes,
/// what has changed (or not) and making the decision about wether
/// the new runtime breaks API compatibility with the reference one.
pub struct DiffAnalyzer<'a> {
	pub runtime_1: &'a ReducedRuntime,
	pub runtime_2: &'a ReducedRuntime,
	pub changes: &'a ChangedWrapper,
}

impl<'a> DiffAnalyzer<'a> {
	pub fn new(runtime_1: &'a ReducedRuntime, runtime_2: &'a ReducedRuntime, changes: &'a ChangedWrapper) -> Self {
		Self { runtime_1, runtime_2, changes }
	}

	pub fn get_pallet_changes(
		&self,
		pallet_id: u32,
	) -> Option<&MapChange<PalletId, ReducedPalletDesc, Vec<ReducedPalletChange>>> {
		self.changes.get_pallet_changes_by_id(pallet_id)
	}
}

impl<'a> Compatible for DiffAnalyzer<'a> {
	fn compatible(&self) -> bool {
		self.changes
			.0
			 .0
			.pallets
			.iter()
			.map(|p| match p {
				comparable::MapChange::Added(_k, _d) => true,
				comparable::MapChange::Removed(_k) => false,
				comparable::MapChange::Changed(_k, c) => c.iter().map(|x| x.compatible()).all(|x| x),
			})
			.all(|x| x)
	}
}

pub trait Compatible {
	/// This function reports whether the 2 runtimes APIs are compatible or not.
	/// This helps decide whether the runtime `transaction_version` should be bumped or not.
	fn compatible(&self) -> bool;
}

#[cfg(test)]
mod test_diffanalyzer {
	use super::*;
	use crate::differs::{reduced::reduced_differ::ReducedDiffer, test_runtimes::*};
	use std::path::PathBuf;
	use wasm_loader::Source;
	use wasm_testbed::WasmTestBed;

	fn compare_runtimes_compatibility(runtime_a: PathBuf, runtime_b: PathBuf) -> bool {
		let a = WasmTestBed::new(&Source::File(runtime_a)).unwrap();
		let b = WasmTestBed::new(&Source::File(runtime_b)).unwrap();

		let ra = a.metadata().into();
		let rb = b.metadata().into();
		let comp = ReducedDiffer::compare(&ra, &rb);

		match comp {
			Some(changes) => {
				let da = DiffAnalyzer::new(&ra, &rb, &changes);
				println!("spec_version {:?} -> {:?}", a.core_version().spec_version, b.core_version().spec_version);
				println!(
					"transaction {:?} -> {:?}",
					a.core_version().transaction_version,
					b.core_version().transaction_version
				);
				let compatible = da.compatible();
				println!("compatible = {:?}", compatible);
				compatible
			}
			None => {
				println!("No change found");
				true
			}
		}
	}

	#[test]
	#[ignore = "local data"]
	fn test_compatible_9260_9260() {
		assert!(compare_runtimes_compatibility(
			get_runtime_file(Chain::Polkadot, 14, 9260).unwrap(),
			get_runtime_file(Chain::Polkadot, 14, 9260).unwrap(),
		));
	}

	#[test]
	#[ignore = "local data"]
	fn test_compatible_9270_9270() {
		assert!(compare_runtimes_compatibility(
			get_runtime_file(Chain::Polkadot, 14, 9270).unwrap(),
			get_runtime_file(Chain::Polkadot, 14, 9270).unwrap(),
		));
	}

	#[test]
	#[ignore = "local data"]
	fn test_compatible_not_9260_9270() {
		assert!(!compare_runtimes_compatibility(
			get_runtime_file(Chain::Polkadot, 14, 9260).unwrap(),
			get_runtime_file(Chain::Polkadot, 14, 9280).unwrap(),
		));
	}

	#[test]
	#[ignore = "local data"]
	fn test_compatible_ksm_not_9280_9290() {
		assert!(!compare_runtimes_compatibility(
			get_runtime_file(Chain::Kusama, 14, 9280).unwrap(),
			get_runtime_file(Chain::Kusama, 14, 9290).unwrap(),
		));
	}

	#[test]
	#[ignore = "local data"]
	fn test_compatible_dot_not_9280_9290() {
		assert!(!compare_runtimes_compatibility(
			get_runtime_file(Chain::Polkadot, 14, 9280).unwrap(),
			get_runtime_file(Chain::Polkadot, 14, 9290).unwrap(),
		));
	}

	#[test]
	#[ignore = "local data"]
	fn test_changes_9280_9290() {
		// TODO: add macro/fn analyze!(Source, Source) -> DiffAnalyzer
		let a = get_runtime_file(Chain::Polkadot, 14, 9280).expect("Runtime file should exist");
		let ra = WasmTestBed::new(&Source::File(a)).unwrap().metadata().into();
		let b = get_runtime_file(Chain::Polkadot, 14, 9290).expect("Runtime file should exist");
		let rb = WasmTestBed::new(&Source::File(b)).unwrap().metadata().into();

		let changes = ReducedDiffer::compare(&ra, &rb).unwrap();

		let da = DiffAnalyzer::new(&ra, &rb, &changes);
		let pallet_system_changes = da.get_pallet_changes(0).unwrap();
		println!("pallet_system_changes = {:#?}", pallet_system_changes);

		// There is a single change in the system pallet between 9280 and 9290: Constant: Version
		match pallet_system_changes {
			MapChange::Changed(k, changes) => {
				assert_eq!(&0, k);
				assert_eq!(1, changes.len());
				let change = &changes[0];
				assert!(change.compatible());
			}
			_ => panic!("Unexpected change while comparing 9280 and 9290"),
		}

		let pallet_balances_changes = da.get_pallet_changes(4).unwrap();
		println!("pallet_balances_changes = {:#?}", pallet_balances_changes);

		// There is a single change in the balances pallet between 9280 and 9290: Calls: Signature changed
		match pallet_balances_changes {
			MapChange::Changed(k, changes) => {
				assert_eq!(&4, k);
				assert_eq!(1, changes.len());
				let change = &changes[0];
				assert!(!change.compatible());
			}
			_ => panic!("Unexpected change while comparing 9280 and 9290"),
		}
	}
}

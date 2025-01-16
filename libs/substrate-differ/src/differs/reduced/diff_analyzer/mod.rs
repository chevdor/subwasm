mod compatible;
mod require_tx_version_bump;
mod traits;

use log::trace;
pub use traits::*;

use crate::differs::reduced::prelude::ReducedRuntimeChange;

use super::{calls::PalletId, changed_wapper::ChangedWrapper, reduced_pallet::*};
use comparable::MapChange;
use std::rc::Rc;

/// This struct holds both the `[ReducedRuntime]` and its changes.
/// It allows computing stats about the amount of changes,
/// what has changed (or not) and making the decision about wether
/// the new runtime breaks API compatibility with the reference one.
pub struct DiffAnalyzer {
	pub changes: Rc<ChangedWrapper>,
}

impl DiffAnalyzer {
	pub fn new(changes: Rc<ChangedWrapper>) -> Self {
		Self { changes }
	}

	pub fn get_pallet_changes(
		&self,
		pallet_id: u32,
	) -> Option<&MapChange<PalletId, ReducedPalletDesc, Vec<ReducedPalletChange>>> {
		self.changes.get_pallet_changes_by_id(pallet_id)
	}

	pub fn is_storage_compatible(&self) -> bool {
		if self.changes.0.changes.is_empty() {
			return true;
		}

		let res = self
			.changes
			.0
			.changes
			.iter()
			.map(|change| match change {
				ReducedRuntimeChange::Pallets(pallets) => pallets
					.iter()
					.map(|p| match p {
						comparable::MapChange::Added(_key, _desc) => true,
						comparable::MapChange::Removed(_key) => {
							trace!("IsStorageCompatible | Removed pallet needs migration to clear its storage");
							false
						}
						comparable::MapChange::Changed(_key, change) => {
							// check changed pallets for storage compatibility
							change.iter().map(|x| x.is_storage_compatible()).all(|x| x)
						}
					})
					.all(|x| x),
				ReducedRuntimeChange::Extrinsic(_) => true,
			})
			.all(|x| x);
		trace!("IsStorageCompatible | Analyzer: {res}");
		res
	}
}

#[cfg(test)]
mod test_diffanalyzer {
	use super::*;
	use crate::differs::{reduced::reduced_diff_result::ReducedDiffResult, test_runtimes::*};
	use std::path::PathBuf;
	use wasm_loader::Source;
	use wasm_testbed::WasmTestBed;

	fn analyze(rf1: RuntimeFile, rf2: RuntimeFile) -> Option<DiffAnalyzer> {
		let a = rf1.try_into().expect("Runtime file should exist");
		let b = rf2.try_into().expect("Runtime file should exist");

		let ra = WasmTestBed::new(&Source::File(a)).expect("Failed loading runtime").metadata().into();
		let rb = WasmTestBed::new(&Source::File(b)).expect("Failed loading runtime").metadata().into();
		ReducedDiffResult::new(ra, rb).changes.map(DiffAnalyzer::new)
	}

	fn require_tx_version_bump(runtime_a: PathBuf, runtime_b: PathBuf) -> bool {
		let a = WasmTestBed::new(&Source::File(runtime_a)).expect("Failed loading runtime");
		let b = WasmTestBed::new(&Source::File(runtime_b)).expect("Failed loading runtime");

		let ra = a.metadata().into();
		let rb = b.metadata().into();
		let diff_result = ReducedDiffResult::new(ra, rb);
		diff_result.require_transaction_version_bump()
	}

	fn compat(runtime_a: PathBuf, runtime_b: PathBuf) -> bool {
		let a = WasmTestBed::new(&Source::File(runtime_a)).expect("Failed loading runtime");
		let b = WasmTestBed::new(&Source::File(runtime_b)).expect("Failed loading runtime");

		let ra = a.metadata().into();
		let rb = b.metadata().into();
		let diff_result = ReducedDiffResult::new(ra, rb);
		diff_result.compatible()
	}

	#[test]
	#[ignore = "local data"]
	fn test_require_tx_version_bump_9260_9260() {
		assert!(!require_tx_version_bump(
			RuntimeFile::new(Chain::Polkadot, 14, 9260).try_into().expect("Failed loading runtime"),
			RuntimeFile::new(Chain::Polkadot, 14, 9260).try_into().expect("Failed loading runtime"),
		));
	}

	#[test]
	#[ignore = "local data"]
	fn test_require_tx_version_bump_9400_9420() {
		// TODO: tricky... It does require a bump since transfer changed from id 0 to 7 but the diff sees a renaming only
		let r1 = RuntimeFile::new(Chain::AssetHubPolkadot, 14, 9400).try_into().expect("Failed loading runtime");
		let r2 = RuntimeFile::new(Chain::AssetHubPolkadot, 14, 9420).try_into().expect("Failed loading runtime");
		assert!(!require_tx_version_bump(r1, r2));
	}

	#[test]
	#[ignore = "local data"]
	fn test_require_tx_version_bump_collectives_9400_9420() {
		// pallet 10 (Balances), Call transfer changed from index 0 to 7.
		let r1 = RuntimeFile::new(Chain::CollectivesPolkadot, 14, 9400).try_into().expect("Failed loading runtime");
		let r2 = RuntimeFile::new(Chain::CollectivesPolkadot, 14, 9420).try_into().expect("Failed loading runtime");
		assert!(require_tx_version_bump(r1, r2));
	}

	#[test]
	#[ignore = "local data"]
	fn test_require_tx_version_bump_9270_9270() {
		assert!(!require_tx_version_bump(
			RuntimeFile::new(Chain::Polkadot, 14, 9270).try_into().expect("Failed loading runtime"),
			RuntimeFile::new(Chain::Polkadot, 14, 9270).try_into().expect("Failed loading runtime"),
		));
	}

	#[test]
	#[ignore = "local data"]
	fn test_require_tx_version_bump_not_9260_9270() {
		assert!(!require_tx_version_bump(
			RuntimeFile::new(Chain::Polkadot, 14, 9260).try_into().expect("Failed loading runtime"),
			RuntimeFile::new(Chain::Polkadot, 14, 9270).try_into().expect("Failed loading runtime"),
		));
	}

	#[test]
	#[ignore = "local data"]
	fn test_require_tx_version_bump_not_9260_9280() {
		assert!(!require_tx_version_bump(
			RuntimeFile::new(Chain::Polkadot, 14, 9260).try_into().expect("Failed loading runtime"),
			RuntimeFile::new(Chain::Polkadot, 14, 9280).try_into().expect("Failed loading runtime"),
		));
	}

	#[test]
	#[ignore = "local data"]
	fn test_require_tx_version_bump_ksm_not_9280_9290() {
		assert!(!require_tx_version_bump(
			RuntimeFile::new(Chain::Kusama, 14, 9280).try_into().expect("Failed loading runtime"),
			RuntimeFile::new(Chain::Kusama, 14, 9290).try_into().expect("Failed loading runtime"),
		));
	}

	#[test]
	#[ignore = "local data"]
	fn test_require_tx_version_bump_dot_not_9280_9290() {
		// Lots of signature changes but nothing requiring a tx version bump
		assert!(!require_tx_version_bump(
			RuntimeFile::new(Chain::Polkadot, 14, 9280).try_into().expect("Failed loading runtime"),
			RuntimeFile::new(Chain::Polkadot, 14, 9290).try_into().expect("Failed loading runtime"),
		));
	}

	#[test]
	#[ignore = "local data"]
	fn test_compatible_not_9280_9290() {
		// Lots of signature changes but nothing requiring a tx version bump
		assert!(!compat(
			RuntimeFile::new(Chain::Polkadot, 14, 9280).try_into().expect("Failed loading runtime"),
			RuntimeFile::new(Chain::Polkadot, 14, 9290).try_into().expect("Failed loading runtime"),
		));
	}

	#[test]
	#[ignore = "local data"]
	fn test_changes_9280_9290() {
		let da = analyze(RuntimeFile::new(Chain::Polkadot, 14, 9280), RuntimeFile::new(Chain::Polkadot, 14, 9290))
			.expect("Failed loading runtime");
		let pallet_system_changes = da.get_pallet_changes(0).expect("Failed loading runtime");
		println!("pallet_system_changes = {pallet_system_changes:?}");

		// There is a single change in the system pallet between 9280 and 9290: Constant: Version
		match pallet_system_changes {
			MapChange::Changed(k, changes) => {
				assert_eq!(&0, k);
				assert_eq!(1, changes.len());
				let change = &changes[0];
				assert!(!change.require_tx_version_bump());
				assert!(change.compatible());
			}
			_ => panic!("Unexpected change while comparing 9280 and 9290"),
		}

		let pallet_balances_changes = da.get_pallet_changes(4).expect("Failed loading runtime");
		println!("pallet_balances_changes = {pallet_balances_changes:#?}");

		// There is a single change in the balances pallet between 9280 and 9290: Calls: Signature changed
		match pallet_balances_changes {
			MapChange::Changed(k, changes) => {
				assert_eq!(&4, k);
				assert_eq!(1, changes.len());
				let change = &changes[0];
				assert!(!change.compatible());
				assert!(!change.require_tx_version_bump());
			}
			_ => panic!("Unexpected change while comparing 9280 and 9290"),
		}
	}

	#[test]
	#[cfg(feature = "v13")]
	#[cfg(feature = "v14")]
	#[ignore = "local data"]
	fn test_different_variants_v13_v14() {
		let a = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_V13_1))).expect("Failed loading runtime");
		let b = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_V14))).expect("Failed loading runtime");
		let _differ = ReducedDiffer::new(a.metadata(), b.metadata());
	}

	#[test]
	#[cfg(feature = "v13")]
	#[ignore = "local data"]
	fn test_v13() {
		let a = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_V13_1))).expect("Failed loading runtime");
		let b = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_V13_2))).expect("Failed loading runtime");
		let comp = ReducedDiffer::compare(&a, &b);
	}

	#[test]
	#[cfg(feature = "v14")]
	#[ignore = "local data"]
	fn test_v14_polkadot_9290_9290() {
		let diff = analyze(RuntimeFile::new(Chain::Polkadot, 14, 9290), RuntimeFile::new(Chain::Polkadot, 14, 9290));
		assert!(diff.is_none());
	}

	#[test]
	#[cfg(all(feature = "v13", feature = "v14"))]
	#[ignore = "local data"]
	fn test_v14_polkadot_9100_9260() {
		let diff = analyze(RuntimeFile::new(Chain::Polkadot, 14, 9100), RuntimeFile::new(Chain::Polkadot, 14, 9260))
			.expect("Failed loading runtime");

		assert!(!diff.require_tx_version_bump())
	}

	#[test]
	#[cfg(feature = "v14")]
	#[ignore = "local data"]
	fn test_v14_polkadot_9260_9270_full() {
		let diff = analyze(RuntimeFile::new(Chain::Polkadot, 14, 9260), RuntimeFile::new(Chain::Polkadot, 14, 9270))
			.expect("Failed loading runtime");

		assert!(!diff.require_tx_version_bump())
	}

	#[test]
	#[cfg(feature = "v14")]
	#[ignore = "local data"]
	fn test_v14_polkadot_9260_9270_content() {
		let analyzer =
			analyze(RuntimeFile::new(Chain::Polkadot, 14, 9260), RuntimeFile::new(Chain::Polkadot, 14, 9270))
				.expect("Failed loading runtime");
		let pallet_changes = analyzer.changes.get_pallets_changes();
		assert_eq!(4, pallet_changes.len());
		assert!(!analyzer.require_tx_version_bump());
		assert!(!analyzer.compatible());
	}

	#[test]
	#[cfg(feature = "v14")]
	#[ignore = "local data"]
	fn test_v14_polkadot_9270_9270_content() {
		let a: PathBuf = RuntimeFile::new(Chain::Polkadot, 14, 9270).try_into().expect("Failed loading runtime");
		// let rb = RuntimeFile::new(Chain::Polkadot, 14, 9270);
		// let analyzer = analyze(ra, rb).unwrap();
		// let pallet_changes = analyzer.changes.get_pallets_changes();
		// assert_eq!(0, pallet_changes.len());
		// assert!(!analyzer.require_tx_version_bump());
		// assert!(analyzer.compatible());

		let ra = WasmTestBed::new(&Source::File(a.clone())).expect("Failed loading runtime").metadata().into();
		let rb = WasmTestBed::new(&Source::File(a.clone())).expect("Failed loading runtime").metadata().into();

		let diff_result = ReducedDiffResult::new(ra, rb);
		assert!(!diff_result.require_transaction_version_bump());
		assert!(diff_result.compatible());
	}

	#[test]
	#[ignore = "local data"]
	#[should_panic]
	fn test_unsupported_variants() {
		let diff = analyze(RuntimeFile::new(Chain::Polkadot, 12, 9000), RuntimeFile::new(Chain::Polkadot, 12, 9000))
			.expect("Failed loading runtime");

		assert!(!diff.require_tx_version_bump())
	}

	#[test]
	#[cfg(feature = "v14")]
	#[ignore = "local data"]
	fn test_v14_polkadot_9280_9290_full() {
		let diff = analyze(RuntimeFile::new(Chain::Polkadot, 14, 9280), RuntimeFile::new(Chain::Polkadot, 14, 9290))
			.expect("Failed loading runtime");
		assert!(!diff.require_tx_version_bump());

		println!("changes = {:?}", diff.changes);
		// assert_eq!(12, diff.changes.get )

		// todo: add folling expectations to the test. Those are tested in test_changes_9280_9290
		// all the changes below are SIG changes
		// - TEST: Summary pallet  | compat change  | breaking change
		// 			[System] | 0              | sig: 1 [blockWeight] type: FrameSupportWeightsPerDispatchClassU64 -> FrameSupportWeightsPerDispatchClassWeight
		// 		[Scheduler]  | 0 			  | 0
		// 		 [Preimage] | 0 			  | 0
		// 			 [Babe] | 0 			  | 0
		// 		[Timestamp] | 0 			  | 0
		// 		  [Indices] | 0 			  | sig: 2 (transfer + forceTransfer)
		// 		 [Balances] | 0 			  | 0
		// 	   [Authorship] | 0 			  | 0
		// 		  [Staking] | 0 			  | 0
		// 		  [Session] | 0 			  | 0
		// 		  [Grandpa] | 0 			  | 0
		// 		 [ImOnline] | 0 			  | 0
		// 		[Democracy] | 0    			  | 3
		// 		  [Council] | 0    			  | 1
		// [TechnicalCommittee] | 0    		  | 1
		//  [PhragmenElection] | 0    		  | 0
		// [TechnicalMembership] | 0    	  | 5
		// 		 [Treasury] | 0    			  | 0
		// 		   [Claims] | 0    			  | 0
		// 		  [Vesting] | 0    			  | 0
		// 		 [Identity] | 0    			  | 2
		// 			[Proxy] | 0    			  | 8
		// 		 [Multisig] | 0    			  | 2
		// 		 [Bounties] | 0    			  | 0
		// 	[ChildBounties] | 0    			  | 0
		// 			 [Tips] | 0    			  | 2
		// [ElectionProviderMultiPhase] | 0   | 0
		// 		[VoterList] | 0    			  | 2
		//   [NominationPools] | 0    		  | 3
		// 	[Configuration] idx: 51 | 0    	  | 0
		// 	   [setUmpServiceTotalWeight] | 0  | 1
		// 	  [setUmpMaxIndividualWeight]| 0    | 1
		// 	 [ParaInherent] idx: 54 | 0    	  | 0
		// 			[Paras] idx: 56 | 0    	  | 0
		// 	  [Initializer] idx: 57 | 0    	  | 0
		// 			  [Ump] idx: 59| 0    	  | 0
		// 			  [serviceOverweight] | 0  | 1
		// 			 [Hrmp] idx: 60 | 0    	  | 0
		// 	[ParasDisputes] idx: 62| 0    	  | 0
		// 		[Registrar] idx: 70 | 0   	  | 0
		// 			[Slots] idx: 71 | 0   	  | 0
		// 		 [Auctions] idx: 72| 0    	  | 0
		// 		[Crowdloan] idx: 73 | 0    	  | 0
		// 		[XcmPallet] idx: 99 | 0    	  | 1
	}
}

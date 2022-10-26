// TODO: This mod does nothing but calling ReducedDiffResult::new so we can call ReducedDiffResult::new directly and remove this code. Later we can rename ReducedDiffResult into ReducedDiffer if that makes sense.
// TODO: Move the tests away to appropriate mod

// use super::{reduced_diff_result::ReducedDiffResult, reduced_runtime::*};

// /// The [ReducedDiffer] works exclusively on 2 [ReducedRuntime].
// pub struct ReducedDiffer;

// impl<'a> ReducedDiffer {
// 	pub fn compare(ra: &'a ReducedRuntime, rb: &'a ReducedRuntime) -> ReducedDiffResult {
// 		println!("ReducedDiffer::compare(...)");
// 		ReducedDiffResult::new(ra, rb)
// 	}
// }

// #[cfg(test)]
// mod test_diff_runtimes {
// 	use super::ReducedDiffer;
// 	use crate::differs::test_runtimes::*;
// 	use std::path::PathBuf;
// 	use wasm_loader::Source;
// 	use wasm_testbed::WasmTestBed;

// TODO: MOVE to diff_analyzer

// 	#[test]
// 	#[cfg(feature = "v13")]
// 	#[ignore = "local data"]
// 	fn test_v13() {
// 		let a = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_V13_1))).unwrap();
// 		let b = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_V13_2))).unwrap();
// 		let comp = ReducedDiffer::compare(&a, &b);
// 	}

// 	#[test]
// 	#[cfg(feature = "v14")]
// 	#[ignore = "local data"]
// 	fn test_v14_polkadot_9100_9100() {
// 		let ra = get_runtime_file(Chain::Westmint, 14, 9100).expect("Runtime file should exist");
// 		let a = WasmTestBed::new(&Source::File(ra)).unwrap().metadata().into();
// 		let rb = get_runtime_file(Chain::Westmint, 14, 9100).expect("Runtime file should exist");
// 		let b = WasmTestBed::new(&Source::File(rb)).unwrap().metadata().into();
// 		let comp = ReducedDiffer::compare(&a, &b);

// 		assert!(comp.is_none());
// 	}

// 	#[test]
// 	#[cfg(feature = "v14")]
// 	#[ignore = "local data"]
// 	fn test_v14_polkadot_9100_9260() {
// 		let ra = get_runtime_file(Chain::Westmint, 14, 9100).expect("Runtime file should exist");
// 		let a = WasmTestBed::new(&Source::File(ra)).unwrap().metadata().into();
// 		let rb = get_runtime_file(Chain::Westmint, 14, 9260).expect("Runtime file should exist");
// 		let b = WasmTestBed::new(&Source::File(rb)).unwrap().metadata().into();
// 		let comp = ReducedDiffer::compare(&a, &b);

// 		assert!(comp.is_some());
// 	}

// 	#[test]
// 	#[cfg(feature = "v14")]
// 	#[ignore = "local data"]
// 	fn test_v14_polkadot_9260_9260() {
// 		let ra = get_runtime_file(Chain::Westmint, 14, 9260).expect("Runtime file should exist");
// 		let a = WasmTestBed::new(&Source::File(ra)).unwrap().metadata().into();
// 		let rb = get_runtime_file(Chain::Westmint, 14, 9260).expect("Runtime file should exist");
// 		let b = WasmTestBed::new(&Source::File(rb)).unwrap().metadata().into();
// 		let comp = ReducedDiffer::compare(&a, &b);
// 		assert!(comp.is_none());
// 		println!("comp = {}", comp.unwrap());
// 	}

// 	#[test]
// 	#[cfg(feature = "v14")]
// 	#[ignore = "local data"]
// 	fn test_v14_polkadot_9260_9270_full() {
// 		let ra = get_runtime_file(Chain::Westmint, 14, 9270).expect("Runtime file should exist");
// 		let a = WasmTestBed::new(&Source::File(ra)).unwrap().metadata().into();
// 		let rb = get_runtime_file(Chain::Westmint, 14, 9290).expect("Runtime file should exist");
// 		let b = WasmTestBed::new(&Source::File(rb)).unwrap().metadata().into();

// 		let comp = ReducedDiffer::compare(&a, &b).unwrap();
// 		println!("COMP:\n{}", comp);
// 	}

// 	#[test]
// 	#[cfg(feature = "v14")]
// 	#[ignore = "local data"]
// 	fn test_v14_polkadot_9260_9270_system() {
// 		use crate::differs::reduced::reduced_runtime::ReducedRuntime;

// 		let a = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_V14_9100))).unwrap();
// 		let b = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_POLKADOT_V14_9260))).unwrap();

// 		let ra = ReducedRuntime::from(a.metadata());
// 		let rb = ReducedRuntime::from(b.metadata());

// 		let sys_ra = &ra.pallets[&0];
// 		let sys_rb = &rb.pallets[&0];

// 		println!("sys_ra = {}", sys_ra);
// 		println!("sys_rb = {}", sys_rb);

// 		// let comp = ReducedDiffer::compare(&a, &b);
// 		// let result_system = &results.iter().find(|item| item.0 .0 == "System").unwrap().1;
// 		// assert!(matches!(result_system.change, Change::Modified(_)));

// 		// println!("result_system = {:#?}", result_system);
// 	}

// 	#[test]
// 	#[cfg(feature = "v14")]
// 	#[ignore = "local data"]
// 	fn test_v14_polkadot_9280_9290_full() {
// 		use crate::differs::reduced::diff_analyzer::{Compatible, DiffAnalyzer};

// 		let ra = get_runtime_file(Chain::Polkadot, 14, 9280).expect("Runtime file should exist");
// 		let a = WasmTestBed::new(&Source::File(ra)).unwrap().metadata().into();
// 		let rb = get_runtime_file(Chain::Polkadot, 14, 9290).expect("Runtime file should exist");
// 		let b = WasmTestBed::new(&Source::File(rb)).unwrap().metadata().into();

// 		let comp = ReducedDiffer::compare(&a, &b);

// 		assert!(comp.as_ref().is_some());
// 		println!("COMP:");
// 		println!("{:#?}", comp);

// 		let changes = comp.unwrap();
// 		let da = DiffAnalyzer::new(&a, &b, &changes);
// 		assert!(!da.compatible());

// 		println!("changes = {:?}", changes);
// 		// assert_eq!(12, changes)
// 		todo!()
// TODO: TESTS Creat tests
// 		// todo: add folling expectations to the test. Those are tested in test_changes_9280_9290
// 		// all the changes below are SIG changes
// 		// - need tx version bump: YES
// 		// - TEST: Summary pallet  | compat change  | breaking change
// 		// 			[System] | 0              | sig: 1 [blockWeight] type: FrameSupportWeightsPerDispatchClassU64 -> FrameSupportWeightsPerDispatchClassWeight
// 		// 		[Scheduler]  | 0 			  | 0
// 		// 		 [Preimage] | 0 			  | 0
// 		// 			 [Babe] | 0 			  | 0
// 		// 		[Timestamp] | 0 			  | 0
// 		// 		  [Indices] | 0 			  | sig: 2 (transfer + forceTransfer)
// 		// 		 [Balances] | 0 			  | 0
// 		// 	   [Authorship] | 0 			  | 0
// 		// 		  [Staking] | 0 			  | 0
// 		// 		  [Session] | 0 			  | 0
// 		// 		  [Grandpa] | 0 			  | 0
// 		// 		 [ImOnline] | 0 			  | 0
// 		// 		[Democracy] | 0    			  | 3
// 		// 		  [Council] | 0    			  | 1
// 		// [TechnicalCommittee] | 0    		  | 1
// 		//  [PhragmenElection] | 0    		  | 0
// 		// [TechnicalMembership] | 0    	  | 5
// 		// 		 [Treasury] | 0    			  | 0
// 		// 		   [Claims] | 0    			  | 0
// 		// 		  [Vesting] | 0    			  | 0
// 		// 		 [Identity] | 0    			  | 2
// 		// 			[Proxy] | 0    			  | 8
// 		// 		 [Multisig] | 0    			  | 2
// 		// 		 [Bounties] | 0    			  | 0
// 		// 	[ChildBounties] | 0    			  | 0
// 		// 			 [Tips] | 0    			  | 2
// 		// [ElectionProviderMultiPhase] | 0   | 0
// 		// 		[VoterList] | 0    			  | 2
// 		//   [NominationPools] | 0    		  | 3
// 		// 	[Configuration] idx: 51 | 0    	  | 0
// 		// 	   [setUmpServiceTotalWeight] | 0  | 1
// 		// 	  [setUmpMaxIndividualWeight]| 0    | 1
// 		// 	 [ParaInherent] idx: 54 | 0    	  | 0
// 		// 			[Paras] idx: 56 | 0    	  | 0
// 		// 	  [Initializer] idx: 57 | 0    	  | 0
// 		// 			  [Ump] idx: 59| 0    	  | 0
// 		// 			  [serviceOverweight] | 0  | 1
// 		// 			 [Hrmp] idx: 60 | 0    	  | 0
// 		// 	[ParasDisputes] idx: 62| 0    	  | 0
// 		// 		[Registrar] idx: 70 | 0   	  | 0
// 		// 			[Slots] idx: 71 | 0   	  | 0
// 		// 		 [Auctions] idx: 72| 0    	  | 0
// 		// 		[Crowdloan] idx: 73 | 0    	  | 0
// 		// 		[XcmPallet] idx: 99 | 0    	  | 1
// 	}

// 	#[test]
// 	#[ignore = "local data"]
// 	#[should_panic]
// 	fn test_unsupported_variants() {
// 		let a = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_V12))).unwrap().metadata().into();
// 		let b = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_V12))).unwrap().metadata().into();
// 		let _differ = ReducedDiffer::compare(&a, &b);
// 	}
// }

use super::diff_result::DiffResult;
use super::reduced_runtime::ReducedRuntime;
use crate::differs::{DiffOptions, Differ};
use crate::differs::{reduced::reduced_pallet::ReducedPallet, reduced::*, reduced::call::prelude::Index, DiffOptions, Differ};
use frame_metadata::{RuntimeMetadata, RuntimeMetadata::*};
use std::collections::HashMap;
use std::collections::HashSet;

// TODO: Placeholder, here we can convert from V14 to V13. We don't need to convert
// once we can normalize.
// pub fn convert(_r: &v14::RuntimeMetadataV14) -> Option<v13::RuntimeMetadataV13> {
// 	todo!()
// }

type MetadataVersion = u32;

// TODO: not great, we could already enforce here getting the same variant
pub struct ReducedDiffer {
	r1: ReducedRuntime,
	r2: ReducedRuntime,
	version: MetadataVersion,
}

impl ReducedDiffer {
	pub fn new(r1: &RuntimeMetadata, r2: &RuntimeMetadata) -> Self {
		log::debug!("++ReducedDiffer");
		// if std::mem::discriminant(r1) != std::mem::discriminant(r2) {
		// 	panic!("Only same Metadata Versions can be compared");
		// };

		match (&r1, &r2) {
			(V13(_), V13(_)) => Self { r1: r1.into(), r2: r2.into(), version: 13 },
			// (V13(_), V14(b)) => {
			// 	Self { r1: r1.into(), r2: (RuntimeMetadata::V13(convert(b).unwrap())).into(), version: 13 }
			// }
			// (V14(a), V13(_)) => {
			// 	Self { r1: (RuntimeMetadata::V13(convert(a).unwrap())).into(), r2: r2.into(), version: 13 }
			// }
			(V14(_), V14(_)) => Self { r1: r1.into(), r2: r2.into(), version: 14 },
			_ => panic!("Unsupported versions set, we support only Vn/Vn or Vn/Vn+1"),
		}
	}

	// This is a raw comparison based on the json serialization of the metadata
	// pub fn compare(&self) {
	// 	log::debug!("A: {:?}", self.r1);
	// 	log::debug!("B: {:?}", self.r2);

	// 	match (self.r1, self.r2) {
	// 		(V13(_a), V13(_b)) => self.compare_reduced(),
	// 		(V14(_a), V14(_b)) => self.compare_reduced(),
	// 		_ => panic!("V12 is unsupported"),
	// 	}
	// }
}

impl Differ<ReducedPallet> for ReducedDiffer {
	// TODO: The following may even go to the default impl in the Trait
	fn diff(&self, options: DiffOptions) -> Vec<(PalletId, DiffResult<'_, ReducedPallet>)> {
		// assert!(self.r1.ver) != std::mem::discriminant(&self.r2), "");
		log::debug!("Comparing 2 v{:?} runtimes", self.version);
		log::debug!("options: {:#?}", options);

		let r1 = &self.r1;
		let r2 = &self.r2;

		// We gather the Set of all indexes in both pallets
		let indexes_1: HashMap<PalletId, &ReducedPallet> =
			r1.pallets.iter().map(|pallet| ((pallet.name.clone(), pallet.index), pallet)).collect();
		let indexes_2: HashMap<PalletId, &ReducedPallet> =
			r2.pallets.iter().map(|pallet| ((pallet.name.clone(), pallet.index), pallet)).collect();
		// println!("indicies {:?}", indexes_1);
		let mut indexes: HashSet<PalletId> = indexes_1.keys().cloned().collect();
		indexes.extend(indexes_2.keys().cloned());
		// println!("indexes_1 = {:?}", indexes_1);
		// println!("indexes_2 = {:?}", indexes_2);
		// println!("indexes = {:?}", indexes);
		// assert_eq!(indexes_1.len(), 51);
		// assert_eq!(indexes_2.len(), 50);
		// assert_eq!(indexes.len(), 51);

		let mut results = vec![];

		indexes.into_iter().for_each(|key| {
			let pallet_a = indexes_1.get(&key);
			let pallet_b = indexes_2.get(&key);

			match (pallet_a, pallet_b) {
				(Some(pallet_a), Some(pallet_b)) => {
					let d = ReducedPallet::diff(pallet_a, pallet_b);

					if let crate::differs::raw::change_type::ChangeType::Unchanged = d.change_type {
						println!("no changes for pallet {:?}", key);
					} else {
						println!("d = {:#?}", d);
					}
					results.push((key, d));
				}
				_ => todo!("write me"),
			}
		});

		results
	}
}

#[cfg(test)]
mod test_diff_runtimes {
	use crate::differs::{raw::change_type::ChangeType, DiffOptions, Differ};

	use super::ReducedDiffer;
	use std::path::PathBuf;
	use wasm_loader::Source;
	use wasm_testbed::WasmTestBed;

	// TODO: put that in a 	single file
	const RUNTIME_V12: &str = "../../data/runtime_v12.wasm";
	const RUNTIME_V13_1: &str = "../../data/kusama/V13/kusama-9030.wasm";
	const RUNTIME_V13_2: &str = "../../data/kusama/V13/kusama-9080.wasm";
	const RUNTIME_V14: &str = "../../data/runtime_v14.wasm";

	#[test]
	#[ignore = "local data"]
	fn test_different_variants() {
		let a = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_V13_1))).unwrap();
		let b = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_V14))).unwrap();
		let _differ = ReducedDiffer::new(a.metadata(), b.metadata());
	}

	#[test]
	#[ignore = "local data"]
	fn test_v13() {
		let a = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_V13_1))).unwrap();
		let b = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_V13_2))).unwrap();
		let differ = ReducedDiffer::new(a.metadata(), b.metadata());
		differ.diff(DiffOptions::default());
	}

	#[test]
	#[ignore = "local data"]
	fn test_v14() {
		let a = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_V14))).unwrap();
		let b = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_V14))).unwrap();
		let differ = ReducedDiffer::new(a.metadata(), b.metadata());
		let results = differ.diff(DiffOptions::default());

		for ((pallet_name, pallet_index), pallet_diff) in results {
			assert!(matches!(pallet_diff.change_type, ChangeType::Unchanged));
		}
	}

	#[test]
	#[ignore = "local data"]
	#[should_panic]
	fn test_unsupported_variants() {
		let a = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_V12))).unwrap();
		let b = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_V12))).unwrap();
		let _differ = ReducedDiffer::new(a.metadata(), b.metadata());
	}
}

// #[cfg(test)]
// mod test_normalized {
// 	use super::*;
// 	const RUNTIME_V13: &str = "../../data/runtime_v13.wasm";
// 	const RUNTIME_V14: &str = "../../data/runtime_v14.wasm";

// 	#[test]
// 	fn test_ctor() {
// 		let item1 = PalletItem { index: None, name: "foo".into(), signature: Box::new(String::from("foo")) };
// 		let pi = PalletItemType::Call(item1);
// 		// let item2 = PalletItem { index: Some(42u32), name: "foobar".into(), signature: String::"foobar".into() };
// 		// let item3 = PalletItem { index: Some(42u32), name: "foobar".into(), signature: vec![42, "foobar"] };
// 		// let normalizedPallet = ReducedPallet { items: Some(vec![item1, item2, item3]) };
// 		let normalized_pallet = ReducedPallet { items: Some(vec![pi]) };

// 		let reduced_runtime = ReducedRuntime { pallets: Some(vec![normalized_pallet]) };

// 		let differ = MetadataPartialDiffer::new()
// 	}
// }

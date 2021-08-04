use frame_metadata::{v13, v14, RuntimeMetadata, RuntimeMetadata::*};
use serde_json::Value;

// TODO: not great, we could already enforce here getting the same variant
pub struct MetadataPartialDiffer {
	r1: ReducedRuntime,
	r2: ReducedRuntime,
	version: u8,
}

struct PalletItem {
	index: Option<u32>,
	name: String,
	signature: Box<dyn Signature>,
}

enum PalletItemType {
	Call(PalletItem),
	Storage(PalletItem),
	Event(PalletItem),
	Error(PalletItem),
	Constant(PalletItem),
}

// type Signature = Box<dyn MySerialize>;

pub struct ReducedPallet {
	items: Option<Vec<PalletItemType>>,
}

// Hasher / PartialEq / Serialize
// Eq, Encode,

pub struct ReducedRuntime {
	pallets: Option<Vec<ReducedPallet>>,
}

impl ReducedRuntime {
	pub fn from_v13(v13: &v13::RuntimeMetadataV13) -> Result<Self, String> {
		todo!()
	}
	pub fn from_v14(v14: &v14::RuntimeMetadataV14) -> Result<Self, String> {
		todo!()
	}
}

impl From<&RuntimeMetadata> for ReducedRuntime {
	fn from(runtime_metadata: &RuntimeMetadata) -> Self {
		match &runtime_metadata {
			V13(v13) => ReducedRuntime::from_v13(v13).unwrap(),
			V14(v14) => ReducedRuntime::from_v14(v14).unwrap(),
			_ => panic!("Unsupported metadata version"),
		}
	}
}

trait Signature {
	fn serialize(&self) -> Value;
}

impl<S: serde::ser::Serialize> Signature for S {
	fn serialize(&self) -> Value {
		serde_json::to_value(self).unwrap()
	}
}

/// Placeholder, here we can convert from V14 to V13
pub fn convert(_r: &v14::RuntimeMetadataV14) -> Option<v13::RuntimeMetadataV13> {
	todo!()
}

impl<'a> MetadataPartialDiffer {
	pub fn new(r1: &RuntimeMetadata, r2: &RuntimeMetadata) -> Self {
		// if std::mem::discriminant(r1) != std::mem::discriminant(r2) {
		// 	panic!("Only same Metadata Versions can be compared");
		// };

		match (&r1, &r2) {
			(V13(_), V13(_)) => Self { r1: r1.into(), r2: r2.into(), version: 13 },
			(V13(_), V14(b)) => {
				Self { r1: r1.into(), r2: (&RuntimeMetadata::V13(convert(b).unwrap())).into(), version: 13 }
			}
			(V14(a), V13(_)) => {
				Self { r1: (&RuntimeMetadata::V13(convert(a).unwrap())).into(), r2: r2.into(), version: 13 }
			}
			(V14(_), V14(_)) => Self { r1: r1.into(), r2: r2.into(), version: 14 },
			_ => panic!("Unsupported versions set, we support only Vn/Vn or Vn/Vn+1"),
		}
	}

	pub fn compare_reduced(&self) {
		// assert!(self.r1.ver) != std::mem::discriminant(&self.r2), "");

		log::debug!("Comparing 2 v{:?} runtimes", self.version);
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

#[cfg(test)]
mod test_super {
	use super::MetadataPartialDiffer;
	use std::path::PathBuf;
	use wasm_loader::Source;
	use wasm_testbed::WasmTestBed;

	// TODO: put that in a 	single file
	const RUNTIME_V12: &str = "../../data/runtime_v12.wasm";
	const RUNTIME_V13: &str = "../../data/runtime_v13.wasm";
	const RUNTIME_V14: &str = "../../data/runtime_v14.wasm";

	#[test]
	#[ignore = "local data"]
	#[should_panic]
	fn test_different_variants() {
		let a = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_V13))).unwrap();
		let b = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_V14))).unwrap();
		let _differ = MetadataPartialDiffer::new(a.metadata(),b.metadata());
	}

// 	#[test]
// 	#[ignore = "local data"]
// 	fn test_v13() {
// 		let a = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_V13))).unwrap();
// 		let b = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_V13))).unwrap();
// 		let _differ = MetadataPartialDiffer::new(&a.metadata(), &b.metadata());
// 	}

// 	#[test]
// 	#[ignore = "local data"]
// 	fn test_v14() {
// 		let a = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_V14))).unwrap();
// 		let b = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_V14))).unwrap();
// 		let _differ = MetadataPartialDiffer::new(&a.metadata(), &b.metadata());
// 	}

// 	#[test]
// 	#[ignore = "local data"]
// 	#[should_panic]
// 	fn test_unsupported_variants() {
// 		let a = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_V12))).unwrap();
// 		let b = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_V12))).unwrap();
// 		let _differ = MetadataPartialDiffer::new(&a.metadata(), &b.metadata());
// 	}
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

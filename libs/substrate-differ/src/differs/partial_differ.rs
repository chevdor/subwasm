use frame_metadata::{RuntimeMetadata, RuntimeMetadata::*, StorageEntryMetadata};

// TODO: not great, we could already enforce here getting the same variant
pub struct MetadataPartialDiffer<'a> {
	r1: &'a RuntimeMetadata,
	r2: &'a RuntimeMetadata,
}



// enum ConentType {
// 	Call,
// 	Storage,
// 	Event,
// 	Error,
// 	Constant,
// }

// pub struct NormalizedPallet {
// 	calls: Option<Vec<Call>>,
// 	storage: Option<Vec<Storage>>,
// 	events: Option<Vec<Event>>,
// 	errors: Option<Vec<Error>>,
// 	constants: Option<Vec<Constant>>,
// }

// pub struct NormalizedMetadata<T = PartialEq<U>> {
// 	header: T,
// }

impl<'a> MetadataPartialDiffer<'a> {
	pub fn new(r1: &'a RuntimeMetadata, r2: &'a RuntimeMetadata) -> Self {
		if std::mem::discriminant(r1) != std::mem::discriminant(r2) {
			panic!("Only same Metadata Versions can be compared");
		};

		match (r1, r2) {
			(V13(_), V13(_)) => {}
			(V14(_), V14(_)) => {}
			_ => panic!("Unsupported versions"),
		}
		Self { r1, r2 }
	}

	fn compare_v13(&self) {
		log::debug!("Comparing v13 runtimes");
	}

	fn compare_v14(&self) {
		log::debug!("Comparing v14 runtimes");
	}

	/// This is a raw comparison based on the json serialization of the metadata
	pub fn compare(&self) {
		log::debug!("A: {:?}", self.r1);
		log::debug!("B: {:?}", self.r2);

		match (self.r1, self.r2) {
			(V13(_a), V13(_b)) => self.compare_v13(),
			(V14(_a), V14(_b)) => self.compare_v14(),
			_ => panic!("V12 is unsupported"),
		}
	}
}

#[cfg(test)]
mod test_super {
	use std::path::PathBuf;

	use wasm_loader::Source;
	use wasm_testbed::WasmTestBed;

	use super::*;

	const RUNTIME_V12: &str = "../../data/runtime_v12.wasm";
	const RUNTIME_V13: &str = "../../data/runtime_v13.wasm";
	const RUNTIME_V14: &str = "../../data/runtime_v14.wasm";

	#[test]
	#[ignore = "local data"]
	#[should_panic]
	fn test_different_variants() {
		let a = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_V13))).unwrap();
		let b = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_V14))).unwrap();
		let _differ = MetadataPartialDiffer::new(&a.metadata(), &b.metadata());
	}

	#[test]
	#[ignore = "local data"]
	fn test_v13() {
		let a = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_V13))).unwrap();
		let b = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_V13))).unwrap();
		let _differ = MetadataPartialDiffer::new(&a.metadata(), &b.metadata());
	}

	#[test]
	#[ignore = "local data"]
	fn test_v14() {
		let a = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_V14))).unwrap();
		let b = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_V14))).unwrap();
		let _differ = MetadataPartialDiffer::new(&a.metadata(), &b.metadata());
	}

	#[test]
	#[ignore = "local data"]
	#[should_panic]
	fn test_unsupported_variants() {
		let a = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_V12))).unwrap();
		let b = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_V12))).unwrap();
		let _differ = MetadataPartialDiffer::new(&a.metadata(), &b.metadata());
	}
}

use super::{changed_wapper::ChangedWrapper, reduced_runtime::*};

/// This struct holds both the ReducedRuntime and its changes.
/// It allows computing stats about the amount of changes,
/// what has changed (or not) and making the decision about wether
/// the new runtime breaks API compatibility with the reference one.
pub struct DiffAnalyzer<'a> {
	runtime_1: &'a ReducedRuntime,
	runtime_2: &'a ReducedRuntime,
	changes: &'a ChangedWrapper,
}

impl<'a> DiffAnalyzer<'a> {
	pub fn new(runtime_1: &'a ReducedRuntime, runtime_2: &'a ReducedRuntime, changes: &'a ChangedWrapper) -> Self {
		Self { runtime_1, runtime_2, changes }
	}
}

impl<'a> Compatible for DiffAnalyzer<'a> {
	fn compatible(&self) -> bool {
		match self.changes.as_ref() {
			comparable::Changed::Unchanged => true,
			comparable::Changed::Changed(c) => c
				.pallets
				.iter()
				.map(|p| match p {
					comparable::MapChange::Added(_k, _d) => true,
					comparable::MapChange::Removed(_k) => false,
					comparable::MapChange::Changed(_k, c) => c.iter().map(|x| x.compatible()).all(|x| x.into()),
				})
				.all(|x| x),
		}
	}
}

pub trait Compatible {
	// type Item = T;

	/// Consider changes, this function report whether the 2 runtimes APIs are compatible or not.
	/// This helps you decide whether the runtime `transaction_version` should be bumped or not.
	fn compatible(&self) -> bool;

	// fn incompatible_items(&self) -> Vec<T> {

	// }
}

#[cfg(test)]
mod test_diffanalyzer {
	use super::*;
	use crate::differs::{reduced::reduced_differ::ReducedDiffer, test_runtimes::*};
	use std::path::PathBuf;
	use wasm_loader::Source;
	use wasm_testbed::WasmTestBed;

	#[test]
	fn test_compatible_9260_9270() {
		let a = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_POLKADOT_V14_9260))).unwrap();
		let b = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_POLKADOT_V14_9270))).unwrap();

		let differ = ReducedDiffer::new(a.metadata(), b.metadata());
		let comp = differ.comp();
		let rr1: ReducedRuntime = a.metadata().into();
		let rr2: ReducedRuntime = b.metadata().into();

		let da = DiffAnalyzer::new(&rr1, &rr2, &comp);
		assert!(!da.compatible());
	}
}

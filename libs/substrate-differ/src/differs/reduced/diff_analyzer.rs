use super::{changed_wapper::ChangedWrapper, reduced_runtime::*};

/// This struct holds both the ReducedRuntime and its changes.
/// It allows computing stats about the amount of changes,
/// what has changed (or not) and making the decision about wether
/// the new runtime breaks API compatibility with the reference one.
pub struct DiffAnalyzer<'a> {
	// runtime_1: &'a ReducedRuntime,
	// runtime_2: &'a ReducedRuntime,
	changes: &'a ChangedWrapper,
}

impl<'a> DiffAnalyzer<'a> {
	pub fn new(_runtime_1: &'a ReducedRuntime, _runtime_2: &'a ReducedRuntime, changes: &'a ChangedWrapper) -> Self {
		Self {
			// runtime_1, runtime_2,
			changes,
		}
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
					comparable::MapChange::Changed(_k, c) => c.iter().map(|x| x.compatible()).all(|x| x),
				})
				.all(|x| x),
		}
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

	fn compare_runtimes(runtime_a: PathBuf, runtime_b: PathBuf) -> bool {
		let a = WasmTestBed::new(&Source::File(runtime_a)).unwrap();
		let b = WasmTestBed::new(&Source::File(runtime_b)).unwrap();

		let differ = ReducedDiffer::new(a.metadata(), b.metadata());
		let comp = differ.compare();
		let ra = a.metadata().into();
		let rb = b.metadata().into();
		let da = DiffAnalyzer::new(&ra, &rb, &comp);
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

	#[test]
	fn test_compatible_9260_9260() {
		assert!(compare_runtimes(
			get_runtime_file(Chain::Polkadot, 14, 9260).unwrap(),
			get_runtime_file(Chain::Polkadot, 14, 9260).unwrap(),
		));
	}

	#[test]
	fn test_compatible_9270_9270() {
		assert!(compare_runtimes(
			get_runtime_file(Chain::Polkadot, 14, 9270).unwrap(),
			get_runtime_file(Chain::Polkadot, 14, 9270).unwrap(),
		));
	}

	#[test]
	fn test_compatible_not_9260_9270() {
		assert!(!compare_runtimes(
			get_runtime_file(Chain::Polkadot, 14, 9260).unwrap(),
			get_runtime_file(Chain::Polkadot, 14, 9280).unwrap(),
		));
	}

	#[test]
	fn test_compatible_ksm_not_9280_9290() {
		assert!(!compare_runtimes(
			get_runtime_file(Chain::Kusama, 14, 9280).unwrap(),
			get_runtime_file(Chain::Kusama, 14, 9290).unwrap(),
		));
	}

	#[test]
	fn test_compatible_dot_not_9280_9290() {
		assert!(!compare_runtimes(
			get_runtime_file(Chain::Polkadot, 14, 9280).unwrap(),
			get_runtime_file(Chain::Polkadot, 14, 9290).unwrap(),
		));
	}
}

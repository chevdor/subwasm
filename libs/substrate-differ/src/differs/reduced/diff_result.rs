use frame_metadata::RuntimeMetadata;

use super::{
	changed_wapper::{ChangedWrapper, CompOutput},
	diff_analyzer::{Compatible, DiffAnalyzer},
	reduced_differ::ReducedDiffer,
	reduced_runtime::ReducedRuntime,
};
use std::fmt::Display;

pub struct ReducedDiffResult {
	runtime_a: &'static ReducedRuntime,
	runtime_b: &'static ReducedRuntime,
	changes: &'static ChangedWrapper,
	compatible: bool,
}
impl ReducedDiffResult {
	pub fn new(runtime_a: &RuntimeMetadata, runtime_b: &RuntimeMetadata) -> Self {
		let differ = ReducedDiffer::new(runtime_a, runtime_b);
		let (ra, rb) = differ.get_reduced_runtimes();

		let changes = &differ.compare();
		let da = DiffAnalyzer::new(&ra, &rb, &changes);
		let compatible = da.compatible();
		Self { runtime_a: &ra, runtime_b: &rb, changes, compatible }
	}
}

impl Display for ReducedDiffResult {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		//     let compatible = da.compatible();

		// println!("{}", changes);

		// println!(
		// 	"spec_version       : {:>4?} -> {:>4?}",
		// 	runtime_a.core_version().spec_version,
		// 	runtime_b.core_version().spec_version
		// );

		// let tx_version_a = runtime_a.core_version().transaction_version;
		// let tx_version_b = runtime_a.core_version().transaction_version;
		// println!("transaction_version: {:>4?} -> {:>4?}", tx_version_a, tx_version_b,);
		// println!("Compatible: {}", if compatible { "YES" } else { "NO" });
		// if !compatible {
		// 	if tx_version_a == tx_version_b {
		// 		eprintln!("ERROR: You need to bump the transaction_version");
		// 		std::process::exit(1)
		// 	} else {
		// 		println!("GOOD: transaction_version has been bumped already");
		// 		std::process::exit(0)
		// 	}
		// } else {
		// 	println!("OK runtimes are compatibles");
		// 	std::process::exit(0)
		// }
		todo!()
	}
}

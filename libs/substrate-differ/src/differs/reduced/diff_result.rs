use serde::Serialize;

use super::{
	changed_wapper::ChangedWrapper,
	diff_analyzer::{Compatible, DiffAnalyzer},
	reduced_differ::ReducedDiffer,
	reduced_runtime::ReducedRuntime,
};
use std::fmt::Display;

#[derive(Serialize)]
pub struct ReducedDiffResult {
	// #[serde(skip_serializing)]
	// runtime_a: ReducedRuntime,

	// #[serde(skip_serializing)]
	// runtime_b: ReducedRuntime,
	changes: Option<ChangedWrapper>,

	compatible: bool,
}

impl ReducedDiffResult {
	pub fn new(ra: ReducedRuntime, rb: ReducedRuntime) -> Self {
		// let ra=.....into():
		// let rb=.....into():

		// ReducedDiffer::compare(runtime_a, runtime_b);

		// let differ = ReducedDiffer::new(runtime_a, runtime_b);
		// let (ra, rb) = differ.get_reduced_runtimes_as_ref();
		// let (ra, rb) = (ReducedRuntime::from(runtime_a), ReducedRuntime::from(runtime_b));
		let changes = ReducedDiffer::compare(&ra, &rb);

		match changes {
			Some(changes) => {
				let da = DiffAnalyzer::new(&ra, &rb, &changes);
				let compatible = da.compatible();
				Self {
					// runtime_a: ra, runtime_b: rb,
					changes: Some(changes),
					compatible,
				}
			}
			None => Self {
				// runtime_a: ra, runtime_b: rb,
				changes: None,
				compatible: true,
			},
		}
	}
}

impl Display for ReducedDiffResult {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		// TODO: handle with match

		let _ = match &self.changes {
			Some(changes) => f.write_fmt(format_args!("{}", changes)),
			None => f.write_str("No change detected\n"),
		};

		f.write_fmt(format_args!("compatible: {}", self.compatible))

		// TODO: some work here
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
	}
}

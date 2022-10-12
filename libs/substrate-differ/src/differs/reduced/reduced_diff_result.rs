use comparable::Comparable;
use serde::Serialize;
use std::rc::Rc;

use super::{
	changed_wapper::ChangedWrapper,
	diff_analyzer::{Compatible, DiffAnalyzer},
	reduced_runtime::ReducedRuntime,
	reduced_runtime_change_wrapper::ReducedRuntimeChangeWrapper,
};
use std::fmt::Display;

#[derive(Serialize)]
pub struct ReducedDiffResult {
	/// Reference runtime. This is the runtime on the left side of the comparison.
	/// We compare [runtime_b] to [runtime_a].
	#[serde(skip_serializing)]
	pub(crate) runtime_a: Rc<ReducedRuntime>,

	/// This is the Right side of the comparison. We compare this runtime with the reference [runtime_a].
	#[serde(skip_serializing)]
	pub(crate) runtime_b: Rc<ReducedRuntime>,

	/// This is the diff between our 2 runtimes. Anything common and unchanged between the 2 runtimes
	/// is NOT part of these [changes].
	pub(crate) changes: Option<Rc<ChangedWrapper>>,

	/// After computing the [changes] we analysis the content of the changes and set this flag depending
	/// on whether we consider the runtimes compatible or not. If they are not compatible, the `transaction_version` of
	/// [runtime_b] should be bumped before releasing.
	compatible: bool,
}

impl ReducedDiffResult {
	pub fn new(ra: ReducedRuntime, rb: ReducedRuntime) -> Self {
		println!("ReducedDiffResult::new(...)");

		let instance = Self { runtime_a: Rc::new(ra), runtime_b: Rc::new(rb), changes: None, compatible: false };
		instance.init()
	}

	pub fn init(mut self) -> Self {
		let ra = self.runtime_a.clone();
		let rb = self.runtime_b.clone();
		self.changes = match ra.comparison(&rb) {
			comparable::Changed::Unchanged => None,
			comparable::Changed::Changed(reduced_runtime_change) => Some(Rc::new(ChangedWrapper::from(
				ReducedRuntimeChangeWrapper::new(reduced_runtime_change, ra.clone(), rb.clone()),
			))),
		};

		if let Some(changes) = &self.changes {
			let da = DiffAnalyzer::new(changes.clone());
			self.compatible = da.compatible();
		} else {
			self.compatible = true;
		}
		self
	}
}

impl Display for ReducedDiffResult {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let _ = match &self.changes {
			Some(changes) => f.write_fmt(format_args!("{}", changes)),
			None => f.write_str("No change detected\n"),
		};

		f.write_fmt(format_args!("compatible: {}", self.compatible))
	}
}

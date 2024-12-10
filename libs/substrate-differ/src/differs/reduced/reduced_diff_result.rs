use comparable::Comparable;
use log::trace;
use serde::Serialize;
use std::rc::Rc;

use super::{
	changed_wapper::ChangedWrapper,
	diff_analyzer::{Compatible, DiffAnalyzer, RequireTransactionVersionBump},
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
	/// on whether the `transaction_version` of [runtime_b] should be bumped before releasing.
	require_transaction_version_bump: Option<bool>,

	/// After computing the [changes] we analysis the content of the changes and set this flag depending
	/// on whether we consider the runtimes compatible or not.
	compatible: Option<bool>,
}

impl ReducedDiffResult {
	pub fn new(ra: ReducedRuntime, rb: ReducedRuntime) -> Self {
		let instance = Self {
			runtime_a: Rc::new(ra),
			runtime_b: Rc::new(rb),
			changes: None,
			require_transaction_version_bump: None,
			compatible: None,
		};
		instance.init()
	}

	pub fn init(mut self) -> Self {
		self.changes = match self.runtime_a.comparison(&self.runtime_b) {
			comparable::Changed::Unchanged => None,
			comparable::Changed::Changed(reduced_runtime_change) => {
				Some(Rc::new(ChangedWrapper::from(ReducedRuntimeChangeWrapper::new(
					reduced_runtime_change,
					self.runtime_a.clone(),
					self.runtime_b.clone(),
				))))
			}
		};

		if let Some(changes) = &self.changes {
			let da = DiffAnalyzer::new(changes.clone());
			self.require_transaction_version_bump = Some(da.require_tx_version_bump());
			self.compatible = Some(da.compatible());
		} else {
			self.require_transaction_version_bump = Some(false);
			self.compatible = Some(true);
		}

		trace!("require_transaction_version_bump: {:?}", self.require_transaction_version_bump);
		trace!("compatible: {:?}", self.compatible);
		self
	}

	pub fn require_transaction_version_bump(&self) -> bool {
		self.require_transaction_version_bump.expect("Dit not init run ?")
	}

	pub fn compatible(&self) -> bool {
		self.compatible.expect("Dit not init run ?")
	}
}

impl Display for ReducedDiffResult {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let _ = match &self.changes {
			Some(changes) => f.write_fmt(format_args!("{changes}")),
			None => f.write_str("No change detected\n"),
		};

		let _ = f.write_fmt(format_args!("SUMMARY:\n"));
		let _ = f.write_fmt(format_args!(
			"{:.<35}: {}\n",
			"- Compatible",
			self.compatible.map(|v| v.to_string()).unwrap_or(String::from("not computed"))
		));
		let _ = f.write_fmt(format_args!(
			"{:.<35}: {}\n",
			"- Require transaction_version bump",
			self.require_transaction_version_bump.map(|v| v.to_string()).unwrap_or(String::from("n/a"))
		));
		Ok(())
	}
}

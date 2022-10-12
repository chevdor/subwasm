use super::reduced_runtime::*;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ReducedRuntimeChangeWrapper {
	pub(crate) reduced_runtime_change: ReducedRuntimeChange,
	// pub(crate) reduced_runtime_a: &'a ReducedRuntime,
	// pub(crate) reduced_runtime_b: &'a ReducedRuntime,
}

impl ReducedRuntimeChangeWrapper {
	pub fn new(
		reduced_runtime_change: ReducedRuntimeChange,
		// reduced_runtime_a: &'a ReducedRuntime,
		// reduced_runtime_b: &'a ReducedRuntime,
	) -> Self {
		Self {
			reduced_runtime_change,
			// reduced_runtime_a, reduced_runtime_b
		}
	}
}

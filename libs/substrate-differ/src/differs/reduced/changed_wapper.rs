use super::reduced_runtime::*;
use comparable::Changed;
use std::fmt::Display;

pub type CompOutput = Changed<ReducedRuntimeChange>;

#[derive(Debug)]
pub struct ChangedWrapper(CompOutput);

impl From<CompOutput> for ChangedWrapper {
	fn from(c: CompOutput) -> Self {
		Self(c)
	}
}

impl Display for ChangedWrapper {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match &self.0 {
			Changed::Unchanged => f.write_str("UNCHANGED"),
			Changed::Changed(_c) => f.write_fmt(format_args!("CHANGED TODO: TELL MORE")),
		}
	}
}

impl AsRef<CompOutput> for ChangedWrapper {
	fn as_ref(&self) -> &CompOutput {
		&self.0
	}
}

use super::diff_result::DiffResult;
use std::fmt::Debug;

pub trait Diff<T: Debug + PartialEq>: PartialEq + Debug {
	fn diff(&self, other: T) -> DiffResult<T>;
}

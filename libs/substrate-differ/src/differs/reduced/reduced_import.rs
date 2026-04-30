use comparable::Comparable;
use serde::Serialize;

#[derive(Debug, PartialEq, Hash, Comparable, Serialize, Clone, Eq, PartialOrd, Ord)]
pub struct ReducedImport {
	pub module: String,
	pub name: String,
}

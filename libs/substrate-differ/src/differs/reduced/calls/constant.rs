use super::prelude::*;
use comparable::Comparable;
use serde::Serialize;
use std::fmt::Display;

/// Reduced Constant
#[derive(Debug, Serialize, Hash, Comparable, PartialOrd, Ord, PartialEq, Eq, Clone)]
pub struct Constant {
	/// Name
	pub name: String,

	/// Value
	pub value: Value,

	/// Documentation of the constant.
	#[comparable_ignore]
	pub docs: Documentation,
}

impl Constant {
	pub fn new(name: &str, value: Vec<u8>, docs: Documentation) -> Self {
		let name = name.into();
		Self { name, value, docs }
	}
}

impl Display for Constant {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let displayable_value = DisplayableVec::new(&self.value, None).init().to_short_string();
		f.write_fmt(format_args!("{}: {}", self.name, displayable_value))
	}
}

#[cfg(test)]
mod test_reduced_constant {
	use super::*;

	#[test]
	fn test_constant() {
		let c = Constant::new("transfer", vec![12, 42], vec![]);
		println!("c = {c:?}");
		assert_eq!([12, 42], c.value.as_slice());
	}
}

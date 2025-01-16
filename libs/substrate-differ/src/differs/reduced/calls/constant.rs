use super::{hashed_type::HashedType, prelude::*};
use comparable::Comparable;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};

/// Reduced Constant
#[derive(Debug, Deserialize, Serialize, Comparable, PartialEq, Clone)]
#[self_describing]
pub struct Constant {
	/// Name
	pub name: String,

	/// Value
	pub value: Value,

	/// Type
	pub ty: HashedType,

	/// Documentation of the constant.
	#[comparable_ignore]
	pub docs: Documentation,
}

impl Constant {
	pub fn new(name: &str, ty: HashedType, value: Vec<u8>, docs: Documentation) -> Self {
		Self { name: name.into(), ty, value, docs }
	}
}

impl Display for Constant {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let displayable_value = DisplayableVec::new(&self.value, None).init().to_short_string();
		f.write_fmt(format_args!("{}: {}", self.name, displayable_value))
	}
}

impl Display for ConstantChange {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Name(name) => f.write_fmt(format_args!("Name changed: {} -> {}", name.0, name.1))?,
			Self::Value(_value) => f.write_fmt(format_args!("Value changed"))?,
			Self::Ty(ty) => f.write_fmt(format_args!("Type changed: {}", ty))?,
		}
		Ok(())
	}
}

#[cfg(test)]
mod test_reduced_constant {
	use super::*;

	#[test]
	fn test_constant() {
		let c = Constant::new("transfer", "[u8; 2]".into(), vec![12, 42], vec![]);
		println!("c = {c:?}");
		assert_eq!([12, 42], c.value.as_slice());
	}
}

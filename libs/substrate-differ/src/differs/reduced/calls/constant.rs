use super::prelude::*;
use comparable::Comparable;
use serde::Serialize;
use std::{collections::BTreeSet, fmt::Display};

/// Reduced Constant
#[derive(Debug, Serialize, Hash, Comparable, PartialOrd, Ord, PartialEq, Eq)]
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
		let _ = f.write_fmt(format_args!("{}: {:?}", self.name, self.value));

		Ok(())
	}
}

pub fn variant_to_constants(td: &TypeDefVariant<PortableForm>) -> BTreeSet<Constant> {
	td.variants()
		.iter()
		.map(|vv| {
			Constant::new(
				vv.name(),
				vec![42], // TODO: That is surely NOT 42 ........
				vv.docs().iter().map(|f| f.into()).collect(),
			)
		})
		.collect()
}

#[cfg(test)]
mod test_reduced_constant {
	use super::*;

	#[test]
	fn test_constant() {
		let c = Constant::new("transfer", vec![12, 42], vec![]);
		println!("c = {:?}", c);
		assert_eq!([12, 42], c.value.as_slice());
	}
}

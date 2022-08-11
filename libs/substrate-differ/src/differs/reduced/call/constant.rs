use super::prelude::*;
use serde::Serialize;
use std::fmt::Display;

/// Reduced Constant
#[derive(Debug, PartialEq, Eq, Serialize, Hash)]
pub struct Constant {
	/// Index
	pub index: Index,

	/// Name
	pub name: String,

	/// Value
	pub value: Value,

	/// Documentation of the constant.
	pub docs: Documentation,
}

impl Constant {
	pub fn new(index: Index, name: &str, value: Vec<u8>, docs: Documentation) -> Self {
		let name = name.into();
		Self { index, name, value, docs }
	}
}

impl Display for Constant {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let _ = f.write_fmt(format_args!("{:?}: {}", self.index, self.name));

		Ok(())
	}
}

pub fn variant_to_constants(td: &TypeDefVariant<PortableForm>) -> Vec<PalletItem> {
	td.variants()
		.iter()
		.map(|vv| {
			PalletItem::Constant(Constant::new(
				vv.index() as u32,
				vv.name(),
				vec![42],
				vv.docs().iter().map(|f| f.into()).collect(),
			))
		})
		.collect()
}

#[cfg(test)]
mod test_reduced_constant {
	use super::*;

	#[test]
	fn test_constant() {
		let c = Constant::new(1, "transfer", vec![12, 42], vec![]);
		println!("c = {:?}", c);
		assert_eq!(1, c.index);
		assert_eq!([12, 42], c.value.as_slice());
	}
}

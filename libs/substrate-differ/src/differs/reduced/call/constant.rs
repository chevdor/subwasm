use super::prelude::*;
use serde::Serialize;
use std::fmt::Display;

/// Reduced Constant
#[derive(Debug, PartialEq, Eq, Serialize, Hash)]
pub struct Constant {
	pub index: Index,

	pub name: String,

	// TODO
	// /// Type of the module constant.
	// pub ty: String,
	// /// Value stored in the constant (SCALE encoded).

	// TODO: Bring that
	// pub value_hash: Hash,
	/// Documentation of the constant.
	pub docs: Documentation,
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
			PalletItem::Constant(Constant {
				index: vv.index(),
				name: vv.name().to_string(),
				docs: vv.docs().iter().map(|f| f.into()).collect(),
				// TODO
				// ty: todo!(),
				// value_hash: todo!(),
			})
		})
		.collect()
}

#[cfg(test)]
mod test_reduced_call {
	use super::*;

	#[test]
	fn test_constant() {
		let call = Constant { index: 1, name: "transfer".into(), docs: vec![] };
		println!("call = {:?}", call);
	}
}

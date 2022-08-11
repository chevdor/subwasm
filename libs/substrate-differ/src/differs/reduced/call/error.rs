use super::prelude::*;
use serde::Serialize;
use std::fmt::Display;

/// Reduced Error
#[derive(Debug, PartialEq, Eq, Serialize, Hash)]
pub struct Error {
	pub index: Index,

	pub name: String,
	pub docs: Documentation,
}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let _ = f.write_fmt(format_args!("{:?}: {}", self.index, self.name));

		Ok(())
	}
}

pub fn variant_to_errors(td: &TypeDefVariant<PortableForm>) -> Vec<PalletItem> {
	td.variants()
		.iter()
		.map(|vv| {
			PalletItem::Error(Error {
				index: vv.index(),
				name: vv.name().to_string(),
				docs: vv.docs().iter().map(|f| f.into()).collect(),
			})
		})
		.collect()
}

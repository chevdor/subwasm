use super::prelude::*;
use comparable::Comparable;
use serde::Serialize;
use std::{collections::BTreeMap, fmt::Display};

/// Reduced Error
#[derive(Debug, PartialEq, Serialize, Hash, Comparable, PartialOrd, Ord, Eq, Clone)]
pub struct Error {
	pub index: ExtrinsicId,
	pub name: String,

	#[comparable_ignore]
	pub docs: Documentation,
}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let _ = f.write_fmt(format_args!("{: >2}: {}", self.index, self.name));

		Ok(())
	}
}

pub fn variant_to_errors(td: &TypeDefVariant<PortableForm>) -> BTreeMap<PalletId, Error> {
	td.variants
		.iter()
		.map(|vv| {
			(
				vv.index as PalletId,
				Error {
					index: vv.index as PalletId,
					name: vv.name.to_string(),
					docs: vv.docs.iter().map(|f| f.into()).collect(),
				},
			)
		})
		.collect()
}

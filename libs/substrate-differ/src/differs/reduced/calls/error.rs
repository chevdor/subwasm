use super::prelude::*;
use comparable::Comparable;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fmt::Display};

/// Reduced Error
#[derive(Debug, Deserialize, Serialize, Comparable, PartialEq, Clone)]
#[self_describing]
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

impl std::fmt::Display for ErrorChange {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("{self:?}"))
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

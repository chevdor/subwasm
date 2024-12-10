use super::{
	prelude::*,
	signature::{Arg, Signature},
};
use comparable::Comparable;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Reduced Event
#[derive(Debug, PartialEq, Deserialize, Serialize, Hash, Comparable, PartialOrd, Ord, Eq, Clone)]
#[self_describing]
pub struct Event {
	pub index: ExtrinsicId,
	pub name: String,
	pub signature: Signature,

	#[comparable_ignore]
	docs: Documentation,
}

impl std::fmt::Display for Event {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let _ = f.write_fmt(format_args!("{: >2}: {} ( {} )", self.index, self.name, self.signature));

		Ok(())
	}
}

impl std::fmt::Display for EventChange {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Signature(sig) => f.write_fmt(format_args!("{}", sig))?,
			_ => f.write_fmt(format_args!("{:?}", self))?,
		}
		Ok(())
	}
}

pub fn variant_to_events(td: &TypeDefVariant<PortableForm>) -> BTreeMap<PalletId, Event> {
	td.variants
		.iter()
		.map(|vv| {
			let args = vv
				.fields
				.iter()
				.map(|f| Arg { name: f.name.clone().unwrap_or_default(), ty: f.type_name.clone().unwrap_or_default() })
				.collect();

			(
				vv.index as PalletId,
				Event {
					index: vv.index as PalletId,
					name: vv.name.to_string(),
					signature: Signature { args },
					docs: vv.docs.iter().map(|f| f.into()).collect(),
				},
			)
		})
		.collect()
}

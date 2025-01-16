use super::{fields_to_args, prelude::*, signature::Signature};
use comparable::Comparable;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::Arc;

/// Reduced Call
#[derive(Debug, Deserialize, Serialize, Comparable, PartialEq, Clone)]
#[self_describing]
pub struct Call {
	pub index: ExtrinsicId,
	pub name: String,
	pub signature: Signature,

	#[comparable_ignore]
	pub docs: Documentation,
}

impl std::fmt::Display for Call {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("{: >2}: {} ( {} )", self.index, self.name, self.signature))
	}
}

impl std::fmt::Display for CallChange {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Signature(sig) => f.write_fmt(format_args!("{}", sig))?,
			_ => f.write_fmt(format_args!("{:?}", self))?,
		}
		Ok(())
	}
}

// impl Call {
// 	pub fn comp(&self) {
// 		let c: Changed<_> = self.comparison(self);
// 		c.for_each(|x| {
// 			println!("***** x = {:#?}", x);
// 		})
// 	}
// }

pub fn variant_to_calls(
	registry: &Arc<PortableRegistry>,
	td: &TypeDefVariant<PortableForm>,
) -> BTreeMap<PalletId, Call> {
	td.variants
		.iter()
		.map(|vv| {
			let args = fields_to_args(registry, &vv.fields);

			// PalletItem::Call(PalletData {
			// 	index: Indexme(vv.index()Indexs u32),
			// 	name: vv.name().to_string(),
			// 	signature: Box::new(Signature { args }),
			// 	documentation: vv.docs().iter().map(|f| f.into()).collect(),
			// })
			(
				vv.index as PalletId,
				Call {
					index: vv.index as PalletId,
					name: vv.name.to_string(),
					signature: Signature { args },
					docs: vv.docs.iter().map(|f| f.into()).collect(),
				},
			)
		})
		.collect()
}

#[cfg(test)]
mod test_reduced_call {
	use super::*;
	use crate::differs::reduced::calls::Arg;

	#[test]
	fn test_call() {
		let call = Call {
			index: 1,
			name: "transfer".into(),
			signature: Signature {
				args: vec![
					Arg { name: "dest".into(), ty: "<T::Lookup as StaticLookup>::Source".into() },
					Arg { name: "value".into(), ty: "T::Balance".into() },
				],
			},
			docs: vec![],
		};
		println!("call = {call:?}");
	}
}

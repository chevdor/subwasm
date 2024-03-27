use super::{
	prelude::*,
	signature::{Arg, Signature},
};
use comparable::Comparable;
use serde::Serialize;
use std::{collections::BTreeMap, fmt::Display};

/// Reduced Call
#[derive(Debug, PartialEq, Serialize, Hash, Comparable, PartialOrd, Ord, Eq, Clone)]
pub struct Call {
	pub index: ExtrinsicId,
	pub name: String,
	pub signature: Signature,

	#[comparable_ignore]
	pub docs: Documentation,
}

impl Display for Call {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("{: >2}: {} ( {} )", self.index, self.name, self.signature))
	}
}

// impl Display for CallChange {
// 	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// 		f.write_fmt(format_args!("CALL {self}"))
// 	}
// }

// impl Call {
// 	pub fn comp(&self) {
// 		let c: Changed<_> = self.comparison(self);
// 		c.for_each(|x| {
// 			println!("***** x = {:#?}", x);
// 		})
// 	}
// }

pub fn variant_to_calls(td: &TypeDefVariant<PortableForm>) -> BTreeMap<PalletId, Call> {
	td.variants
		.iter()
		.map(|vv| {
			let args = vv
				.fields
				.iter()
				.map(|f| Arg { name: f.name.clone().unwrap_or_default(), ty: f.type_name.clone().unwrap_or_default() })
				.collect();

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

use super::{
	prelude::*,
	signature::{Arg, Signature},
};
use comparable::{Changed, Comparable};
use scale_info::{form::PortableForm, TypeDefVariant};
use serde::Serialize;
use std::{collections::BTreeMap, fmt::Display};

/// Reduced Call
#[derive(Debug, PartialEq, Eq, Serialize, Hash, Comparable, PartialOrd, Ord)]
pub struct Call {
	pub index: Index,
	pub name: String,
	pub signature: Signature,

	#[comparable_ignore]
	pub docs: Documentation,
}

// impl From<scale_info::TypeDef> for Call {
// 	fn from(_: scale_info::TypeDef) -> Self {
// 		todo!()
// 	}
// }

impl Display for Call {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let _ = f.write_fmt(format_args!("[{: >2}] {} ( {} )", self.index, self.name, self.signature));

		// // TODO: impl display for Signature
		// self.signature.args.iter().for_each(|a| {
		// 	let _ = f.write_fmt(format_args!("{}: {}, ", a.name, a.ty));
		// });
		// f.write_str(") ")
		Ok(())
	}
}

impl Call {
	pub fn comp(&self) {
		let c: Changed<_> = self.comparison(self);
		c.for_each(|x| {
			println!("***** x = {:#?}", x);
		})
	}
}

pub fn variant_to_calls(td: &TypeDefVariant<PortableForm>) -> BTreeMap<Index, Call> {
	td.variants()
		.iter()
		.map(|vv| {
			let args = vv
				.fields()
				.iter()
				.map(|f| Arg {
					name: f.name().unwrap_or(&String::from("")).into(),
					ty: f.type_name().unwrap_or(&String::from("")).into(),
				})
				.collect();

			// PalletItem::Call(PalletData {
			// 	index: Indexme(vv.index()Indexs u32),
			// 	name: vv.name().to_string(),
			// 	signature: Box::new(Signature { args }),
			// 	documentation: vv.docs().iter().map(|f| f.into()).collect(),
			// })
			(
				vv.index() as Index,
				Call {
					index: vv.index() as Index,
					name: vv.name().to_string(),
					signature: Signature { args },
					docs: vv.docs().iter().map(|f| f.into()).collect(),
				},
			)
		})
		.collect()
}

#[cfg(test)]
mod test_reduced_call {
	use crate::differs::reduced::calls::signature::Arg;

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
		println!("call = {:?}", call);
	}
}

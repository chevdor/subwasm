use std::{fmt::Display, sync::Arc};

use super::{pallet_data::PalletData, pallet_item::PalletItem};
use frame_metadata::StorageEntryMetadata;
use scale_info::{form::PortableForm, TypeDefVariant};
use serde::Serialize;

type ArgType = String;
type Documentation = Vec<String>;
type Index = u8; // could be u32
				 // TODO: The following is not a String
type Hash = String;

/// Reduced Call
#[derive(Debug, PartialEq, Serialize)]
pub struct Call {
	pub index: Index,
	pub name: String,
	pub signature: Signature,
	pub documentation: Documentation,
}

/// Signature of a reduced call
#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Signature {
	args: Vec<Arg>,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Arg {
	name: String,
	ty: ArgType,
}

/// Reduced Event
#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Event {
	index: Index,
	name: String,
	signature: Signature,
	documentation: Documentation,
}

/// Reduced Storage
#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Storage {
	index: Index,
	name: String,
	// Brought back down to a String to allow new runtimes adding more variants
	// modifier: String,
	// TODO: Check how to handle the following
	// ty: String,
	// Here we don't really care about the default value but its hash
	// TODO
	// default_value_hash: Hash,
	documentation: Documentation,
}

/// Reduced Constant
#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Constant {
	index: Index,

	pub name: String,

	// TODO
	// /// Type of the module constant.
	// pub ty: String,
	// /// Value stored in the constant (SCALE encoded).
	// pub value_hash: Hash,
	/// Documentation of the constant.
	pub documentation: Documentation,
}

/// Reduced Error
#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Error {
	pub index: Index,

	pub name: String,
	pub documentation: Documentation,
}

// impl From<scale_info::TypeDef> for Call {
// 	fn from(_: scale_info::TypeDef) -> Self {
// 		todo!()
// 	}
// }

impl Display for Signature {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.args.iter().for_each(|arg| {
			let _ = f.write_fmt(format_args!("x{}", arg));
		});
		Ok(())
	}
}

impl Display for Arg {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("{}: {}", self.name, self.ty))
	}
}

impl Display for Call {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let _ = f.write_fmt(format_args!("{:?}: {}( {} )", self.index, self.name, self.signature));

		// // TODO: impl display for Signature
		// self.signature.args.iter().for_each(|a| {
		// 	let _ = f.write_fmt(format_args!("{}: {}, ", a.name, a.ty));
		// });
		// f.write_str(") ")
		Ok(())
	}
}

impl Display for Event {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let _ = f.write_fmt(format_args!("{:?}: {}( {} )", self.index, self.name, self.signature));

		Ok(())
	}
}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let _ = f.write_fmt(format_args!("{:?}: {}", self.index, self.name));

		Ok(())
	}
}

impl Display for Storage {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let _ = f.write_fmt(format_args!("{:?}: {}", self.index, self.name));

		Ok(())
	}
}

impl Display for Constant {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let _ = f.write_fmt(format_args!("{:?}: {}", self.index, self.name));

		Ok(())
	}
}
pub fn variant_to_calls(td: &TypeDefVariant<PortableForm>) -> Vec<PalletItem> {
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
			PalletItem::Call(Call {
				index: vv.index(),
				name: vv.name().to_string(),
				signature: Signature { args },
				documentation: vv.docs().iter().map(|f| f.into()).collect(),
			})
		})
		.collect()
}

pub fn variant_to_events(td: &TypeDefVariant<PortableForm>) -> Vec<PalletItem> {
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

			PalletItem::Event(Event {
				index: vv.index(),
				name: vv.name().to_string(),
				signature: Signature { args },
				documentation: vv.docs().iter().map(|f| f.into()).collect(),
			})
		})
		.collect()
}

pub fn variant_to_errors(td: &TypeDefVariant<PortableForm>) -> Vec<PalletItem> {
	td.variants()
		.iter()
		.map(|vv| {
			PalletItem::Error(Error {
				index: vv.index(),
				name: vv.name().to_string(),
				documentation: vv.docs().iter().map(|f| f.into()).collect(),
			})
		})
		.collect()
}

pub fn variant_to_storage(td: &TypeDefVariant<PortableForm>) -> Vec<PalletItem> {
	td.variants()
		.iter()
		.map(|vv| {
			PalletItem::Storage(Storage {
				index: vv.index(),
				name: vv.name().to_string(),
				documentation: vv.docs().iter().map(|f| f.into()).collect(),
				// modifier: vv.field
				// TODO:
				// ty:,
				// default_value_hash: todo!(),
			})
		})
		.collect()
}

pub fn variant_to_constants(td: &TypeDefVariant<PortableForm>) -> Vec<PalletItem> {
	td.variants()
		.iter()
		.map(|vv| {
			PalletItem::Constant(Constant {
				index: vv.index(),
				name: vv.name().to_string(),
				documentation: vv.docs().iter().map(|f| f.into()).collect(),
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
	fn test_() {
		let call = Call {
			index: 1,
			name: "transfer".into(),
			signature: Signature {
				args: vec![
					Arg { name: "dest".into(), ty: "<T::Lookup as StaticLookup>::Source".into() },
					Arg { name: "value".into(), ty: "T::Balance".into() },
				],
			},
			documentation: vec![],
		};
		println!("call = {:?}", call);
	}
}

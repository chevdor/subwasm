use frame_metadata::v14;
use frame_metadata::PalletCallMetadata;
use frame_metadata::RuntimeMetadata;

use frame_metadata::RuntimeMetadata::*;
use scale_info::form::PortableForm;
use std::fmt::Debug;

use super::{pallet_data::PalletData, pallet_item::PalletItem, reduced_pallet::ReducedPallet};
use crate::differs::reduced::call::variant_to_calls;

pub type ReducedRuntimeError = String;
pub type Result<T> = core::result::Result<T, ReducedRuntimeError>;

// One of the following is wrong
impl From<&PalletCallMetadata<PortableForm>> for PalletItem {
	fn from(fn_meta: &PalletCallMetadata<PortableForm>) -> Self {
		PalletItem::Call(fn_meta.into())
	}
}

// impl From<&PalletCallMetadata<PortableForm>> for Vec<PalletItem> {
// 	fn from(fn_meta: &PalletCallMetadata<PortableForm>) -> Self {
// 		PalletItem::Call(fn_meta.into())
// 	}
// }

impl From<&PalletCallMetadata<PortableForm>> for PalletData {
	fn from(call: &PalletCallMetadata<PortableForm>) -> Self {
		Self { name: "todo".to_string(), index: None, signature: Box::new(call.ty), documentation: vec![] }
	}
}

#[derive(Debug, PartialEq)]
pub struct ReducedRuntime {
	// TODO: remove pub once we have an iterator
	pub pallets: Vec<ReducedPallet>, // TODO: Could use a BTreeMap
}

impl From<Vec<ReducedPallet>> for ReducedRuntime {
	fn from(pallets: Vec<ReducedPallet>) -> Self {
		Self { pallets }
	}
}

// TODO: impl Iterator
impl ReducedRuntime {
	#[cfg(feature = "v13")]
	/// Reduce a RuntimeMetadataV13 into a normalized ReducedRuntime
	pub fn from_v13(v13: &v13::RuntimeMetadataV13) -> Result<Self> {
		let mut pallets = convert(&v13.modules).clone();
		// TODO: we may not need to sort
		pallets.sort_by(|a, b| a.index.cmp(&b.index));

		let reduced_pallets: Vec<ReducedPallet> = pallets.iter().map(|p| p.into()).collect();
		let r_rtm: ReducedRuntime = reduced_pallets.into();
		Ok(r_rtm)
	}

	#[cfg(feature = "v14")]
	/// Reduce a RuntimeMetadataV14 into a normalized ReducedRuntime
	pub fn from_v14(v14: &v14::RuntimeMetadataV14) -> Result<Self> {
		let registry = &v14.types;
		let runtime_type = registry.resolve(v14.ty.id()).unwrap();
		println!("runtime_type = {:?}", runtime_type);
		println!("runtime_type = {:?}", runtime_type.path().segments());

		// TODO: deal with extrinsic
		let _extrinsics = &v14.extrinsic;

		let pallets = &v14.pallets;
		let reduced_pallets: Vec<ReducedPallet> = pallets
			.iter()
			.map(|p| {
				let name = &p.name;
				println!("{:?}: {:?}", &p.index, name);
				let calls_maybe = &p.calls;

				if let Some(calls) = calls_maybe {
					let id = calls.ty.id();
					let tt = registry.resolve(id.to_owned()).unwrap();

					let _ = match tt.type_def() {
						scale_info::TypeDef::Variant(v) => {
							let calls: Vec<PalletItem> = variant_to_calls(v);

							calls.iter().for_each(|call| println!("  call = {}", call));
						}
						_ => unimplemented!(),
					};
				} else {
					println!("   {} has no calls", &p.name);
				}

				ReducedPallet { index: 0, name: "junk".into(), items: vec![] }
			})
			.collect();

		let r_rtm: ReducedRuntime = reduced_pallets.into();
		Ok(r_rtm)
	}

	// pub fn diff(&self, other: &ReducedPallet) {
	// 	let r1 = self;
	// 	let r2 = other;

	// }
}

impl From<&RuntimeMetadata> for ReducedRuntime {
	fn from(runtime_metadata: &RuntimeMetadata) -> Self {
		match &runtime_metadata {
			// #[cfg(feature = "v13")]
			// V13(v13) => ReducedRuntime::from_v13(v13).unwrap(),
			#[cfg(feature = "v14")]
			V14(v14) => ReducedRuntime::from_v14(v14).unwrap(),
			_ => panic!("Unsupported metadata version"),
		}
	}
}

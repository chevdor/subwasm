use frame_metadata::v14;
use frame_metadata::PalletCallMetadata;
use frame_metadata::PalletMetadata;
use frame_metadata::RuntimeMetadata;

use frame_metadata::RuntimeMetadata::*;
use scale_info::form::PortableForm;
use scale_info::PortableRegistry;
use std::fmt::Debug;

use super::{pallet_data::PalletData, pallet_item::PalletItem, reduced_pallet::ReducedPallet};
use crate::differs::reduced::call::*;

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

	pub fn get_reduced_pallet_from_v14_pallet(
		p: &PalletMetadata<PortableForm>,
		registry: &PortableRegistry,
	) -> ReducedPallet {
		let name = &p.name;
		println!("{:?}: {:?}", &p.index, name);

		// calls
		let mut calls = if let Some(calls) = &p.calls {
			let id = calls.ty.id();
			let ty = registry.resolve(id.to_owned()).unwrap();

			match ty.type_def() {
				scale_info::TypeDef::Variant(v) => {
					let calls: Vec<PalletItem> = variant_to_calls(v);

					// calls.iter().for_each(|call| println!("  call = {}", call));
					calls
				}
				_ => unimplemented!(),
			}
		} else {
			// println!("   {} has no calls", &p.name);
			vec![]
		};

		// events
		let mut events = if let Some(item) = &p.event {
			let id = item.ty.id();
			let ty = registry.resolve(id.to_owned()).unwrap();

			match ty.type_def() {
				scale_info::TypeDef::Variant(v) => {
					let events: Vec<PalletItem> = variant_to_events(v);

					// events.iter().for_each(|event| println!("  event = {}", event));
					events
				}
				_ => unimplemented!(),
			}
		} else {
			// println!("   {} has no events", &p.name);
			vec![]
		};

		// errors
		let mut errors = if let Some(item) = &p.error {
			let id = item.ty.id();
			let ty = registry.resolve(id.to_owned()).unwrap();

			match ty.type_def() {
				scale_info::TypeDef::Variant(v) => {
					let errors: Vec<PalletItem> = variant_to_errors(v);

					// errors.iter().for_each(|error| println!("  error = {}", error));
					errors
				}
				_ => unimplemented!(),
			}
		} else {
			// println!("   {} has no errors", &p.name);
			vec![]
		};

		// storages
		let mut storages = if let Some(item) = &p.storage {
			item.entries
				.iter()
				.map(|e| {
					let s = Storage { name: e.name.clone(), docs: e.docs.clone() };
					PalletItem::Storage(s)
				})
				.collect()
		} else {
			// println!("   {} has no storage", &p.name);
			vec![]
		};

		// constants
		// todo: it is a vec
		let mut constants: Vec<PalletItem> = p
			.constants
			.iter()
			.map(|i| {
				println!("i = {:?}", i);
				// 	// let id = item.ty.id();
				// 	// let ty = registry.resolve(id.to_owned()).unwrap();

				// 	// match ty.type_def() {
				// 	// 	scale_info::TypeDef::Variant(v) => {
				// 	// 		let constants: Vec<PalletItem> = variant_to_constants(v);

				// 	// 		// constants.iter().for_each(|constant| println!("  constant = {}", constant));
				// 	// 		constants
				// 	// 	}
				// 	// 	_ => unimplemented!(),
				// 	// }
				// 	// TODO: reomve that
				let c = Constant { index: 0, name: i.name.clone(), docs: i.docs.clone() };
				PalletItem::Constant(c)
			})
			.collect();

		let mut items: Vec<PalletItem> = Vec::new();
		items.append(&mut calls);
		items.append(&mut events);
		items.append(&mut errors);
		items.append(&mut storages);
		items.append(&mut constants);
		ReducedPallet { index: 0, name: name.into(), items }
	}

	#[cfg(feature = "v14")]
	/// Reduce a RuntimeMetadataV14 into a normalized ReducedRuntime
	pub fn from_v14(v14: &v14::RuntimeMetadataV14) -> Result<Self> {
		let registry = &v14.types;
		let runtime_type = registry.resolve(v14.ty.id()).unwrap();
		println!("runtime_type = {:?}", runtime_type);
		println!("runtime_type = {:?}", runtime_type.path().segments());

		// TODO: deal with extrinsic as well
		let _extrinsics = &v14.extrinsic;

		let pallets = &v14.pallets;
		let reduced_pallets: Vec<ReducedPallet> =
			pallets.iter().map(|p| ReducedRuntime::get_reduced_pallet_from_v14_pallet(p, registry)).collect();

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

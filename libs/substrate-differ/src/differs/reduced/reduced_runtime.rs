use super::calls::call::Call;
use super::calls::error::Error;
use super::calls::event::Event;
use super::{calls::prelude::Index, pallet_data::PalletData, pallet_item::PalletItem, reduced_pallet::ReducedPallet};
use crate::differs::reduced::calls::call::variant_to_calls;
use crate::differs::reduced::calls::storage::*;
use crate::differs::reduced::calls::{constant::Constant, error::variant_to_errors, event::variant_to_events};
use comparable::Comparable;
use frame_metadata::RuntimeMetadata::*;
use frame_metadata::{v14, PalletCallMetadata, PalletMetadata, RuntimeMetadata};
use scale_info::{form::PortableForm, PortableRegistry};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::Debug;

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
		Self { name: "todo".to_string(), index: None, signature: Box::new(call.ty), docs: vec![] }
	}
}

#[derive(Debug, PartialEq, Eq, Comparable)]
pub struct ReducedRuntime {
	// TODO: remove pub once we have an iterator
	// TODO: Could use a BTreeMap
	pub pallets: HashMap<Index, ReducedPallet>,
}

// impl From<Vec<ReducedPallet>> for ReducedRuntime {
// 	fn from(pallets: Vec<ReducedPallet>) -> Self {
// 		let hashmap =
// 			HashMap::from_iter(pallets.iter().map(|p| (p.index, p)).collect::<Vec<(Index, ReducedPallet)>>());
// 		Self { pallets: hashmap }
// 	}
// }

impl From<HashMap<Index, ReducedPallet>> for ReducedRuntime {
	fn from(pallets: HashMap<Index, ReducedPallet>) -> Self {
		Self { pallets }
	}
}

// TODO: impl Iterator / IntoIterator
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

		// calls
		let calls = if let Some(calls) = &p.calls {
			let id = calls.ty.id();
			let ty = registry.resolve(id.to_owned()).unwrap();

			match ty.type_def() {
				scale_info::TypeDef::Variant(v) => {
					let calls: BTreeMap<Index, Call> = variant_to_calls(v);

					// calls.iter().for_each(|call| println!("  call = {}", call));
					calls
				}
				_ => unimplemented!(),
			}
		} else {
			// println!("   {} has no calls", &p.name);
			BTreeMap::new()
		};

		// events
		let events = if let Some(item) = &p.event {
			let id = item.ty.id();
			let ty = registry.resolve(id.to_owned()).unwrap();

			match ty.type_def() {
				scale_info::TypeDef::Variant(v) => {
					let events: BTreeMap<Index, Event> = variant_to_events(v);

					// events.iter().for_each(|event| println!("  event = {}", event));
					events
				}
				_ => unimplemented!(),
			}
		} else {
			// println!("   {} has no events", &p.name);
			BTreeMap::new()
		};

		// errors
		let errors = if let Some(item) = &p.error {
			let id = item.ty.id();
			let ty = registry.resolve(id.to_owned()).unwrap();

			match ty.type_def() {
				scale_info::TypeDef::Variant(v) => {
					let errors: BTreeMap<Index, Error> = variant_to_errors(v);
					errors
				}
				_ => unimplemented!(),
			}
		} else {
			// println!("   {} has no errors", &p.name);
			BTreeMap::new()
		};

		// storages
		let storages = if let Some(item) = &p.storage {
			item.entries
				.iter()
				.map(|e| Storage { name: e.name.clone(), docs: e.docs.clone(), default_value: e.default.clone() })
				.collect()
		} else {
			// println!("   {} has no storage", &p.name);
			BTreeSet::new()
		};

		// constants
		let constants: BTreeSet<Constant> =
			p.constants.iter().map(|i| Constant::new(&i.name, i.value.clone(), i.docs.clone())).collect();

		// let mut items: Vec<PalletItem> = Vec::new();
		// items.append(&mut calls);
		// items.append(&mut events);
		// items.append(&mut errors);
		// items.append(&mut storages);
		// items.append(&mut constants);
		ReducedPallet { index: p.index.into(), name: name.into(), calls, events, errors, constants, storages }
	}

	#[cfg(feature = "v14")]
	/// Reduce a RuntimeMetadataV14 into a normalized ReducedRuntime
	pub fn from_v14(v14: &v14::RuntimeMetadataV14) -> Result<Self> {
		let registry = &v14.types;
		// let runtime_type = registry.resolve(v14.ty.id()).unwrap();
		// println!("runtime_type = {:?}", runtime_type);
		// println!("runtime_type = {:?}", runtime_type.path().segments());

		// TODO: deal with extrinsic as well
		let _extrinsics = &v14.extrinsic;

		let pallets = &v14.pallets;
		let reduced_pallets: HashMap<Index, ReducedPallet> = pallets
			.iter()
			.map(|p| {
				let reduced_runtime = ReducedRuntime::get_reduced_pallet_from_v14_pallet(p, registry);
				(reduced_runtime.index, reduced_runtime)
			})
			.collect();

		let r_rtm: ReducedRuntime = reduced_pallets.into();
		Ok(r_rtm)
	}
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

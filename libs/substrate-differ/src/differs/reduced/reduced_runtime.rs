use super::{
	calls::{call::Call, error::Error, event::Event, prelude::PalletId},
	reduced_extrinsic::ReducedExtrinsic,
	reduced_pallet::ReducedPallet,
};
use crate::{
	differs::reduced::calls::{
		call::variant_to_calls, constant::Constant, error::variant_to_errors, event::variant_to_events, storage::*,
	},
	error::*,
};

use comparable::Comparable;
use frame_metadata::{
	v14, PalletMetadata,
	RuntimeMetadata::{self, *},
};
use scale_info::{form::PortableForm, PortableRegistry};
use serde::Serialize;
use std::fmt::Debug;
use std::{
	collections::{BTreeMap, HashMap},
	fmt::Display,
};

#[derive(Debug, PartialEq, Comparable, Serialize)]
pub struct ReducedRuntime {
	pub extrinsic: ReducedExtrinsic,
	pub pallets: HashMap<PalletId, ReducedPallet>,
}

impl ReducedRuntime {
	pub fn new(extrinsic: ReducedExtrinsic, pallets: HashMap<PalletId, ReducedPallet>) -> Self {
		Self { extrinsic, pallets }
	}

	#[cfg(feature = "v13")]
	/// Reduce a RuntimeMetadataV13 into a normalized ReducedRuntime
	pub fn from_v13(v13: &v13::RuntimeMetadataV13) -> Result<Self> {
		let mut pallets = convert(&v13.modules).clone();

		let reduced_pallets: Vec<ReducedPallet> = pallets.iter().map(|p| p.into()).collect();
		let r_rtm: ReducedRuntime = reduced_pallets.into();
		Ok(r_rtm)
	}

	#[cfg(feature = "v14")]
	pub fn get_reduced_pallet_from_v14_pallet(
		p: &PalletMetadata<PortableForm>,
		registry: &PortableRegistry,
	) -> Result<ReducedPallet> {
		let name = &p.name;

		// calls
		let calls = if let Some(calls) = &p.calls {
			let id = calls.ty.id;
			let ty = registry.resolve(id.to_owned()).ok_or_else(|| SubstrateDifferError::RegistryError(id))?;

			match &ty.type_def {
				scale_info::TypeDef::Variant(v) => {
					let calls: BTreeMap<PalletId, Call> = variant_to_calls(v);

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
			let id = item.ty.id;
			let ty = registry.resolve(id.to_owned()).ok_or_else(|| SubstrateDifferError::RegistryError(id))?;

			match &ty.type_def {
				scale_info::TypeDef::Variant(v) => {
					let events: BTreeMap<PalletId, Event> = variant_to_events(v);

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
			let id = item.ty.id;
			let ty = registry.resolve(id.to_owned()).ok_or_else(|| SubstrateDifferError::RegistryError(id))?;

			match &ty.type_def {
				scale_info::TypeDef::Variant(v) => {
					let errors: BTreeMap<PalletId, Error> = variant_to_errors(v);
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
				.map(|e| {
					(
						e.name.clone(),
						Storage {
							name: e.name.clone(),
							modifier: format!("{:?}", e.modifier),
							// ty: format!("{:?}", e.ty),
							docs: e.docs.clone(),
							default_value: e.default.clone(),
						},
					)
				})
				.collect()
		} else {
			// println!("   {} has no storage", &p.name);
			BTreeMap::new()
		};

		// constants
		let constants: BTreeMap<String, Constant> = p
			.constants
			.iter()
			.map(|i| (i.name.clone(), Constant::new(&i.name, i.value.clone(), i.docs.clone())))
			.collect();

		Ok(ReducedPallet { index: p.index.into(), name: name.into(), calls, events, errors, constants, storages })
	}

	#[cfg(feature = "v14")]
	/// Reduce a RuntimeMetadataV14 into a normalized ReducedRuntime
	pub fn from_v14(v14: &v14::RuntimeMetadataV14) -> Result<Self> {
		let registry = &v14.types;

		fn map_reduced_pallet(
			registry: &PortableRegistry,
			p: &PalletMetadata<PortableForm>,
		) -> Result<(PalletId, ReducedPallet)> {
			let reduced_pallet = ReducedRuntime::get_reduced_pallet_from_v14_pallet(p, registry)?;
			Ok((reduced_pallet.index, reduced_pallet))
		}

		let pallets = &v14.pallets;
		let reduced_pallets: Result<HashMap<PalletId, ReducedPallet>> =
			pallets.iter().map(|p| map_reduced_pallet(registry, p)).collect();
		let reduced_pallets = reduced_pallets?;

		// TODO: deal with extrinsic as well
		let extrinsic = &v14.extrinsic;
		// println!("extrinsic = {:#?}", extrinsic);
		let reduced_extrinsic = ReducedExtrinsic::from(extrinsic);

		let r_rtm = ReducedRuntime::new(reduced_extrinsic, reduced_pallets);
		Ok(r_rtm)
	}

	/// Prefer using the more efficient [get_pallet_by_id] if you can.
	pub fn get_pallet_by_name(&self, pallet_name: &str) -> Option<&ReducedPallet> {
		self.pallets
			.values()
			.find(|reduced_pallet| reduced_pallet.name.to_lowercase() == pallet_name.to_ascii_lowercase())
	}

	/// You can also use [get_pallet_by_name] but prefer using [get_pallet_by_id] where you can.
	pub fn get_pallet_by_id(&self, pallet_id: PalletId) -> Option<&ReducedPallet> {
		self.pallets.get(&pallet_id)
	}
}

impl TryFrom<&RuntimeMetadata> for ReducedRuntime {
	type Error = SubstrateDifferError;

	fn try_from(runtime_metadata: &RuntimeMetadata) -> std::result::Result<Self, Self::Error> {
		Ok(match &runtime_metadata {
			// TODO: V13 Bring back v13 eventually
			#[cfg(feature = "v13")]
			V13(v13) => ReducedRuntime::from_v13(v13)?,

			#[cfg(feature = "v14")]
			V14(v14) => ReducedRuntime::from_v14(v14)?,

			_ => panic!("Unsupported metadata version"),
		})
	}
}

impl Display for ReducedRuntime {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let _ = writeln!(f, "ReducedRuntime:");

		self.pallets.iter().for_each(|(_id, pallet)| {
			let _ = writeln!(f, "{pallet}");
		});

		Ok(())
	}
}

#[cfg(test)]
mod test_reduced_runtime {
	use super::*;
	use crate::differs::test_runtimes::{get_runtime_file, Chain, RuntimeFile};

	#[test]
	#[cfg(feature = "v14")]
	#[ignore = "local data"]
	fn test_reduce_runtime_v14_polkadot_9290() {
		use wasm_loader::Source;
		use wasm_testbed::WasmTestBed;

		let runtime_file =
			get_runtime_file(RuntimeFile::new(Chain::Polkadot, 14, 9290)).expect("Runtime file should exist");
		let _reduced_runtime: ReducedRuntime =
			WasmTestBed::new(&Source::File(runtime_file)).unwrap().metadata().try_into().unwrap();
	}

	#[test]
	#[cfg(feature = "v14")]
	#[ignore = "local data"]
	fn test_reduce_runtime_get_pallet() {
		use crate::differs::test_runtimes::{get_runtime_file, Chain, RuntimeFile};
		use wasm_loader::Source;
		use wasm_testbed::WasmTestBed;

		let runtime_file =
			get_runtime_file(RuntimeFile::new(Chain::Polkadot, 14, 9290)).expect("Runtime file should exist");
		let reduced_runtime: ReducedRuntime =
			WasmTestBed::new(&Source::File(runtime_file)).unwrap().metadata().try_into().unwrap();
		assert_eq!(0_u32, reduced_runtime.get_pallet_by_name("System").unwrap().index);
		assert_eq!(1_u32, reduced_runtime.get_pallet_by_name("Scheduler").unwrap().index);
	}

	#[test]
	#[cfg(feature = "v14")]
	#[ignore = "local data"]
	fn test_show_reduced_runtime_v14_polkadot_9290() {
		use wasm_loader::Source;
		use wasm_testbed::WasmTestBed;

		let runtime_file =
			get_runtime_file(RuntimeFile::new(Chain::Polkadot, 14, 9290)).expect("Runtime file should exist");
		let reduced_runtime: ReducedRuntime =
			WasmTestBed::new(&Source::File(runtime_file)).unwrap().metadata().try_into().unwrap();

		println!("extrinsics = {:#?}", reduced_runtime.extrinsic);
	}
}

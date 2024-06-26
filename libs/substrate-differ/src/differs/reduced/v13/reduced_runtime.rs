#![cfg(feature = "v13")]

use frame_metadata::{
	v13::{self},
	PalletCallMetadata, RuntimeMetadata,
	RuntimeMetadata::*,
};
use scale_info::{
	form::PortableForm,
	scale::{Decode, Encode},
	IntoPortable as _, MetaType, PortableRegistry, Registry, TypeInfo,
};
use serde_json::Value;
use std::fmt::Debug;

use super::{pallet_data::PalletData, pallet_item::PalletItem, reduced_pallet::ReducedPallet, signature::Signature};
use crate::differs::{
	reduced::calls::{self, variant_to_calls},
	utils::convert,
};
use std::fmt::Display;

/// Some keys are duplicate data. We remove them here.
fn purge_v13_keys(value: Value) -> Value {
	let mut serialized = value.serialize();
	let mut c = serialized.as_object_mut().unwrap().to_owned();
	let _ = c.remove("name");
	let _ = c.remove("documentation");
	Value::Object(c)
}

// TODO: V13 those impl can be made with a macro
impl From<&v13::FunctionMetadata> for PalletData {
	fn from(f: &v13::FunctionMetadata) -> Self {
		let index = None;
		let name = convert(&f.name).to_string();
		let signature = Box::new(purge_v13_keys(f.serialize()));
		let docs = convert(&f.documentation).iter().map(|s| s.to_string()).collect();
		PalletData::new(name, index, signature, docs)
	}
}

impl From<&v13::FunctionMetadata> for PalletItem {
	fn from(fn_meta: &v13::FunctionMetadata) -> Self {
		PalletItem::Call(fn_meta.into())
	}
}

impl From<&v13::EventMetadata> for PalletData {
	fn from(f: &v13::EventMetadata) -> Self {
		let index = None;
		let name = convert(&f.name).to_string();
		let signature = Box::new(purge_v13_keys(f.serialize()));
		let docs = convert(&f.documentation).iter().map(|s| s.to_string()).collect();
		PalletData::new(name, index, signature, docs)
	}
}

impl From<&v13::EventMetadata> for PalletItem {
	fn from(fn_meta: &v13::EventMetadata) -> Self {
		PalletItem::Event(fn_meta.into())
	}
}

impl From<&v13::ErrorMetadata> for PalletData {
	fn from(f: &v13::ErrorMetadata) -> Self {
		let index = None;
		let name = convert(&f.name).to_string();
		let signature = Box::new(purge_v13_keys(f.serialize()));
		let docs = convert(&f.documentation).iter().map(|s| s.to_string()).collect();
		PalletData::new(name, index, signature, docs)
	}
}

impl From<&v13::ErrorMetadata> for PalletItem {
	fn from(fn_meta: &v13::ErrorMetadata) -> Self {
		PalletItem::Error(fn_meta.into())
	}
}

impl From<&v13::ModuleConstantMetadata> for PalletData {
	fn from(f: &v13::ModuleConstantMetadata) -> Self {
		let index = None;
		let name = convert(&f.name).to_string();
		let signature = Box::new(purge_v13_keys(f.serialize()));
		let docs = convert(&f.documentation).iter().map(|s| s.to_string()).collect();
		PalletData::new(name, index, signature, docs)
	}
}

impl From<&v13::ModuleConstantMetadata> for PalletItem {
	fn from(fn_meta: &v13::ModuleConstantMetadata) -> Self {
		PalletItem::Constant(fn_meta.into())
	}
}

impl From<&v13::StorageEntryMetadata> for PalletData {
	fn from(f: &v13::StorageEntryMetadata) -> Self {
		let index = None;
		let name = convert(&f.name).to_string();
		let signature = Box::new(purge_v13_keys(f.serialize()));
		let docs = convert(&f.documentation).iter().map(|s| s.to_string()).collect();
		PalletData::new(name, index, signature, docs)
	}
}

impl From<&v13::StorageEntryMetadata> for PalletItem {
	fn from(fn_meta: &v13::StorageEntryMetadata) -> Self {
		PalletItem::Storage(fn_meta.into())
	}
}

impl From<&v14::PalletCallMetadata> for PalletData {
	fn from(f: &v14::PalletCallMetadata) -> Self {
		let meta_type = f.ty;
		let _ti = meta_type.type_info();

		let _index = meta_type.type_id();
		let _name = String::new();

		todo!("V13");
		// PalletData::new(name, index, signature, documentation)
	}
}

#[derive(Debug, PartialEq)]
pub struct ReducedRuntime {
	pub pallets: Vec<ReducedPallet>,
}

impl From<Vec<ReducedPallet>> for ReducedRuntime {
	fn from(pallets: Vec<ReducedPallet>) -> Self {
		Self { pallets }
	}
}

#[cfg(test)]
mod test_reduced_conversion {
	use crate::differs::reduced::reduced_runtime;

	use super::*;
	use std::path::PathBuf;
	use wasm_loader::Source;
	use wasm_testbed::WasmTestBed;

	#[test]
	#[cfg(feature = "v13")]
	fn test_reduce_v13() {
		let runtime_v13 = get_runtime_file(RuntimeFile::new(Chain::Polkadot, 13, 9030)).unwrap();
		let testbed = WasmTestBed::new(&Source::File(PathBuf::from(runtime_v13))).unwrap();
		let metadata = testbed.metadata();
		match metadata {
			V13(v13) => {
				let rrtm = reduced_runtime::ReducedRuntime::from_v13(v13).unwrap();
				println!("rrtm = {:#?}", rrtm);
				assert_eq!(31, rrtm.pallets.len());
			}
			_ => unreachable!(),
		}
	}

	#[test]
	fn test_reduce_v13() {
		let testbed = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_V14))).unwrap();
		let metadata = testbed.metadata();

		match metadata {
			V13(v13) => {
				let rrtm = reduced_runtime::ReducedRuntime::from_v13(v14).unwrap();
				// println!("rrtm = {:#?}", rrtm);

				assert_eq!(rrtm.pallets.len(), 30);

				let first_pallet = &rrtm.pallets[0];
				assert_eq!(0, first_pallet.index);
				assert_eq!("System", first_pallet.name);

				assert!(first_pallet.items.len() > 0);
			}
			_ => unreachable!(),
		}
	}
}

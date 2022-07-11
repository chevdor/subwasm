use frame_metadata::{
	v13::{self},
	v14, RuntimeMetadata,
	RuntimeMetadata::*,
};
// use scale_info::form::{Form, PortableForm};
use serde_json::Value;
use std::fmt::Debug;

use super::{pallet_data::PalletData, pallet_item::PalletItem, reduced_pallet::ReducedPallet, signature::Signature};
use crate::differs::utils::convert;

pub type ReducedRuntimeError = String;
pub type Result<T> = core::result::Result<T, ReducedRuntimeError>;

/// Some keys are duplicate data. We remove them here.
fn purge_v13_keys(value: Value) -> Value {
	let mut serialized = value.serialize();
	let mut c = serialized.as_object_mut().unwrap().to_owned(); // TODO: could use a match and prevent the unwrap()

	// println!("c before = {:?}", &c);
	let _ = c.remove("name");
	let _ = c.remove("documentation");
	// println!("c after = {:?}", &c);
	Value::Object(c)
}

// TODO those impl can be made with a macro
impl From<&v13::FunctionMetadata> for PalletData {
	fn from(f: &v13::FunctionMetadata) -> Self {
		let index = None;
		let name = convert(&f.name).to_string();
		let signature = Box::new(purge_v13_keys(f.serialize()));
		let documentation = convert(&f.documentation).iter().map(|s| s.to_string()).collect();
		PalletData::new(name, index, signature, documentation)
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
		let documentation = convert(&f.documentation).iter().map(|s| s.to_string()).collect();
		PalletData::new(name, index, signature, documentation)
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
		let documentation = convert(&f.documentation).iter().map(|s| s.to_string()).collect();
		PalletData::new(name, index, signature, documentation)
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
		let documentation = convert(&f.documentation).iter().map(|s| s.to_string()).collect();
		PalletData::new(name, index, signature, documentation)
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
		let documentation = convert(&f.documentation).iter().map(|s| s.to_string()).collect();
		PalletData::new(name, index, signature, documentation)
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

		todo!();
		// PalletData::new(name, index, signature, documentation)
	}
}

impl From<&v14::PalletCallMetadata> for PalletItem {
	fn from(fn_meta: &v14::PalletCallMetadata) -> Self {
		PalletItem::Call(fn_meta.into())
	}
}

#[derive(Debug, PartialEq)]
pub struct ReducedRuntime {
	// TODO: remove pub once we have an iterator
	pub pallets: Vec<ReducedPallet>, // TODO: Could use a BTreeMap
}

// impl SliceIndex<[ReducedPallet] for u32 {

// }

impl From<Vec<ReducedPallet>> for ReducedRuntime {
	fn from(pallets: Vec<ReducedPallet>) -> Self {
		Self { pallets }
	}
}

// TODO: impl Iterator
impl ReducedRuntime {
	/// Reduce a RuntimeMetadataV13 into a normalized ReducedRuntime
	pub fn from_v13(v13: &v13::RuntimeMetadataV13) -> Result<Self> {
		let mut pallets = convert(&v13.modules).clone();
		pallets.sort_by(|a, b| a.index.cmp(&b.index)); // TODO: we may not need to sort
		let reduced_pallets: Vec<ReducedPallet> = pallets.iter().map(|p| p.into()).collect();
		let r_rtm: ReducedRuntime = reduced_pallets.into();
		Ok(r_rtm)
	}

	/// Reduce a RuntimeMetadataV14 into a normalized ReducedRuntime
	pub fn from_v14(_v14: &v14::RuntimeMetadataV14) -> Result<Self> {
		todo!()
	}

	// pub fn diff(&self, other: &ReducedPallet) {
	// 	let r1 = self;
	// 	let r2 = other;

	// }
}

impl From<&RuntimeMetadata> for ReducedRuntime {
	fn from(runtime_metadata: &RuntimeMetadata) -> Self {
		match &runtime_metadata {
			V13(v13) => ReducedRuntime::from_v13(v13).unwrap(),
			V14(v14) => ReducedRuntime::from_v14(v14).unwrap(),
			_ => panic!("Unsupported metadata version"),
		}
	}
}

#[cfg(test)]
mod test_reduced_conversion {
	use crate::differs::reduced::reduced_runtime;

	use super::*;
	use std::path::PathBuf;
	use wasm_loader::Source;
	use wasm_testbed::WasmTestBed;

	// TODO: put that in a 	single file
	// const RUNTIME_V12: &str = "../../data/runtime_v12.wasm";
	const RUNTIME_V13: &str = "../../data/polkadot/V13/polkadot-9030.wasm";
	const RUNTIME_V14: &str = "../../data/polkadot/V14/polkadot_runtime.compact.compressed.wasm";

	#[test]
	fn test_reduce_v13() {
		let testbed = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_V13))).unwrap();
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
	fn test_reduce_v14() {
		let testbed = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_V14))).unwrap();
		let metadata = testbed.metadata();
		match metadata {
			V14(v14) => {
				let rrtm = reduced_runtime::ReducedRuntime::from_v14(v14).unwrap();
				println!("rrtm = {:#?}", rrtm);
				assert_eq!(rrtm.pallets.len(), 9);
			}
			_ => unreachable!(),
		}
	}
}

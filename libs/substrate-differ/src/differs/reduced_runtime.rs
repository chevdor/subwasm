use frame_metadata::{
	v13::{self},
	v14, RuntimeMetadata,
	RuntimeMetadata::*,
};
use serde_json::{Map, Value};
use std::{convert::TryInto, fmt::Debug};

use crate::differs::utils::convert;

pub type ReducedRuntimeError = String;
pub type Result<T> = core::result::Result<T, ReducedRuntimeError>;

#[derive(Debug)]
struct PalletData {
	name: String,
	index: Option<u32>,
	signature: Box<dyn Signature>,
	// TODO: remove signature, add arguments
	documentation: Vec<String>,
}

impl PartialEq for PalletData {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name
			&& self.index == other.index
			&& self.signature.serialize() == other.signature.serialize()
	}

	fn ne(&self, other: &Self) -> bool {
		!self.eq(other)
	}
}

impl Debug for dyn Signature {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self.serialize().to_string())
	}
}

trait Signature {
	fn serialize(&self) -> Value;
	// fn eq(&self, other: &dyn Signature) -> bool;
}

impl<S: serde::ser::Serialize> Signature for S {
	fn serialize(&self) -> Value {
		serde_json::to_value(self).unwrap()
	}

	// fn eq(&self, other: &dyn Signature) -> bool {
	// 	serde_json::to_value(self) == serde_json::to_value(other)
	// }
}

#[derive(Debug, PartialEq)]
enum PalletItem {
	Call(PalletData),
	Event(PalletData),
	Error(PalletData),
	Storage(PalletData),
	Constant(PalletData),
}

// fn is_documentation(v: &Map<String, Value>) -> bool {
//     println!("v = {:?}", v);
// 	true
// }

fn purge_v13_keys(value: Value) -> Value {
	let mut serialized = value.serialize();
	let mut c = serialized.as_object_mut().unwrap().to_owned(); // TODO: could use a match and prevent the unwrap()
	println!("c before = {:?}", &c);
	let _ = c.remove("name");
	let _ = c.remove("documentation");
	println!("c after = {:?}", &c);
	Value::Object(c)
}

// TODO those impl can be made with a macro
impl From<&v13::FunctionMetadata> for PalletData {
	fn from(f: &v13::FunctionMetadata) -> Self {
		let index = None;
		let name = convert(&f.name).to_string();
		let signature = Box::new(purge_v13_keys(f.serialize()));
		let documentation = convert(&f.documentation).iter().map(|s| s.to_string()).collect();
		PalletData { index, name, signature, documentation }
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
		// let mut serialized = f.serialize();
		// let mut c = serialized.as_object_mut().unwrap().to_owned(); // TODO: could use a match and prevent the unwrap()
		// println!("c before = {:?}", &c);
		// let _ = c.remove("name");
		// let _ = c.remove("documentation");
		// println!("c after = {:?}", &c);

		let signature = Box::new(purge_v13_keys(f.serialize()));
		let documentation = convert(&f.documentation).iter().map(|s| s.to_string()).collect();
		PalletData { index, name, signature, documentation }
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
		PalletData { index, name, signature, documentation }
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
		PalletData { index, name, signature, documentation }
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
		PalletData { index, name, signature, documentation }
	}
}

impl From<&v13::StorageEntryMetadata> for PalletItem {
	fn from(fn_meta: &v13::StorageEntryMetadata) -> Self {
		PalletItem::Storage(fn_meta.into())
	}
}

// type Signature = Box<dyn MySerialize>;

#[derive(Debug, PartialEq)]
pub struct ReducedPallet {
	/// Index of the pallet
	index: u32,

	/// Name of the pallet
	name: String,

	/// Vec of all the `PalletItem`
	items: Option<Vec<PalletItem>>,
}

// TODO: impl Iterator
impl ReducedPallet {
	fn new(index: u32, name: String) -> Self {
		Self { index, name, items: Some(Vec::new()) }
	}

	fn push(self, item: PalletItem) {
		if let Some(mut items) = self.items {
			items.push(item);
		}
	}
}

#[cfg(test)]
impl Default for ReducedPallet {
	fn default() -> Self {
		Self { index: 42, name: "Foobar".into(), items: None }
	}
}

impl From<&v13::ModuleMetadata> for ReducedPallet {
	fn from(v13: &v13::ModuleMetadata) -> Self {
		let index = v13.index.into();
		let name = convert(&v13.name).to_string();
		let mut items: Vec<PalletItem> = Vec::new();

		// Calls
		let calls = match &v13.calls.as_ref() {
			Some(items) => {
				let pallet_items: Vec<PalletItem> = convert(items).iter().map(|c| c.into()).collect();
				Some(pallet_items)
			}
			None => None,
		};

		if let Some(mut c) = calls {
			println!("calls = {:?}", c.len());
			items.append(&mut c);
		}
		// Events
		let events = match &v13.event.as_ref() {
			Some(items) => {
				let pallet_items: Vec<PalletItem> = convert(items).iter().map(|c| c.into()).collect();
				Some(pallet_items)
			}
			None => None,
		};

		if let Some(mut c) = events {
			println!("events = {:?}", c.len());
			items.append(&mut c);
		}

		// Storage
		let storage = match &v13.storage.as_ref() {
			Some(items) => {
				// let pallet_items: Vec<PalletItem> = convert(items).iter().map(|c| c.into()).collect();
				let pallet_items: Vec<PalletItem> = convert(&convert(items).entries).iter().map(|c| c.into()).collect();
				Some(pallet_items)
			}
			None => None,
		};

		if let Some(mut c) = storage {
			println!("storage = {:?}", c.len());
			items.append(&mut c);
		}

		// Errors
		let mut errors: Vec<PalletItem> = convert(&v13.errors).iter().map(|c| c.into()).collect();
		println!("errors = {:?}", errors.len());
		items.append(&mut errors);

		// Constants
		let mut constants: Vec<PalletItem> = convert(&v13.constants).iter().map(|c| c.into()).collect();
		println!("constants = {:?}", constants.len());
		items.append(&mut constants);

		let items = if items.is_empty() { None } else { Some(items) };

		Self { index, name, items }
	}
}

impl From<&v14::PalletMetadata> for ReducedPallet {
	fn from(_: &v14::PalletMetadata) -> Self {
		todo!()
	}
}

// impl TryFrom<ModuleMetadata> for ReducedPallet {
//    	type Error = &'static str;

//     fn try_from(m: ModuleMetadata) -> std::result::Result<Self, Self::Error> {
//         todo!()
//     }
// }

#[derive(Debug)]
pub struct ReducedRuntime {
	pallets: Option<Vec<ReducedPallet>>, // TODO: Could use a BTreeMap
}

impl From<Vec<ReducedPallet>> for ReducedRuntime {
	fn from(reduced_palets: Vec<ReducedPallet>) -> Self {
		Self { pallets: Some(reduced_palets) }
	}
}

// TODO: impl Iterator
impl ReducedRuntime {
	fn new() -> Self {
		Self { pallets: Some(Vec::new()) }
	}

	/// Add a reduced pallet to the reduced runtime
	fn push(self, pallet: ReducedPallet) {
		if let Some(mut pallets) = self.pallets {
			pallets.push(pallet);
		}
	}

	/// Reduce a RuntimeMetadataV13 into a normalized ReducedRuntime
	pub fn from_v13(v13: &v13::RuntimeMetadataV13) -> Result<Self> {
		// println!("v13 = {:?}", v13);
		// let r_rtm = Self::new();

		let mut pallets = convert(&v13.modules).clone();
		pallets.sort_by(|a, b| a.index.cmp(&b.index)); // TODO: we may not need to sort

		let reduced_pallets: Vec<ReducedPallet> = pallets.iter().map(|p| p.into()).collect();
		let r_rtm: ReducedRuntime = reduced_pallets.into();

		// 	let r_pallet = ReducedPallet::new(pallet.index.into(), convert(&pallet.name).clone());
		// 	r_rtm.push(r_pallet);
		// });

		// pallets.iter().for_each(|pallet| {
		// 	let r_pallet = ReducedPallet::new(pallet.index.into(), convert(&pallet.name).clone());
		// 	r_rtm.push(r_pallet);
		// });

		// todo!("I am on it !")
		// Err(String::from("tbd"))
		Ok(r_rtm)
	}

	/// Reduce a RuntimeMetadataV14 into a normalized ReducedRuntime
	pub fn from_v14(_v14: &v14::RuntimeMetadataV14) -> Result<Self> {
		todo!()
	}
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
	use crate::differs::reduced_runtime;

	use super::*;
	use std::path::PathBuf;
	use wasm_loader::Source;
	use wasm_testbed::WasmTestBed;

	// TODO: put that in a 	single file
	// const RUNTIME_V12: &str = "../../data/runtime_v12.wasm";
	const RUNTIME_V13: &str = "../../data/runtime_v13.wasm";
	// const RUNTIME_V14: &str = "../../data/runtime_v14.wasm";

	#[test]
	fn test_reduce_v13() {
		let testbed = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_V13))).unwrap();
		let metadata = testbed.metadata();
		match metadata {
			V13(v13) => {
				let rrtm = ReducedRuntime::new();
				let rrtm = reduced_runtime::ReducedRuntime::from_v13(v13);
				println!("rrtm = {:#?}", rrtm);
				assert!(rrtm.is_ok());
			}
			_ => unreachable!(),
		}
	}

	#[test]
	#[ignore = "todo"]
	fn test_reduce_v14() {
		todo!();
	}
}

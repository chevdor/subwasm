use frame_metadata::{
	v12::ModuleMetadata,
	v13::{self, FunctionMetadata},
	v14, RuntimeMetadata,
	RuntimeMetadata::*,
};
use serde_json::Value;
use std::{convert::TryFrom, fmt::Debug};

use crate::differs::utils::convert;

pub type ReducedRuntimeError = String;
pub type Result<T> = core::result::Result<T, ReducedRuntimeError>;

#[derive(Debug)]
struct PalletData {
	name: String,
	index: Option<u32>,
	signature: Box<dyn Signature>,
	// TODO: remove signature, add arguments + documentation
}

impl Debug for dyn Signature {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self.serialize().to_string())
	}
}

#[derive(Debug)]
enum PalletItem {
	Call(PalletData),
	Event(PalletData),
	Error(PalletData),
	Storage(PalletData),
	Constant(PalletData),
}

impl From<&v13::FunctionMetadata> for PalletData {
	fn from(f: &v13::FunctionMetadata) -> Self {
		let index = None;
		let name = convert(&f.name).to_string();
		let signature = Box::new(f.serialize());
		PalletData { index, name, signature }
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
		let signature = Box::new(f.serialize());
		PalletData { index, name, signature }
	}
}

impl From<&v13::EventMetadata> for PalletItem {
	fn from(fn_meta: &v13::EventMetadata) -> Self {
		PalletItem::Call(fn_meta.into())
	}
}

// type Signature = Box<dyn MySerialize>;

#[derive(Debug)]
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

		// Errors
		// let errors = match &v13.errors {
		// 	Some(items) => {
		// 		let pallet_items: Vec<PalletItem> = convert(items).iter().map(|c| c.into()).collect();
		// 		Some(pallet_items)
		// 	},
		// 	None => None,
		// };

		// if let Some(mut c) = events {
		// 	println!("events = {:?}", c.len());
		// 	items.append(&mut c);
		// }

		// TODO
		// Storage
		// let storage_items = match &v13.storage.as_ref() {
		// 	Some(items) => {
		// 		let pallet_items: Vec<PalletItem> = convert(items).iter().map(|c| c.into()).collect();
		// 		Some(pallet_items)
		// 	}
		// 	None => None,
		// };
		// if let Some(mut c) = storage_items {
		// 	println!("storage = {:?}", c.len());
		// 	items.append(&mut c);
		// }
		// println!("pallet_items = {:#?}", pallet_items);

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

trait Signature {
	fn serialize(&self) -> Value;
}

impl<S: serde::ser::Serialize> Signature for S {
	fn serialize(&self) -> Value {
		serde_json::to_value(self).unwrap()
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
				let rrtm = reduced_runtime::ReducedRuntime::from_v13(v13); // TODO: fix that
				// println!("rrtm = {:#?}", rrtm);
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

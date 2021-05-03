use codec::Decode;
use frame_metadata_subsee::{RuntimeMetadata, RuntimeMetadataPrefixed}; // TODO: Check v12, v13,
use sc_executor::{CallInWasm, RuntimeVersion, WasmExecutionMethod, WasmExecutor};
use sp_io::TestExternalities;
use sp_wasm_interface::HostFunctions;
use std::fmt;
use substrate_runtime_proposal_hash::{get_result, SrhResult};
use wasm_loader::*;

/// This is a "magic" number signaling that out Wasm is a substrate wasm.
type ReservedMeta = [u8; 4];
const META: ReservedMeta = [0x6d, 0x65, 0x74, 0x61]; // 1635018093 in decimal, 'atem' as string...

/// A Substrate wasm runtime testbed. This is a (very) minimum environment that allows loading and
/// executing *some* calls into the wasm. Note that any call that most calls that requires storage will fail
/// (for instance `balances.transfer`, `system.remark`, ...)
pub struct WasmTestBed {
	/// The WASM as bytes
	wasm: Vec<u8>,

	/// Prefixed version of the RuntimeMetadata.
	runtime_metadata_prefixed: RuntimeMetadataPrefixed,

	/// Metadata version
	metadata_version: u8,

	/// Core version as reported by the runtime
	core_version: Option<RuntimeVersion>,
}

impl fmt::Debug for WasmTestBed {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("SubstrateWasm")
			.field("metadata_version", &self.metadata_version)
			.field("wasm length", &self.wasm.len())
			.finish()
	}
}

impl WasmTestBed {
	pub fn new(source: Source) -> Result<Self, String> {
		let wasm = WasmLoader::load(source)?;
		let encoded = Self::call(&wasm, "Metadata_metadata", &[])?;
		let decoded_metadata = <Vec<u8>>::decode(&mut &encoded[..]).expect("Decoding");

		if !WasmTestBed::is_substrate_wasm(&decoded_metadata) {
			panic!("Not a valid Substrate runtime");
		}

		let runtime_metadata_prefixed: RuntimeMetadataPrefixed =
			codec::Decode::decode(&mut &decoded_metadata[..]).expect("Decoding RuntimeMetadataPrefixed");

		let core_version = Self::get_core_version(&wasm);
		let metadata_version = Self::get_metadata_version(&decoded_metadata);

		Ok(Self { wasm, runtime_metadata_prefixed, metadata_version, core_version })
	}

	/// Call a function in the provided wasm. Note that we can only call a few limited set of functions
	/// as we have no blocks, storage, etc...
	fn call(wasm: &[u8], method: &str, call_data: &[u8]) -> Result<Vec<u8>, String> {
		let mut ext = TestExternalities::default();
		let mut ext = ext.ext();
		let executor = WasmExecutor::new(
			WasmExecutionMethod::Interpreted,
			Some(16), // at least 12 for polkadot v12
			sp_io::SubstrateHostFunctions::host_functions(),
			8,
			None,
		);
		executor.call_in_wasm(&wasm, None, method, call_data, &mut ext, sp_core::traits::MissingHostFunctions::Allow)
	}

	fn is_substrate_wasm(data: &[u8]) -> bool {
		data[0..4] == META
	}

	fn get_metadata_version(data: &[u8]) -> u8 {
		data[4]
	}

	pub fn get_core_version(wasm: &[u8]) -> Option<RuntimeVersion> {
		let encoded = Self::call(&wasm, "Core_version", &[]).unwrap();
		<RuntimeVersion>::decode(&mut &encoded[..]).ok()
	}

	/// We probably don't need to maintain this as decoding the runtime will
	/// tell us whether the version is supported or not.
	pub fn is_supported(&self) -> bool {
		matches!(self.metadata_version, x if x >= 12)
	}

	/// Get a reference to the substrate wasm's metadata version.
	pub fn metadata_version(&self) -> &u8 {
		&self.metadata_version
	}

	/// Get a reference to the substrate wasm's runtime metadata prefixed.
	pub fn runtime_metadata_prefixed(&self) -> &RuntimeMetadataPrefixed {
		&self.runtime_metadata_prefixed
	}

	/// Get the `RuntimeMetada`
	pub fn metadata(&self) -> &RuntimeMetadata {
		&self.runtime_metadata_prefixed.1
	}

	/// Get a reference to the substrate wasm's core version.
	pub fn core_version(&self) -> &Option<RuntimeVersion> {
		&self.core_version
	}

	/// Compute the proposal hash of the runtime
	pub fn proposal_hash(&self) -> String {
		let result: SrhResult = get_result(&self.wasm);
		format!("0x{}", &result.encodedd_hash)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use sp_runtime::RuntimeString;
	use std::path::PathBuf;

	const WASM_NO_SUBSTRATE: &str = "../../data/wasm/qjs.wasm";
	const KUSAMA_1050_VXX: &str = "../../data/kusama/kusama-1050.wasm";
	const KUSAMA_1062_VXX: &str = "../../data/kusama/kusama-1062.wasm";
	const KUSAMA_2030_VXX: &str = "../../data/kusama/kusama-2030.wasm";
	const POLKADOT_01_V11: &str = "../../data/polkadot/polkadot-01.wasm";
	const POLKADOT_29_V12: &str = "../../data/polkadot/polkadot-29.wasm";

	#[cfg(test)]
	mod common {
		use super::*;
		#[test]
		#[ignore = "need data"]
		fn it_panics_on_non_substrate_wasm() {
			assert!(WasmTestBed::new(Source::File(PathBuf::from(WASM_NO_SUBSTRATE))).is_err());
		}
	}

	#[cfg(test)]
	mod kusama {
		use super::*;

		#[test]
		#[should_panic]
		#[ignore = "need data"]
		fn it_loads_kusama_1050() {
			let runtime = WasmTestBed::new(Source::File(PathBuf::from(KUSAMA_1050_VXX))).unwrap();
			println!("{:#?}", runtime);
			assert!(runtime.metadata_version == 11);
			assert!(runtime.core_version.is_none());
			assert!(runtime.is_supported());
		}

		#[test]
		#[should_panic]
		#[ignore = "need data"]
		fn it_loads_kusama_1062() {
			let runtime = WasmTestBed::new(Source::File(PathBuf::from(KUSAMA_1062_VXX))).unwrap();
			println!("{:#?}", runtime);
			assert!(runtime.metadata_version == 11);
			assert!(runtime.is_supported());

			let v = &runtime.core_version.unwrap();
			assert!(v.spec_name == RuntimeString::from("kusama"));
			assert!(v.impl_name == RuntimeString::from("parity-kusama"));
			assert!(v.authoring_version == 2);
			assert!(v.spec_version == 1062);
			assert!(v.impl_version == 0);
			assert!(v.apis.len() == 12);
		}

		#[test]
		#[ignore = "need data"]
		fn it_loads_kusama_metdata() {
			let runtime = WasmTestBed::new(Source::File(PathBuf::from(KUSAMA_2030_VXX))).unwrap();
			println!("{:#?}", runtime);
			assert!(runtime.metadata_version == 12);
			assert!(runtime.is_supported());

			// let rmp = runtime.runtime_metadata_prefixed();
			// println!("{:#?}", rmp);
		}

		#[test]
		#[ignore = "need data"]
		fn it_loads_kusama_2030() {
			let runtime = WasmTestBed::new(Source::File(PathBuf::from(KUSAMA_2030_VXX))).unwrap();
			println!("{:#?}", runtime);
			assert!(runtime.metadata_version == 12);
			assert!(runtime.is_supported());

			let v = &runtime.core_version.unwrap();
			assert!(v.spec_name == RuntimeString::from("kusama"));
			assert!(v.impl_name == RuntimeString::from("parity-kusama"));
			assert!(v.authoring_version == 2);
			assert!(v.spec_version == 2030);
			assert!(v.impl_version == 0);
			assert!(v.apis.len() == 12);
		}
	}

	#[cfg(test)]
	mod polkadot {
		use super::*;

		#[test]
		#[should_panic]
		#[ignore = "need data"]
		fn it_loads_polkadot_01() {
			let runtime = WasmTestBed::new(Source::File(PathBuf::from(POLKADOT_01_V11))).unwrap();
			println!("{:#?}", runtime);
			assert!(runtime.metadata_version == 11);
			assert!(runtime.is_supported());
		}

		#[test]
		#[ignore = "need data"]
		fn it_loads_polkadot_29() {
			let runtime = WasmTestBed::new(Source::File(PathBuf::from(POLKADOT_29_V12))).unwrap();

			println!("{:#?}", runtime);

			assert!(runtime.metadata_version == 12);
			assert!(runtime.is_supported());
		}
	}
}

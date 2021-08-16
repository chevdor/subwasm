mod error;
mod logger_mock;

pub use error::{Result, WasmTestbedError};
use frame_metadata::{RuntimeMetadata, RuntimeMetadataPrefixed};
use sc_executor::{CallInWasm, RuntimeVersion, WasmExecutionMethod, WasmExecutor};
use scale::Decode;
use sp_core::Hasher;
use sp_runtime::traits::BlakeTwo256;
use sp_wasm_interface::HostFunctions;
use std::fmt;
use substrate_runtime_proposal_hash::{get_parachainsystem_authorize_upgrade, get_result, SrhResult};
use wasm_loader::*;

/// This is a "magic" number signaling that out Wasm is a substrate wasm.
pub type ReservedMeta = [u8; 4];
const META: ReservedMeta = [0x6d, 0x65, 0x74, 0x61]; // 1635018093 in decimal, 'atem' as string...

/// A Substrate wasm runtime testbed. This is a (very) minimum environment that allows loading and
/// executing *some* calls into the wasm. Note that any call that most calls that requires storage will fail
/// (for instance `balances.transfer`, `system.remark`, ...)
pub struct WasmTestBed {
	/// The WASM as bytes, it has been uncompressed as needed here.
	wasm: Vec<u8>,

	/// Original bytes, before (de)compression
	bytes: Vec<u8>,

	compression: Compression,

	/// Prefixed version of the RuntimeMetadata.
	runtime_metadata_prefixed: RuntimeMetadataPrefixed,

	metadata: Vec<u8>,

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
	pub fn new(source: &Source) -> Result<Self> {
		let loader = WasmLoader::load_from_source(source).map_err(|_| WasmTestbedError::Loading(source.to_string()))?;

		let wasm = loader.bytes().to_vec();
		let metadata_encoded = Self::call(&wasm, "Metadata_metadata", &[])?;
		let metadata =
			<Vec<u8>>::decode(&mut &metadata_encoded[..]).map_err(|_| WasmTestbedError::Decoding(metadata_encoded))?;

		log::debug!("decoded_metadata bytes, length: {}", metadata.len());
		if !WasmTestBed::is_substrate_wasm(&metadata) {
			return Err(WasmTestbedError::Unsupported);
		}

		// Self::print_magic_and_version(&metadata);

		let runtime_metadata_prefixed: RuntimeMetadataPrefixed =
			scale::Decode::decode(&mut &metadata[..]).map_err(|e| {
				log::error!("e = {:#?}", e);
				WasmTestbedError::Decoding(metadata[..128].to_vec())
			})?;

		let core_version = Self::get_core_version(&wasm);
		let metadata_version = Self::get_metadata_version(&metadata);

		Ok(Self {
			wasm,
			bytes: loader.uncompressed_bytes().to_vec(),
			runtime_metadata_prefixed,
			metadata,
			metadata_version,
			core_version,
			compression: loader.compression(),
		})
	}

	/// Prints magic and version from a raw buffer.
	/// This is mainly used for troubleshooting when decoding
	/// a wasm fails.
	pub fn print_magic_and_version(data: &[u8]) {
		let is_substrate_wasm = WasmTestBed::is_substrate_wasm(&data.to_vec());
		let version = WasmTestBed::get_metadata_version(data);

		println!("✨ Magic number found: {}", if is_substrate_wasm { "YES" } else { "NO" });
		println!("#️⃣ Extracted version : V{:?}", version);
	}

	pub fn compression(&self) -> Compression {
		self.compression
	}

	#[allow(clippy::ptr_arg)]
	pub fn is_substrate_wasm(metadata: &WasmBytes) -> bool {
		[metadata[0], metadata[1], metadata[2], metadata[3]] == META
	}

	pub fn reserved_meta(&self) -> ReservedMeta {
		[self.metadata[0], self.metadata[1], self.metadata[2], self.metadata[3]]
	}

	pub fn reserved_meta_valid(&self) -> bool {
		self.reserved_meta() == META
	}

	pub fn wasm(&self) -> &WasmBytes {
		&self.wasm
	}

	pub fn raw_bytes(&self) -> &WasmBytes {
		&self.bytes
	}

	pub fn get_metadata_version(data: &[u8]) -> u8 {
		data[4]
	}

	/// Call a function in the provided wasm. Note that we can only call a few limited set of functions
	/// as we have no blocks, storage, etc...
	fn call(wasm: &[u8], method: &str, call_data: &[u8]) -> Result<Vec<u8>> {
		let mut ext = sp_state_machine::BasicExternalities::default();
		let mut host_functions = sp_io::SubstrateHostFunctions::host_functions();
		host_functions.push(&logger_mock::LoggerMock);

		// host_functions
		// 	.clone()
		// 	.into_iter()
		// 	.for_each(|host_fn| debug!("{:?}", host_fn.name()));

		let executor = WasmExecutor::new(
			WasmExecutionMethod::Interpreted,
			// At least 12 for Polkadot V12/V13.
			// Substrate V14 requires 34.
			// Polkadot V14 requires 20.
			Some(64),
			host_functions,
			8,
			None,
		);

		executor
			.call_in_wasm(wasm, None, method, call_data, &mut ext, sp_core::traits::MissingHostFunctions::Allow)
			.map_err(|_| WasmTestbedError::Calling(method.to_string()))
	}

	pub fn get_core_version(wasm: &[u8]) -> Option<RuntimeVersion> {
		let encoded = Self::call(wasm, "Core_version", &[]).unwrap();
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

	/// Get the size of the runtime
	pub fn size(&self) -> usize {
		self.bytes.len()
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
	pub fn core_version(&self) -> Option<&RuntimeVersion> {
		self.core_version.as_ref()
	}

	/// Compute the proposal hash of the runtime
	pub fn proposal_hash(&self) -> String {
		let result: SrhResult = get_result(substrate_runtime_proposal_hash::PREFIX_SYSTEM_SETCODE, &self.bytes);
		format!("0x{}", &result.encodedd_hash)
	}

	/// Compute the proposal hash of the runtime
	pub fn parachain_authorize_upgrade_hash(&self) -> String {
		let result = get_parachainsystem_authorize_upgrade(&self.bytes);
		format!("0x{}", hex::encode(result))
	}

	/// Compute the blake2-256 hash of the runtime
	pub fn blake2_256_hash(&self) -> String {
		let result = BlakeTwo256::hash(&self.bytes);
		format!("{:?}", result)
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
	const WESTEND_V30_V12: &str = "../../data/westend/westend_runtime-v900-rc2.compact.wasm";
	const POLKADOT_DEV: &str = "../../data/v900/polkadot-dev-v900-rc1.wasm";
	const RUNTIME_V12: &str = "../../data/runtime_v12.wasm";
	const RUNTIME_V13: &str = "../../data/runtime_v13.wasm";
	const RUNTIME_V14: &str = "../../data/runtime_v14.wasm";

	#[cfg(test)]
	mod common {
		use super::*;
		#[test]
		#[ignore = "local data"]
		fn it_panics_on_non_substrate_wasm() {
			assert!(WasmTestBed::new(&Source::File(PathBuf::from(WASM_NO_SUBSTRATE))).is_err());
		}
	}

	#[cfg(test)]
	mod runtime_versions {
		use super::*;

		#[test]
		#[ignore = "local data"]
		fn it_loads_v12() {
			let runtime = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_V12))).unwrap();
			println!("{:#?}", runtime);
			assert!(runtime.metadata_version == 12);
			assert!(runtime.core_version.is_some());
			assert!(runtime.is_supported());
		}

		#[test]
		#[ignore = "local data"]
		fn it_loads_v13() {
			let runtime = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_V13))).unwrap();
			println!("{:#?}", runtime);
			assert!(runtime.metadata_version == 13);
			assert!(runtime.core_version.is_some());
			assert!(runtime.is_supported());
		}

		#[test]
		#[ignore = "local data"]
		fn it_loads_v14() {
			let runtime = WasmTestBed::new(&Source::File(PathBuf::from(RUNTIME_V14)));
			// println!("runtime = {:?}", runtime);
			assert!(!runtime.is_err());
			println!("runtime = {:?}", &runtime);
			let runtime = runtime.unwrap();
			// println!("{:#?}", runtime);
			assert!(runtime.metadata_version == 14);
			// assert!(runtime.core_version.is_none());
			assert!(runtime.is_supported());
		}
	}

	#[cfg(test)]
	mod kusama {
		use super::*;

		#[test]
		#[should_panic]
		#[ignore = "local data"]
		fn it_loads_kusama_1050() {
			let runtime = WasmTestBed::new(&Source::File(PathBuf::from(KUSAMA_1050_VXX))).unwrap();
			println!("{:#?}", runtime);
			assert!(runtime.metadata_version == 11);
			assert!(runtime.core_version.is_none());
			assert!(runtime.is_supported());
		}

		#[test]
		#[should_panic]
		#[ignore = "local data"]
		fn it_loads_kusama_1062() {
			let runtime = WasmTestBed::new(&Source::File(PathBuf::from(KUSAMA_1062_VXX))).unwrap();
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
		#[ignore = "local data"]
		fn it_loads_kusama_metdata() {
			let runtime = WasmTestBed::new(&Source::File(PathBuf::from(KUSAMA_2030_VXX))).unwrap();
			println!("{:#?}", runtime);
			assert!(runtime.metadata_version == 12);
			assert!(runtime.is_supported());
		}

		#[test]
		#[ignore = "local data"]
		fn it_loads_kusama_2030() {
			let runtime = WasmTestBed::new(&Source::File(PathBuf::from(KUSAMA_2030_VXX))).unwrap();
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
		#[ignore = "local data"]
		fn it_loads_polkadot_01() {
			let runtime = WasmTestBed::new(&Source::File(PathBuf::from(POLKADOT_01_V11))).unwrap();
			println!("{:#?}", runtime);
			assert!(runtime.metadata_version == 11);
			assert!(runtime.is_supported());
		}

		#[test]
		#[ignore = "local data"]
		fn it_loads_polkadot_29() {
			let runtime = WasmTestBed::new(&Source::File(PathBuf::from(POLKADOT_29_V12))).unwrap();

			println!("{:#?}", runtime);

			assert!(runtime.metadata_version == 12);
			assert!(runtime.is_supported());
		}
	}

	#[cfg(test)]
	mod westend {
		use super::*;

		#[test]
		#[ignore = "local data"]
		fn it_loads_westend_30() {
			let runtime = WasmTestBed::new(&Source::File(PathBuf::from(WESTEND_V30_V12))).unwrap();
			println!("{:#?}", runtime);
			assert!(runtime.metadata_version == 12);
			assert!(runtime.is_supported());
		}
	}

	#[cfg(test)]
	mod polkadot_dev {
		use super::*;

		#[test]
		#[ignore = "local data"]
		fn it_loads_polkadot_dev() {
			let runtime = WasmTestBed::new(&Source::File(PathBuf::from(POLKADOT_DEV))).unwrap();
			println!("{:#?}", runtime);
			assert!(runtime.metadata_version == 12);
			assert!(runtime.is_supported());
		}
	}
}

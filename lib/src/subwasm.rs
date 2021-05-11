use calm_io::stdoutln;
use color_eyre::eyre;
use frame_metadata::{v12, RuntimeMetadata};
use wasm_loader::Source;
use wasm_testbed::{WasmTestBed, WasmTestbedError};

use crate::{print_magic_and_version, RuntimeInfo};
pub struct Subwasm {
	testbed: WasmTestBed,
	runtime_info: RuntimeInfo,
}

impl Subwasm {
	pub fn new(source: &Source) -> Self {
		let testbed = WasmTestBed::new(source)
			.map_err(|e| {
				eprintln!("{}", e);
				if let WasmTestbedError::Decoding(data) = e {
					print_magic_and_version(&data);
				}
				const REPO: &str = env!("CARGO_PKG_REPOSITORY");
				const NAME: &str = env!("CARGO_PKG_NAME");
				const VERSION: &str = env!("CARGO_PKG_VERSION");
				println!("🗣️ If you think it should have worked, please open an issue at {}/issues", REPO);
				println!("and attach your runtime and mention using {} v{}", NAME, VERSION);
				panic!("Could not load runtime");
			})
			.unwrap();

		let runtime_info = RuntimeInfo::new(&testbed);
		Self { testbed, runtime_info }
	}

	pub fn runtime_info(&self) -> &RuntimeInfo {
		&self.runtime_info
	}

	pub fn display_infos(&self) -> color_eyre::Result<()> {
		let metadata = self.testbed.runtime_metadata_prefixed();

		match &metadata.1 {
			RuntimeMetadata::V12(_v12) => {
				println!("Detected Substrate Runtime V12");
			}
			RuntimeMetadata::V13(_v13) => {
				println!("Detected Substrate Runtime V13");
			}
			_ => return Err(eyre::eyre!("Unsupported metadata version")),
		};
		Ok(())
	}

	pub fn print_modules_list(&self) -> color_eyre::Result<()> {
		let metadata = self.testbed.runtime_metadata_prefixed();

		match &metadata.1 {
			RuntimeMetadata::V12(v12) => {
				let modules = match &v12.modules {
					v12::DecodeDifferentArray::Decoded(modules) => modules,
					v12::DecodeDifferentArray::Encode(_) => return Err(eyre::eyre!("Metadata should be Decoded")),
				};

				modules.iter().for_each(|module| println!(" - {:02}: {:?}", module.index, module.name));
			}
			RuntimeMetadata::V13(_v13) => {
				// let _pallet = v13.modules.iter().inspect(|module| println!(" - {:?}{:?}", module.index, module.name));
				// .find(|m| &m.name == pallet)
				todo!("Not yet implemented");
			}
			_ => return Err(eyre::eyre!("Unsupported metadata version")),
		}
		Ok(())
	}

	/// Display all the metadata or a part of it for a given pallet
	pub fn display_metadata(&self) -> color_eyre::Result<()> {
		// let pallet_filter: Option<String> = Some("Identity".to_string());
		let pallet_filter: Option<String> = None;
		let metadata = self.testbed.runtime_metadata_prefixed();

		let serialized = if let Some(ref pallet) = pallet_filter {
			match &metadata.1 {
				RuntimeMetadata::V12(v12) => {
					let modules = match &v12.modules {
						v12::DecodeDifferentArray::Decoded(modules) => modules,
						v12::DecodeDifferentArray::Encode(_) => return Err(eyre::eyre!("Metadata should be Decoded")),
					};
					let pallet_metadata = modules
						.iter()
						.find(|module| module.name == v12::DecodeDifferent::Decoded(pallet.into()))
						.ok_or_else(|| eyre::eyre!("pallet not found in metadata"))?;
					serde_json::to_string_pretty(&pallet_metadata)?
				}
				RuntimeMetadata::V13(_v13) => {
					// let pallet = v13
					// 	.modules
					// 	.iter()
					// 	.find(|m| &m.name == pallet)
					// 	.ok_or_else(|| eyre::eyre!("pallet not found in metadata"))?;
					// serde_json::to_string_pretty(&pallet)?
					todo!("Not yet implemented");
				}
				_ => return Err(eyre::eyre!("Unsupported metadata version")),
			}
		} else {
			serde_json::to_string_pretty(&metadata)?
		};
		println!("{}", serialized);
		Ok(())
	}

	/// Display the metadata as json
	// TOOD: fix name
	pub fn display_raw_metadata(&self) -> color_eyre::Result<()> {
		let pallet_filter: Option<String> = None;
		let metadata = self.testbed.metadata();

		let serialized = if let Some(ref pallet) = pallet_filter {
			match metadata {
				RuntimeMetadata::V12(v12) => {
					let modules = match &v12.modules {
						v12::DecodeDifferentArray::Decoded(modules) => modules,
						v12::DecodeDifferentArray::Encode(_) => return Err(eyre::eyre!("Metadata should be Decoded")),
					};
					let pallet_metadata = modules
						.iter()
						.find(|module| module.name == v12::DecodeDifferent::Decoded(pallet.into()))
						.ok_or_else(|| eyre::eyre!("pallet not found in metadata"))?;
					serde_json::to_string_pretty(&pallet_metadata)?
				}
				// RuntimeMetadata::V13(v13) => {
				// 	let pallet = v13
				// 		.modules
				// 		.iter()
				// 		.find(|m| &m.name == pallet)
				// 		.ok_or_else(|| eyre::eyre!("pallet not found in metadata"))?;
				// 	serde_json::to_string_pretty(&pallet)?
				// }
				_ => return Err(eyre::eyre!("Unsupported metadata version")),
			}
		} else {
			serde_json::to_string_pretty(&metadata)?
		};

		// The following fails if piped to another command that truncates the output.
		// Typical use case here is: subwasm meta | head
		// The failure is due to https://github.com/rust-lang/rust/issues/46016
		// TODO: Once the above is fixed, we can remove the dependency on calm_io
		// println!("{}", serialized);

		match stdoutln!("{}", serialized) {
			Ok(_) => Ok(()),
			Err(e) => match e.kind() {
				std::io::ErrorKind::BrokenPipe => Ok(()),
				_ => Err(e),
			},
		}?;

		Ok(())
	}
}

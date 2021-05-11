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

	// pub fn print_runtime_infos(&self) {
	// 	let sizes = |x| -> (f32, usize) { (x as f32 / 1024.0 / 1024.0, x) };
	// 	// TODO: Fetch block number/hash so we know what we got when we called with block_ref = None

	// 	// RUNTIME SIZE
	// 	let size = self.testbed.size();

	// 	println!(
	// 		"🏋️  Runtime Size:\t{:.3?} MB ({} bytes)",
	// 		sizes(size).0,
	// 		sizes(size).1.to_formatted_string(&Locale::en)
	// 	);

	// 	// METADATA VERSION
	// 	let metadata_version = self.testbed.metadata_version();
	// 	println!("🎁 Metadata version:\tV{:?}", metadata_version);

	// 	// CORE VERSIONS
	// 	let version = self.testbed.core_version().as_ref().expect("Some version");
	// 	println!("🔥 Core version:\t{}", version);

	// 	println!("🗳️  Proposal hash:\t{}", self.testbed.proposal_hash());
	// }

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
}

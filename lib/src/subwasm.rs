use std::io::Write;

use crate::{
	metadata_wrapper::{self, MetadataWrapper},
	RuntimeInfo,
};

use wasm_loader::Source;
use wasm_testbed::{WasmTestBed, WasmTestbedError};

pub struct Subwasm {
	testbed: WasmTestBed,
	runtime_info: RuntimeInfo,
}

impl Subwasm {
	pub fn new(source: &Source) -> Self {
		let testbed = WasmTestBed::new(source)
			.map_err(|e| {
				eprintln!("{e}");
				if let WasmTestbedError::Decoding(data) = e {
					WasmTestBed::print_magic_and_version(&data);
				}
				const REPO: &str = env!("CARGO_PKG_REPOSITORY");
				const NAME: &str = env!("CARGO_PKG_NAME");
				const VERSION: &str = env!("CARGO_PKG_VERSION");
				println!("🗣️ If you think it should have worked, please open an issue at {REPO}/issues");
				println!("and attach your runtime and mention using {NAME} v{VERSION}");
				println!("The source was {source} ");

				panic!("Could not load runtime");
			})
			.unwrap();

		let runtime_info = RuntimeInfo::new(&testbed);
		Self { testbed, runtime_info }
	}

	pub fn runtime_info(&self) -> &RuntimeInfo {
		&self.runtime_info
	}

	// TODO: clean up
	// pub fn display_infos(&self) -> color_eyre::Result<()> {
	// 	let metadata = self.testbed.runtime_metadata_prefixed();

	// 	match &metadata.1 {
	// 		RuntimeMetadata::V12(_v12) => {
	// 			println!("Detected Substrate Runtime V12");
	// 		}
	// 		RuntimeMetadata::V13(_v13) => {
	// 			println!("Detected Substrate Runtime V13");
	// 		}
	// 		RuntimeMetadata::V14(_v14) => {
	// 			println!("Detected Substrate Runtime V14");
	// 		}
	// 		_ => return Err(eyre::eyre!("Unsupported metadata version")),
	// 	};
	// 	Ok(())
	// }

	pub fn write_metadata<O: Write>(
		&self,
		fmt: metadata_wrapper::OutputFormat,
		filter: Option<String>,
		out: &mut O,
	) -> color_eyre::Result<()> {
		let metadata = self.testbed.runtime_metadata_prefixed();
		let wrapper = MetadataWrapper(&metadata.1);
		wrapper.write(fmt, filter, out)
	}
}

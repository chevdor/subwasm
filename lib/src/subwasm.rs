use std::io::Write;

use crate::{
	metadata_wrapper::{self, MetadataWrapper},
	RuntimeInfo,
};

use substrate_differ::differs::reduced::reduced_runtime::ReducedRuntime;
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

	// pub fn display_module(&self, filter: String) {
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

	// pub fn display_modules_list(&self) {
	// 	let metadata = self.testbed.runtime_metadata_prefixed();
	// 	let wrapper = MetadataWrapper(&metadata.1);
	// 	wrapper.display_modules_list();
	// }

	pub fn display_reduced_runtime(&self, json: bool) {
		let reduced_runtime: ReducedRuntime = self.testbed.metadata().into();
		if json {
			println!("{}", serde_json::to_string_pretty(&reduced_runtime).expect("Failed encoding reduced runtime"));
		} else {
			println!("{reduced_runtime}");
		}
	}

	pub fn display_reduced_pallet(&self, pallet: &str, json: bool) {
		let reduced_runtime: ReducedRuntime = self.testbed.metadata().into();
		let pallet_maybe = reduced_runtime.get_pallet_by_name(pallet);

		if let Some(reduced_pallet) = pallet_maybe {
			if json {
				println!("{}", serde_json::to_string_pretty(&reduced_pallet).expect("Failed encoding reduced runtime"));
			} else {
				println!("{reduced_pallet}");
			}
		} else {
			println!("Pallet '{pallet}' not found.");
		}
	}

	// /// Display the metadata as json
	// pub fn display_metadata_json(&self) {
	// 	let pallet_filter: Option<String> = None;
	// 	let metadata = self.testbed.metadata();

	// 	let serialized = if let Some(ref pallet) = pallet_filter {
	// 		match metadata {
	// 			RuntimeMetadata::V12(v12) => {
	// 				let modules = convert(&v12.modules);

	// 				let pallet_metadata = modules
	// 					.iter()
	// 					.find(|module| module.name == DecodeDifferent::Decoded(pallet.into()))
	// 					.expect("pallet not found in metadata");
	// 				serde_json::to_string_pretty(&pallet_metadata)
	// 			}
	// 			// RuntimeMetadata::V13(v13) => {
	// 			// 	let pallet = v13
	// 			// 		.modules
	// 			// 		.iter()
	// 			// 		.find(|m| &m.name == pallet)
	// 			// 		.ok_or_else(|| eyre::eyre!("pallet not found in metadata"))?;
	// 			// 	serde_json::to_string_pretty(&pallet)?
	// 			// }
	// 			_ => panic!("Unsupported metadata version"),
	// 		}
	// 	} else {
	// 		serde_json::to_string_pretty(&metadata)
	// 	};

	// 	// The following fails if piped to another command that truncates the output.
	// 	// Typical use case here is: subwasm meta | head
	// 	// The failure is due to https://github.com/rust-lang/rust/issues/46016
	// 	// TODO: Once the above is fixed, we can remove the dependency on calm_io
	// 	// println!("{}", serialized);

	// 	let serialized = serialized.unwrap();
	// 	let _ = match stdoutln!("{}", serialized) {
	// 		Ok(_) => Ok(()),
	// 		Err(e) => match e.kind() {
	// 			std::io::ErrorKind::BrokenPipe => Ok(()),
	// 			_ => Err(e),
	// 		},
	// 	};
	// 	wrapper.write(fmt, filter, out)
	// }
}

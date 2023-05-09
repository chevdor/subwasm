use std::io::Write;

use crate::{
	metadata_wrapper::{self, MetadataWrapper},
	utils::print_big_output_safe,
	RuntimeInfo,
};

use anyhow::bail;
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

	pub fn display_reduced_runtime(&self, json: bool) -> anyhow::Result<()> {
		let reduced_runtime: ReducedRuntime = self.testbed.metadata().into();

		if json {
			let serialized = serde_json::to_string_pretty(&reduced_runtime)?;
			print_big_output_safe(&serialized)
		} else {
			print_big_output_safe(&reduced_runtime.to_string())
		}
	}

	pub fn display_reduced_pallet(&self, pallet: &str, json: bool) -> anyhow::Result<()> {
		let reduced_runtime: ReducedRuntime = self.testbed.metadata().into();
		let pallet_maybe = reduced_runtime.get_pallet_by_name(pallet);

		if let Some(reduced_pallet) = pallet_maybe {
			if json {
				let serialized = serde_json::to_string_pretty(&reduced_pallet)?;

				print_big_output_safe(&serialized)
			} else {
				print_big_output_safe(&reduced_pallet.to_string())
			}
		} else {
			bail!("Pallet '{pallet}' not found.")
		}
	}
}

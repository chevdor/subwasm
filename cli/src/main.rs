#![feature(external_doc)]
#![doc(include = "../README.md")]
mod opts;

use clap::Clap;
use opts::*;
use std::path::PathBuf;
use subwasmlib::*;
use wasm_loader::*;
use wasm_testbed::*;

/// Main entry point of the `subwasm` cli.
fn main() -> color_eyre::Result<()> {
	let opts: Opts = Opts::parse();

	match opts.subcmd {
		SubCommand::Get(get_opts) => {
			println!("Getting runtime from a node at {}", get_opts.url);
			download_runtime(&get_opts.url, get_opts.block, get_opts.output)?;
		}

		SubCommand::Info(info_opts) => {
			let src = Source::File(PathBuf::from(&info_opts.input));
			if let Ok(runtime) = WasmTestBed::new(src) {
				match info_opts.details_level {
					0 => {
						display_infos(runtime.runtime_metadata_prefixed())?;
						println!(
							"Version {:?} {} supported.",
							runtime.metadata_version(),
							if runtime.is_supported() { "is" } else { "is NOT" }
						);
						println!("Proposal hash: {}", runtime.proposal_hash());
					}
					_ => {
						display_modules_list(runtime.runtime_metadata_prefixed())?;
					}
				}
			} else {
				panic!("That does not look like a Substrate runtime, or least not a recent one we can decode.")
			}
		}

		SubCommand::Metadata(meta_opts) => {
			let src = Source::File(PathBuf::from(&meta_opts.input));
			let runtime = WasmTestBed::new(src).expect("Loading runtime to testbed");
			display_raw_metadata(runtime.metadata())?;
		}
	};

	Ok(())
}

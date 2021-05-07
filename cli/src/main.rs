mod opts;

use clap::{crate_name, crate_version, Clap};
use opts::*;
use subwasmlib::*;
use wasm_testbed::*;

macro_rules! noquiet {
	( $q:expr, $x:expr ) => {{
		if !$q {
			$x
		}
	}};
}

/// Main entry point of the `subwasm` cli.
fn main() -> color_eyre::Result<()> {
	let opts: Opts = Opts::parse();
	noquiet!(opts.quiet, println!("Running {} v{}", crate_name!(), crate_version!()));

	match opts.subcmd {
		SubCommand::Get(get_opts) => {
			let url = &get_url(get_opts.chain.as_deref(), &get_opts.url);
			println!("Getting runtime from {:?}", url);
			download_runtime(url, get_opts.block, get_opts.output)?;
		}

		SubCommand::Info(info_opts) => {
			let src = get_source(info_opts.chain.as_deref(), info_opts.source);
			println!("Loading from {}", src);

			let runtime = WasmTestBed::new(&src)
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

			match info_opts.details_level {
				0 => {
					// println!(
					// 	"Version {:?} {} supported.",
					// 	runtime.metadata_version(),
					// 	if runtime.is_supported() { "is" } else { "is NOT" }
					// );
					// display_infos(runtime.runtime_metadata_prefixed())?;
					print_runtime_infos(src);
				}
				_ => {
					display_modules_list(runtime.runtime_metadata_prefixed())?;
				}
			}
		}

		SubCommand::Metadata(meta_opts) => {
			let runtime = WasmTestBed::new(&meta_opts.source).expect("Loading runtime to testbed");
			display_raw_metadata(runtime.metadata())?;
		}

		SubCommand::Diff(diff_opts) => {
			diff(diff_opts.a, diff_opts.b);
		}
	};

	Ok(())
}

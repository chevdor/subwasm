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

	match opts.subcmd {
		SubCommand::Get(get_opts) => {
			noquiet!(opts.quiet, println!("Running {} v{}", crate_name!(), crate_version!()));
			let chain_name = get_opts.chain.map(|some| some.name);
			let url = &get_url(chain_name.as_deref(), &get_opts.url);
			download_runtime(url, get_opts.block, get_opts.output)?;
		}

		SubCommand::Info(info_opts) => {
			noquiet!(opts.quiet, println!("Running {} v{}", crate_name!(), crate_version!()));
			let chain_name = info_opts.chain.map(|some| some.name);
			let source = get_source(chain_name.as_deref(), info_opts.source);

			let subwasm = Subwasm::new(&source);
			// let infos = subwasm.get_infos(); // good for json...
			// subwasm.print_infos();

			match info_opts.details_level {
				0 => {
					// println!(
					// 	"Version {:?} {} supported.",
					// 	runtime.metadata_version(),
					// 	if runtime.is_supported() { "is" } else { "is NOT" }
					// );
					// display_infos(runtime.runtime_metadata_prefixed())?;
					subwasm.print_runtime_infos();
				}
				_ => {
					subwasm.display_modules_list()?;
				}
			}
		}

		SubCommand::Metadata(meta_opts) => {
			let runtime = WasmTestBed::new(&meta_opts.source).expect("Loading runtime to testbed");
			display_raw_metadata(runtime.metadata())?;
		}

		SubCommand::Diff(diff_opts) => {
			noquiet!(opts.quiet, println!("Running {} v{}", crate_name!(), crate_version!()));

			diff(diff_opts.a, diff_opts.b);
		}
	};

	Ok(())
}

#[cfg(test)]
mod test {
	use assert_cmd::Command;
	use std::path::Path;

	#[test]
	fn it_shows_help() {
		let mut cmd = Command::cargo_bin("subwasm").unwrap();
		let assert = cmd.arg("--help").assert();
		assert.success().code(0);
	}

	#[test]
	fn it_fails_without_source() {
		let mut cmd = Command::cargo_bin("subwasm").unwrap();
		let assert = cmd.arg("info tcp://foo.bar").assert();
		assert.failure().code(2);
	}

	#[test]
	fn it_gets_a_runtime() {
		let mut cmd = Command::cargo_bin("subwasm").unwrap();

		let assert = cmd.args(&["get", "--output", "/tmp/runtime.wasm", "wss://rpc.polkadot.io"]).assert();
		assert.success().code(0);
		assert!(Path::new("/tmp/runtime.wasm").exists());
	}

	#[test]
	fn it_fails_on_bad_chain() {
		let mut cmd = Command::cargo_bin("subwasm").unwrap();

		let assert = cmd.args(&["get", "--chain", "foobar"]).assert();
		assert.failure().code(101);
	}
}

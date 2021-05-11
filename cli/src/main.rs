mod opts;

use clap::{crate_name, crate_version, Clap};
use opts::*;
use subwasmlib::*;

/// Simple macro that only execute $statement if $opts don#t contain neither the quiet nor the json flag
macro_rules! noquiet {
	( $opts:ident, $statement:expr ) => {{
		if !$opts.quiet && !$opts.json {
			$statement
		}
	}};
}

/// Main entry point of the `subwasm` cli
fn main() -> color_eyre::Result<()> {
	let opts: Opts = Opts::parse();

	match opts.subcmd {
		SubCommand::Get(get_opts) => {
			noquiet!(opts, println!("Running {} v{}", crate_name!(), crate_version!()));
			let chain_name = get_opts.chain.map(|some| some.name);
			let url = &get_url(chain_name.as_deref(), &get_opts.url);

			download_runtime(url, get_opts.block, get_opts.output)?;
		}

		SubCommand::Info(info_opts) => {
			noquiet!(opts, println!("Running {} v{}", crate_name!(), crate_version!()));

			let chain_name = info_opts.chain.map(|some| some.name);
			let source = get_source(chain_name.as_deref(), info_opts.source);

			noquiet!(opts, println!("⏱️  Loading WASM from {:?}", &source));
			let subwasm = Subwasm::new(&source);

			subwasm.runtime_info().print(opts.json);
		}

		SubCommand::Metadata(meta_opts) => {
			noquiet!(opts, println!("Running {} v{}", crate_name!(), crate_version!()));

			let chain_name = meta_opts.chain.map(|some| some.name);
			let source = get_source(chain_name.as_deref(), meta_opts.source);

			noquiet!(opts, println!("⏱️  Loading WASM from {:?}", &source));
			let subwasm = Subwasm::new(&source);

			match opts.json {
				true => subwasm.display_raw_metadata()?,
				false => subwasm.print_modules_list()?, // false => subwasm.display_metadata()?,
			}
		}

		SubCommand::Diff(diff_opts) => {
			noquiet!(opts, println!("Running {} v{}", crate_name!(), crate_version!()));

			let chain_a = diff_opts.chain_a.map(|some| some.name);
			let src_a = get_source(chain_a.as_deref(), diff_opts.src_a);

			let chain_b = diff_opts.chain_b.map(|some| some.name);
			let src_b = get_source(chain_b.as_deref(), diff_opts.src_b);

			diff(src_a, src_b);
		}
	};

	Ok(())
}

#[cfg(test)]
mod test {
	use assert_cmd::Command;
	use std::path::Path;

	#[test]
	#[ignore = "assert_cmd bug, see https://github.com/assert-rs/assert_cmd/issues/117"]
	fn it_shows_help() {
		let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
		let assert = cmd.arg("--help").assert();
		assert.success().code(0);
	}

	#[test]
	#[ignore = "assert_cmd bug, see https://github.com/assert-rs/assert_cmd/issues/117"]
	fn it_fails_without_source() {
		let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
		let assert = cmd.arg("info tcp://foo.bar").assert();
		assert.failure().code(2);
	}

	#[test]
	#[ignore = "assert_cmd bug, see https://github.com/assert-rs/assert_cmd/issues/117"]
	fn it_gets_a_runtime() {
		let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

		let assert = cmd.args(&["get", "--output", "/tmp/runtime.wasm", "wss://rpc.polkadot.io"]).assert();
		assert.success().code(0);
		assert!(Path::new("/tmp/runtime.wasm").exists());
	}

	#[test]
	#[ignore = "assert_cmd bug, see https://github.com/assert-rs/assert_cmd/issues/117"]
	fn it_fails_on_bad_chain() {
		let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

		let assert = cmd.args(&["get", "--chain", "foobar"]).assert();
		assert.failure().code(101);
	}
}

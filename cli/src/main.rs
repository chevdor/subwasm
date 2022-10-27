mod opts;

use clap::{crate_authors, crate_name, crate_version, Parser};
use env_logger::Env;
use log::info;
use opts::*;
use subwasmlib::*;
use text_style::{AnsiColor, StyledStr};

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
	env_logger::Builder::from_env(Env::default().default_filter_or("none")).init();
	let opts: Opts = Opts::parse();
	noquiet!(opts, println!("Running {} v{}", crate_name!(), crate_version!()));
	noquiet!(opts, println!("{}", crate_authors!(", ")));

	match opts.subcmd {
		SubCommand::Get(get_opts) => {
			let chain_name = get_opts.chain.map(|some| some.name);
			let url = &get_url(chain_name.as_deref(), &get_opts.url);

			download_runtime(url, get_opts.block, get_opts.output)?;
		}

		SubCommand::Info(info_opts) => {
			let chain_name = info_opts.chain.map(|some| some.name);
			let source = get_source(chain_name.as_deref(), info_opts.source, info_opts.block);

			info!("⏱️  Loading WASM from {:?}", &source);
			let subwasm = Subwasm::new(&source);

			subwasm.runtime_info().print(opts.json);
		}

		SubCommand::Version(version_opts) => {
			let chain_name = version_opts.chain.map(|some| some.name);
			let source = get_source(chain_name.as_deref(), version_opts.source, version_opts.block);

			info!("⏱️  Loading WASM from {:?}", &source);
			let subwasm = Subwasm::new(&source);

			subwasm.runtime_info().print_version(opts.json);
		}

		SubCommand::Metadata(meta_opts) => {
			let chain_name = meta_opts.chain.map(|some| some.name);
			let source = get_source(chain_name.as_deref(), meta_opts.source, meta_opts.block);

			info!("⏱️  Loading WASM from {:?}", &source);
			let subwasm = Subwasm::new(&source);

			subwasm.display_metadata_json()
		}

		SubCommand::Diff(diff_opts) => {
			log::debug!("Method: {:?}", diff_opts.method);

			let chain_a = diff_opts.chain_a.map(|some| some.name);
			let src_a = get_source(chain_a.as_deref(), diff_opts.src_a, None);

			let chain_b = diff_opts.chain_b.map(|some| some.name);
			let src_b = get_source(chain_b.as_deref(), diff_opts.src_b, None);

			match diff_opts.method {
				DiffMethod::Reduced => {
					let diff_result = reduced_diff(src_a, src_b);
					if opts.json {
						let s = serde_json::to_string_pretty(&diff_result).unwrap();
						println!("{s}");
					} else {
						let warning = StyledStr::plain(
							"!!! THE SUBWASM REDUCED DIFFER IS EXPERIMENTAL, DOUBLE CHECK THE RESULTS !!!\n",
						);

						let warning = if opts.no_color {
							warning
						} else {
							warning.on(AnsiColor::Yellow.light()).with(AnsiColor::Red.light()).bold()
						};

						text_style::termion::render(std::io::stdout(), &warning).expect("Could not render line");
						println!("{diff_result}");
						text_style::termion::render(std::io::stdout(), &warning).expect("Could not render line");
					}
				}
				DiffMethod::Summary => todo!(),
			}
		}

		SubCommand::Compress(copts) => {
			compress(copts.input, copts.output).unwrap();
		}

		SubCommand::Decompress(dopts) => {
			decompress(dopts.input, dopts.output).unwrap();
		}

		SubCommand::ShowReduced(sr_opts) => {
			let chain_name = sr_opts.chain.map(|some| some.name);
			let source = get_source(chain_name.as_deref(), sr_opts.src, sr_opts.block);
			info!("⏱️  Loading WASM from {:?}", &source);
			let subwasm = Subwasm::new(&source);

			subwasm.display_reduced_runtime()
		}
	};

	Ok(())
}

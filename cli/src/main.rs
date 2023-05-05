mod opts;

use std::{env, io::Write};

use clap::{crate_name, crate_version, Parser};
use env_logger::Env;
use log::info;
use opts::*;
use serde_json::json;
use subwasmlib::*;
use text_style::{AnsiColor, StyledStr};

/// Main entry point of the `subwasm` cli
fn main() -> color_eyre::Result<()> {
	env_logger::Builder::from_env(Env::default().default_filter_or("none")).init();
	let opts: Opts = Opts::parse();

	color_eyre::install()?;

	match opts.subcmd {
		Some(SubCommand::Get(get_opts)) => {
			let chain_name = get_opts.chain.map(|some| some.name);
			let url = &get_url(chain_name.as_deref(), &get_opts.url);

			download_runtime(url, get_opts.block, get_opts.output)?;
		}

		Some(SubCommand::Info(info_opts)) => {
			let chain_name = info_opts.chain.map(|some| some.name);
			let source = get_source(chain_name.as_deref(), info_opts.source, info_opts.block);

			info!("⏱️  Loading WASM from {:?}", &source);
			let subwasm = Subwasm::new(&source);

			subwasm.runtime_info().print(opts.json);
		}

		Some(SubCommand::Version(version_opts)) => {
			let chain_name = version_opts.chain.map(|some| some.name);
			let source = get_source(chain_name.as_deref(), version_opts.source, version_opts.block);

			info!("⏱️  Loading WASM from {:?}", &source);
			let subwasm = Subwasm::new(&source);

			subwasm.runtime_info().print_version(opts.json);
		}

		Some(SubCommand::Metadata(meta_opts)) => {
			let chain_name = meta_opts.chain.map(|some| some.name);
			let source = get_source(chain_name.as_deref(), meta_opts.source, meta_opts.block);

			info!("⏱️  Loading WASM from {:?}", &source);
			let subwasm = Subwasm::new(&source);

			let mut fmt: OutputFormat = meta_opts.format.unwrap_or_else(|| "human".into()).into();
			if opts.json {
				eprintln!("--json is DEPRECATED, use --format=json instead");
				fmt = OutputFormat::Json;
			}

			let mut output = meta_opts.output;
			if let Some(out) = &output {
				if out.is_empty() || out == "auto" {
					match fmt {
						OutputFormat::Human => output = Some("metadata.txt".into()),
						OutputFormat::Json => output = Some("metadata.json".into()),
						OutputFormat::Scale => output = Some("metadata.scale".into()),
						OutputFormat::HexScale => output = Some("metadata.hex".into()),
						OutputFormat::JsonScale => output = Some("metadata.jscale".into()),
					}
				}
			}

			let mut out: Box<dyn Write> = if let Some(output) = &output {
				Box::new(std::fs::File::create(output)?)
			} else {
				Box::new(std::io::stdout())
			};

			match subwasm.write_metadata(fmt, meta_opts.module, &mut out) {
				Ok(_) => Ok(()),
				Err(e) => {
					if let Some(e) = e.root_cause().downcast_ref::<std::io::Error>() {
						if e.kind() == std::io::ErrorKind::BrokenPipe {
							log::debug!("ignoring broken pipe error: {:?}", e);
							return Ok(());
						}
					}
					Err(e)
				}
			}?
		}

		Some(SubCommand::Diff(diff_opts)) => {
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

		Some(SubCommand::Compress(copts)) => {
			compress(copts.input, copts.output)?;
		}

		Some(SubCommand::Decompress(dopts)) => {
			decompress(dopts.input, dopts.output)?;
		}

		None => {
			if opts.version {
				let name = crate_name!();
				let version = crate_version!();
				let commit_hash = env::var("SUBWASM_CLI_GIT_COMMIT_HASH");
				let build_date = env::var("SUBWASM_CLI_BUILD_DATE");

				if !opts.json {
					let commit_hash_str = if let Ok(s) = commit_hash { format!("-{s}") } else { String::from("") };
					let build_date_str = if let Ok(s) = build_date { format!(" built {s}") } else { String::from("") };
					println!("{name} v{version}{commit_hash_str}{build_date_str}");
				} else {
					let version_data = json!({
						"name": name,
						"version": version,
						"commit": commit_hash.unwrap_or_default(),
						"build_date": build_date.unwrap_or_default(),
					});
					let s = serde_json::to_string_pretty(&version_data).unwrap();
					println!("{s}");
				}
			}
		}

		Some(SubCommand::ShowReduced(sr_opts)) => {
			let chain_name = sr_opts.chain.map(|some| some.name);
			let source = get_source(chain_name.as_deref(), sr_opts.src, sr_opts.block);
			info!("⏱️  Loading WASM from {:?}", &source);
			let subwasm = Subwasm::new(&source);

			if let Some(pallet) = sr_opts.pallet {
				subwasm.display_reduced_pallet(&pallet, opts.json)
			} else {
				subwasm.display_reduced_runtime(opts.json)
			}
		}
	};

	Ok(())
}

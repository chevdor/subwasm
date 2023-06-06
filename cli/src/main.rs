mod error;
mod opts;

use clap::{crate_name, crate_version, Parser};
use env_logger::Env;
use log::*;
use opts::*;
use serde_json::json;
use std::{env, io::Write, path::PathBuf};
use subwasmlib::{source::Source, *};
use text_style::{AnsiColor, StyledStr};
use url::Url;
use wasm_loader::BlockRef;

/// Main entry point of the `subwasm` cli
fn main() -> color_eyre::Result<()> {
	env_logger::Builder::from_env(Env::default().default_filter_or("none")).init();
	color_eyre::install()?;

	let opts: Opts = Opts::parse();

	match opts.subcmd {
		Some(SubCommand::Get(get_opts)) => {
			let gh_url = get_github_url(get_opts.github)?;
			let download_url = select_url(gh_url, get_opts.url);

			match (download_url, get_opts.rpc_url) {
				(None, Some(rpc_url)) => {
					let chain_name = get_opts.chain.map(|some| some.name);
					let url = &get_url(chain_name.as_deref(), &rpc_url.into());
					Ok(download_runtime(url, get_opts.block, get_opts.output)?)
				}
				(Some(url), None) | (Some(url), Some(_)) => {
					let target = get_output_file(get_opts.output);
					let output = fetch_at_url(url, Some(target))?;
					debug!("Fetched at {output:?}");
					if output.exists() {
						info!("Got runtime at {output:?}");
						Ok(())
					} else {
						panic!("Failed fetching file")
					}
				}
				_ => {
					unreachable!()
				}
			}
		}

		Some(SubCommand::Info(info_opts)) => {
			let gh_url = get_github_url(info_opts.github)?;
			let download_url = select_url(gh_url, info_opts.url);
			let source = get_source(info_opts.file, info_opts.chain, info_opts.block, download_url)?;

			info!("⏱️  Loading WASM from {:?}", &source);
			let subwasm = Subwasm::new(&source.try_into()?)?;

			Ok(subwasm.runtime_info().print(opts.json)?)
		}

		Some(SubCommand::Version(info_opts)) => {
			let gh_url = get_github_url(info_opts.github)?;
			let download_url = select_url(gh_url, info_opts.url);
			let source = get_source(info_opts.file, info_opts.chain, info_opts.block, download_url)?;

			info!("⏱️  Loading WASM from {:?}", &source);
			let subwasm: Subwasm = Subwasm::new(&source.try_into()?)?;

			Ok(subwasm.runtime_info().print_version(opts.json)?)
		}

		Some(SubCommand::Metadata(meta_opts)) => {
			// let chain_name = meta_opts.chain.map(|some| some.name);
			// let source = get_source(chain_name.as_deref(), meta_opts.source, meta_opts.block)?;
			let source = meta_opts.source.try_into()?;
			info!("⏱️  Loading WASM from {:?}", &source);
			let subwasm = Subwasm::new(&source)?;

			let mut fmt: OutputFormat = meta_opts.format.unwrap_or_else(|| "human".into()).into();
			if opts.json {
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

			subwasm.write_metadata(fmt, meta_opts.module, &mut out)?;

			// TODO: Remove that when deprecating the --json flag but beware, --json is a global flag used by other commands...
			if opts.json {
				eprintln!("The --json flag is DEPRECATED for the metadata command, use --format=json instead");
			}

			Ok(())
		}

		Some(SubCommand::Diff(diff_opts)) => {
			log::debug!("Method: {:?}", diff_opts.method);

			// // let chain_a = diff_opts.chain_a.map(|some| some.name);
			// let src_a = Source::from_str(diff_opts.src_a)?;
			let src_a = diff_opts.src_a;
			let src_b = diff_opts.src_b;

			// // let chain_b = diff_opts.chain_b.map(|some| some.name);
			// let src_b = get_source(chain_b.as_deref(), diff_opts.src_b, None)?;

			match diff_opts.method {
				DiffMethod::Reduced => {
					let diff_result = reduced_diff(src_a.try_into()?, src_b.try_into()?).expect("Reduced diff failed");
					if opts.json {
						let s = serde_json::to_string_pretty(&diff_result).expect("serde_json ran into issues");
						println!("{s}");
						Ok(())
					} else {
						let warning = StyledStr::plain(
							"!!! THE SUBWASM REDUCED DIFFER IS EXPERIMENTAL, DOUBLE CHECK THE RESULTS !!!\n",
						);

						let warning = if opts.no_color {
							warning
						} else {
							warning.on(AnsiColor::Yellow.light()).with(AnsiColor::Red.light()).bold()
						};

						text_style::crossterm::render(std::io::stdout(), &warning).expect("Could not render line");
						println!("{diff_result}");
						text_style::crossterm::render(std::io::stdout(), &warning).expect("Could not render line");

						Ok(())
					}
				}
			}
		}

		Some(SubCommand::Compress(copts)) => Ok(compress(copts.input, copts.output)?),

		Some(SubCommand::Decompress(dopts)) => Ok(decompress(dopts.input, dopts.output)?),

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
					let s = serde_json::to_string_pretty(&version_data).expect("serde_json ran into issues");
					println!("{s}");
				}
				Ok(())
			} else {
				unreachable!("We show help if there is no arg");
			}
		}

		Some(SubCommand::Show(show_opts)) => {
			// let chain_name = show_opts.chain.map(|some| some.name);
			// let source = get_source(chain_name.as_deref(), show_opts.src, show_opts.block)?;
			let gh_url = get_github_url(show_opts.github)?;
			let download_url = select_url(gh_url, show_opts.url);
			let source = get_source(show_opts.file, show_opts.chain, show_opts.block, download_url)?;

			info!("⏱️  Loading WASM from {:?}", &source);
			let subwasm: Subwasm = Subwasm::new(&source.try_into()?)?;

			if show_opts.summary {
				Ok(subwasm.display_reduced_summary(opts.json)?)
			} else {
				if let Some(pallet) = show_opts.pallet {
					Ok(subwasm.display_reduced_pallet(&pallet, opts.json)?)
				} else {
					Ok(subwasm.display_reduced_runtime(opts.json)?)
				}
			}
		}
	}
}

/// Get the github artifacts url
pub fn get_github_url(s: Option<String>) -> Result<Option<Url>> {
	if let Some(g) = s {
		let (runtime, version) = gh_to_runtime_and_version(&g)?;
		Ok(Some(get_github_artifact_url(runtime, version)))
	} else {
		Ok(None)
	}
}

/// Depending on the options passed by the user we select and return the URL
pub fn select_url(gh_url: Option<Url>, dl_url: Option<Url>) -> Option<Url> {
	match (gh_url, dl_url) {
		(None, Some(u)) => Some(u),
		(Some(u), None) => Some(u),
		_ => None,
	}
}

/// Retrive one unique source from all the options the user may pass
pub fn get_source(
	file: Option<PathBuf>,
	chain: Option<ChainInfo>,
	block: Option<BlockRef>,
	dl_url: Option<Url>,
) -> Result<Source> {
	let source: Source = Source::from_options(file, chain, block, dl_url)?;
	// If the source is a URL, we try to fetch it first
	Ok(match source {
		Source::URL(u) => {
			debug!("Fetching runtime from {}", u);
			let runtime_file = fetch_at_url(u, None)?;
			debug!("Runtime fetched at {:?}", runtime_file.display());
			Source::File(runtime_file)
		}
		s => s,
	})
}

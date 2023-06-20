mod error;
mod opts;

use clap::{crate_name, crate_version, Parser};
use env_logger::Env;
use log::*;
use opts::*;
use serde_json::json;
use std::{env, io::Write, path::PathBuf, str::FromStr};
use subwasmlib::{source::Source, *};
use text_style::{AnsiColor, StyledStr};
use url::Url;
use wasm_loader::{BlockRef, NodeEndpoint, Source as WasmLoaderSource};

/// Main entry point of the `subwasm` cli
fn main() -> color_eyre::Result<()> {
	env_logger::Builder::from_env(Env::default().default_filter_or("none")).init();
	color_eyre::install()?;

	let opts: Opts = Opts::parse();
	// debug!("opts: {opts:#?}");

	match opts.subcmd {
		Some(SubCommand::Get(get_opts)) => {
			debug!("get_opts: {get_opts:#?}");
			let gh_url =
				if let Some(u) = get_opts.github { Some(GithubRef::from_str(u.as_str())?.as_url()) } else { None };
			debug!("gh_url: {gh_url:?}");

			let download_url = select_url(gh_url, get_opts.url);
			debug!("download_url: {download_url:?}");

			// Get the RPC url if the user passed a chain name, alias, or url
			let rpc_url = if let Some(o) = get_opts.rpc_url {
				Some(o.as_url()?)
			} else if let Some(c) = get_opts.chain {
				Some(c.get_random_url(None)?)
			} else {
				None
			};
			debug!("rpc_url: {rpc_url:?}");

			match (download_url, rpc_url) {
				(None, Some(rpc_url)) => {
					let _file =
						download_runtime(NodeEndpoint::from_str(rpc_url.as_str())?, get_opts.block, get_opts.output)?;
					Ok(())
				}
				(Some(url), _) => {
					let target = get_output_file_local(get_opts.output);
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
			let gh_url =
				if let Some(u) = info_opts.github { Some(GithubRef::from_str(u.as_str())?.as_url()) } else { None };
			let download_url = select_url(gh_url, info_opts.url);
			let source = get_source(info_opts.file, info_opts.chain, info_opts.block, download_url)?;

			info!("⏱️  Loading WASM from {:?}", &source);
			let subwasm = Subwasm::new(&source.try_into()?)?;

			Ok(subwasm.runtime_info().print(opts.json)?)
		}

		Some(SubCommand::Version(info_opts)) => {
			let gh_url =
				if let Some(u) = info_opts.github { Some(GithubRef::from_str(u.as_str())?.as_url()) } else { None };
			let download_url = select_url(gh_url, info_opts.url);
			let source = get_source(info_opts.file, info_opts.chain, info_opts.block, download_url)?;

			info!("⏱️  Loading WASM from {:?}", &source);
			let subwasm: Subwasm = Subwasm::new(&source.try_into()?)?;

			Ok(subwasm.runtime_info().print_version(opts.json)?)
		}

		Some(SubCommand::Metadata(meta_opts)) => {
			let gh_url =
				if let Some(u) = meta_opts.github { Some(GithubRef::from_str(u.as_str())?.as_url()) } else { None };
			let download_url = select_url(gh_url, meta_opts.url);
			let source = get_source(meta_opts.file, meta_opts.chain, meta_opts.block, download_url)?;

			info!("⏱️  Loading WASM from {:?}", &source);
			let subwasm: Subwasm = Subwasm::new(&source.try_into()?)?;

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
			// debug!("{:#?}", &diff_opts);

			let runtime_1 = diff_opts.runtime_1.as_file()?;
			debug!("Runtime 1: {}", runtime_1.display());

			let runtime_2 = diff_opts.runtime_2.as_file()?;
			debug!("Runtime 2: {}", runtime_2.display());

			let src_a = WasmLoaderSource::File(runtime_1);
			let src_b = WasmLoaderSource::File(runtime_2);

			let diff_result = reduced_diff(src_a, src_b).expect("Reduced diff failed");
			if opts.json {
				let s = serde_json::to_string_pretty(&diff_result).expect("serde_json ran into issues");
				println!("{s}");
				Ok(())
			} else {
				let warning =
					StyledStr::plain("!!! THE SUBWASM REDUCED DIFFER IS EXPERIMENTAL, DOUBLE CHECK THE RESULTS !!!\n");

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
			let gh_url =
				if let Some(u) = show_opts.github { Some(GithubRef::from_str(u.as_str())?.as_url()) } else { None };
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
) -> error::Result<Source> {
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

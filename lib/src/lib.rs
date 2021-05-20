use num_format::{Locale, ToFormattedString};
use std::path::Path;
use std::{fs::File, path::PathBuf};
use std::{io::prelude::*, str::FromStr};
use wasm_loader::{BlockRef, NodeEndpoint, OnchainBlock, Source};
use wasm_testbed::WasmTestBed;
mod convert;
mod metadata_wrapper;

mod chain_info;
mod error;
mod macros;
mod runtime_info;
mod subwasm;
mod types;
pub use chain_info::*;
use log::{debug, info};
pub use runtime_info::*;
pub use subwasm::*;
pub use types::*;

/// Prints magic and version from a raw buffer.
/// This is mainly used for troubleshooting when decoding
/// a wasm fails.
pub fn print_magic_and_version(data: &[u8]) {
	let is_substrate_wasm = WasmTestBed::is_substrate_wasm(&data.to_vec());
	let version = WasmTestBed::get_metadata_version(data);

	println!("✨ Magic number found: {}", if is_substrate_wasm { "YES" } else { "NO" });
	println!("#️⃣ Extracted version : V{:?}", version);
}

/// Returns Some node url if possible, None otherwise.
fn get_node_url(chain: Option<&str>) -> Option<String> {
	if let Some(chain) = chain {
		let chain_info = ChainInfo::from_str(chain).expect("Unknown chain");

		chain_info.get_random_url(None)
	} else {
		None
	}
}

/// Get the url of a node based on the user's input
/// If --chain NAME is passed and NAME is a supported chain
/// we return a random node from the known list for chain NAME.
/// If not, we fall back to the --url flag
pub fn get_url(chain: Option<&str>, reference: &OnchainBlock) -> String {
	let url = reference.endpoint.to_string();
	let node_url = get_node_url(chain);

	if let Some(chain_url) = node_url {
		chain_url
	} else {
		url
	}
}

/// Get the Source of some wasm based on the user's input
/// If --chain NAME is passed and NAME is a supported chain
/// we return a random node from the known list for chain NAME.
/// If not, we fall back to the `source`
pub fn get_source(chain: Option<&str>, source: Source) -> Source {
	let node_url = get_node_url(chain);

	if let Some(chain_url) = node_url {
		let endpoint = NodeEndpoint::from_str(&chain_url).unwrap();
		Source::Chain(OnchainBlock { endpoint, block_ref: None })
	} else {
		source
	}
}

/// Fetch the runtime from a node and store the wasm locally
pub fn download_runtime(url: &str, block_ref: Option<BlockRef>, output: Option<PathBuf>) -> color_eyre::Result<()> {
	let url = match url {
		url if url.starts_with("ws") => NodeEndpoint::WebSocket(url.to_string()),
		url if url.starts_with("http") => NodeEndpoint::Http(url.to_string()),
		_ => panic!("The url should either start with http or ws"),
	};

	let reference = OnchainBlock { endpoint: url, block_ref };
	// let wasm = wasm_loader::WasmLoader::fetch_wasm(&reference).expect("Getting wasm from the node");

	let loader =
		wasm_loader::WasmLoader::load_from_source(&Source::Chain(reference)).expect("Getting wasm from the node");
	let wasm = loader.bytes();

	info!("Got the runtime, its size is {:?}", wasm.len());

	let outfile = match output {
		Some(path) => path,

		// TODO: now that we have it, we could use runtime_<name>_<version>.wasm after querying the version
		_ => {
			let mut i = 0;
			let mut path;

			loop {
				path = format!("runtime_{:03?}.wasm", i);
				i += 1;
				assert!(i < 1000, "Ran out of indexes");
				if !Path::new(&path).exists() {
					break;
				}
			}
			PathBuf::from(path)
		}
	};

	info!("Saving runtime to {:?}", outfile);
	let mut buffer = File::create(outfile)?;
	buffer.write_all(&wasm)?;
	Ok(())
}

/// Compare 2 runtimes. It compares their versions first
/// then their metata.
pub fn diff(src_a: Source, src_b: Source) {
	let size = |x| -> (f32, usize) { (x as f32 / 1024.0 / 1024.0, x) };

	debug!("Loading WASM runtimes:");
	println!("  🅰️  {:?}", src_a);
	let runtime_a = WasmTestBed::new(&src_a).expect("Can only diff if the 2 runtimes can load");
	println!("  🅱️  {:?}", src_b);
	let runtime_b = WasmTestBed::new(&src_b).expect("Can only diff if the 2 runtimes can load");

	// RUNTIME SIZE
	let size_a = runtime_a.size();
	let size_b = runtime_b.size();

	info!("Checking runtime sizes:");
	if size_a == size_b {
		println!(
			"  ✅  Both size are identical: {:.3?} MB ({} bytes)",
			size(size_a).0,
			size(size_a).1.to_formatted_string(&Locale::en)
		);
	} else {
		println!("  🅰️  {:.3?} MB ({} bytes)", size(size_a).0, size(size_a).1.to_formatted_string(&Locale::en));
		println!("  🅱️  {:.3?} MB ({} bytes)", size(size_b).0, size(size_b).1.to_formatted_string(&Locale::en));
	}

	// METADATA VERSIONS
	let metadata_a_version = runtime_a.metadata_version();
	let metadata_b_version = runtime_b.metadata_version();
	println!("Checking metadata versions:");
	if metadata_a_version == metadata_b_version {
		println!("  ✅ Both metadata versions are identical: V{:?}", metadata_a_version);
	} else {
		println!("Found different metadata versions:");
		println!("  🅰️  V{:?}", metadata_a_version);
		println!("  🅱️  V{:?}", metadata_b_version);
	}

	// CORE VERSIONS
	println!("Checking core versions:");
	let version_a = runtime_a.core_version().expect("Some version");
	let version_b = runtime_b.core_version().expect("Some version");

	if version_a == version_b {
		print!("  ✅  The 2 core versions are identical: ");
		println!("{}", version_a);
	} else {
		println!("  ❌ The 2 core versions are different: ");
		// println!("{:#?}", version_a);
		println!("  🅰️  {}", version_a);
		// println!("{:#?}", version_b);
		println!("  🅱️  {}", version_b);
	}

	println!("Checking runtime metadata:");
	let metadata_a = runtime_a.metadata();
	let metadata_b = runtime_b.metadata();

	if metadata_a == metadata_b {
		// println!("  {}", version_a);
		println!("  ✅  The metadata are identical");
	} else {
		println!("  ❌  The metadata are different");
	}
}

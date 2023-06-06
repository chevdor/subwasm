#![allow(clippy::derive_partial_eq_without_eq)]

mod chain_info;
mod chain_urls;
mod convert;
mod error;
mod macros;
mod metadata_wrapper;
mod runtime_info;
pub mod source;
mod subwasm;
mod types;
mod utils;

pub use error::*;
use log::{debug, info};
pub use metadata_wrapper::OutputFormat;
use std::{
	fs::File,
	io::prelude::*,
	path::{Path, PathBuf},
	str::FromStr,
};
pub use substrate_differ::differs::diff_method::DiffMethod;
use substrate_differ::differs::reduced::{reduced_diff_result::ReducedDiffResult, reduced_runtime::ReducedRuntime};
use wasm_loader::{BlockRef, Compression, NodeEndpoint, OnchainBlock, Source, WasmLoader};
use wasm_testbed::WasmTestBed;

pub use chain_info::*;
pub use runtime_info::*;
pub use source::*;
pub use subwasm::*;
pub use types::*;
pub use utils::*;

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
///
/// If `chain` is passed and is a supported chain
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

// /// Get the Source of some wasm based on the user's input
// /// If --chain NAME is passed and NAME is a supported chain
// /// we return a random node from the known list for chain NAME.
// /// If not, we fall back to the `source`
// pub fn get_source(chain: Option<&str>, source: Source, block_ref: Option<String>) -> Result<Source> {
// 	// let node_url = get_node_url(chain);

// 	// if let Some(chain_url) = node_url {
// 	// 	let endpoint = NodeEndpoint::from_str(&chain_url)?;
// 	// 	Ok(Source::Chain(OnchainBlock { endpoint, block_ref }))
// 	// } else {
// 	// 	Ok(source)
// 	// }
// }

/// Use the user's wish if any or make up a target
pub fn get_output_file(wish: Option<PathBuf>) -> PathBuf {
	match wish {
		Some(path) => path,

		_ => {
			let mut i = 0;
			let mut path;

			loop {
				path = format!("runtime_{i:03?}.wasm");
				i += 1;
				assert!(i < 1000, "Ran out of indexes");
				if !Path::new(&path).exists() {
					break;
				}
			}
			PathBuf::from(path)
		}
	}
}

/// Fetch the runtime from a node and store the wasm locally
pub fn download_runtime(url: &str, block_ref: Option<BlockRef>, output: Option<PathBuf>) -> Result<()> {
	let url = match url {
		url if url.starts_with("ws") => NodeEndpoint::WebSocket(url.to_string()),
		url if url.starts_with("http") => NodeEndpoint::Http(url.to_string()),
		_ => {
			return Err(SubwasmLibError::Parsing(
				url.to_string(),
				"The url should either start with http or ws".to_string(),
			));
		}
	};

	let reference = OnchainBlock { endpoint: url, block_ref };
	log::info!("Downloading runtime from  {:?}", reference);

	let loader =
		wasm_loader::WasmLoader::load_from_source(&Source::Chain(reference)).expect("Getting wasm from the node");
	let wasm = loader.original_bytes();

	log::info!("Got the runtime, its size is {:?}", wasm.len());

	let outfile = get_output_file(output);

	info!("Saving runtime to {outfile:?}");
	let mut buffer = File::create(outfile)?;
	buffer.write_all(wasm)?;
	Ok(())
}

pub fn reduced_diff(src_a: Source, src_b: Source) -> Result<ReducedDiffResult> {
	log::debug!("REDUCED: Loading WASM runtimes:");
	log::info!("  🅰️  {:?}", src_a);
	let runtime_a = WasmTestBed::new(&src_a)?;
	log::info!("  🅱️  {:?}", src_b);
	let runtime_b = WasmTestBed::new(&src_b)?;

	let ra = ReducedRuntime::from(runtime_a.metadata());
	let rb = ReducedRuntime::from(runtime_b.metadata());

	Ok(ReducedDiffResult::new(ra, rb))
}

/// Compress a given runtime into a new file. You cannot compress
/// a runtime that is already compressed.
pub fn compress(input: PathBuf, output: PathBuf) -> Result<()> {
	let wasm = WasmLoader::load_from_source(&Source::File(input))?;

	if wasm.compression().compressed() {
		return Err(error::SubwasmLibError::AlreadyCompressed());
	}

	let bytes_compressed =
		Compression::compress(wasm.original_bytes()).map_err(|_e| error::SubwasmLibError::CompressionFailed())?;

	debug!("original   = {:?}", wasm.original_bytes().len());
	debug!("compressed = {:?}", bytes_compressed.len());
	info!("Saving compressed runtime to {:?}", output);

	let mut buffer = File::create(output)?;
	buffer.write_all(&bytes_compressed.to_vec())?;

	Ok(())
}

/// Decompress a given runtime file. It is fine decompressing an already
/// decompressed runtime, you will just get the same.
pub fn decompress(input: PathBuf, output: PathBuf) -> Result<()> {
	let wasm = WasmLoader::load_from_source(&Source::File(input))?;

	let bytes_decompressed = match wasm.compression().compressed() {
		false => wasm.original_bytes().clone(),
		true => Compression::decompress(wasm.original_bytes())
			.map_err(|_e| error::SubwasmLibError::DecompressionFailed())?,
	};

	debug!("original     = {:?}", wasm.original_bytes().len());
	debug!("decompressed = {:?}", bytes_decompressed.len());

	info!("Saving decompressed runtime to {:?}", output);
	let mut buffer = File::create(output)?;
	buffer.write_all(&bytes_decompressed.to_vec())?;

	Ok(())
}

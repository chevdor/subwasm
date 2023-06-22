#![allow(clippy::derive_partial_eq_without_eq)]

pub mod error;
pub mod source;

mod chain_info;
mod chain_urls;
mod convert;
mod github_ref;
mod macros;
mod metadata_wrapper;
mod runtime_info;
mod subwasm;
mod types;
mod utils;

use std::{fs::File, io::prelude::*, path::PathBuf, str::FromStr};
use substrate_differ::differs::reduced::{reduced_diff_result::ReducedDiffResult, reduced_runtime::ReducedRuntime};
use url::Url;
use wasm_loader::{BlockRef, Compression, NodeEndpoint, OnchainBlock, Source, WasmLoader};
use wasm_testbed::WasmTestBed;

pub use chain_info::*;
pub use error::*;
pub use github_ref::*;
pub use metadata_wrapper::OutputFormat;
pub use runtime_info::*;
pub use source::*;
pub use substrate_differ::differs::diff_method::DiffMethod;
pub use subwasm::*;
pub use types::*;
pub use utils::*;

/// Returns Some node url if possible, None otherwise.
fn get_node_url(chain: &str) -> Result<Url> {
	let chain_info = ChainInfo::from_str(chain).expect("Unknown chain");
	chain_info.get_random_url(None)
}

/// Get the url of a node based on the user's input
///
/// If `chain` is passed and is a supported chain
/// we return a random node from the known list for chain NAME.
/// If not, we fall back to the --url flag
pub fn get_url(chain: Option<&str>, reference: &OnchainBlock) -> Result<Url> {
	if chain.is_none() {
		return Err(SubwasmLibError::Generic("Missing chain input".to_string()));
	}
	let chain = chain.expect("Chain was provided");

	let url =
		reference.endpoint.as_url().map_err(|_e| SubwasmLibError::EndpointNotFound(reference.endpoint.to_string()));
	let node_url = get_node_url(chain);

	if let Ok(chain_url) = node_url {
		Ok(chain_url)
	} else {
		url
	}
}

/// Fetch the runtime from a node and store the wasm locally.
/// The wasm is store at the provided target or into a file name that is generated.
pub fn download_runtime(
	endpoint: NodeEndpoint,
	block_ref: Option<BlockRef>,
	target: Option<PathBuf>,
) -> Result<PathBuf> {
	let reference = OnchainBlock { endpoint, block_ref };
	log::info!("Downloading runtime from  {:?}", reference);

	let loader =
		wasm_loader::WasmLoader::load_from_source(&Source::Chain(reference)).expect("Getting wasm from the node");
	let wasm = loader.original_bytes();

	log::info!("Got the runtime, its size is {:?}", wasm.len());

	let outfile = get_output_file_local(target);

	log::info!("Saving runtime to {outfile:?}");
	let mut buffer = File::create(&outfile)?;
	buffer.write_all(wasm)?;
	Ok(outfile)
}

/// Compute the diff of 2 runtimes
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

	log::debug!("original   = {:?}", wasm.original_bytes().len());
	log::debug!("compressed = {:?}", bytes_compressed.len());
	log::info!("Saving compressed runtime to {:?}", output);

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

	log::debug!("original     = {:?}", wasm.original_bytes().len());
	log::debug!("decompressed = {:?}", bytes_decompressed.len());

	log::info!("Saving decompressed runtime to {:?}", output);
	let mut buffer = File::create(output)?;
	buffer.write_all(&bytes_decompressed.to_vec())?;

	Ok(())
}

use log::{debug, info};
use std::path::Path;
use std::{fs::File, path::PathBuf};
use std::{io::prelude::*, str::FromStr};
pub use substrate_differ::differs::diff_method::DiffMethod;
use substrate_differ::differs::{
	raw::{raw_differ::RawDiffer, raw_differ_options::RawDifferOptions},
	reduced::reduced_differ::ReducedDiffer,
	summary::RuntimeSummaryDiffer,
	DiffOptions,
};
use wasm_loader::{BlockRef, Compression, NodeEndpoint, OnchainBlock, Source, WasmLoader};
use wasm_testbed::WasmTestBed;
mod chain_info;
mod chain_urls;
mod convert;
mod error;
mod macros;
mod metadata_wrapper;
mod runtime_info;
mod subwasm;
mod types;
pub use chain_info::*;
pub use runtime_info::*;
pub use subwasm::*;
pub use types::*;

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
pub fn get_source(chain: Option<&str>, source: Source, block_ref: Option<String>) -> Source {
	let node_url = get_node_url(chain);

	if let Some(chain_url) = node_url {
		let endpoint = NodeEndpoint::from_str(&chain_url).unwrap();
		Source::Chain(OnchainBlock { endpoint, block_ref })
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
	log::info!("Downloading runtime from  {:?}", reference);

	let loader =
		wasm_loader::WasmLoader::load_from_source(&Source::Chain(reference)).expect("Getting wasm from the node");
	let wasm = loader.original_bytes();

	log::info!("Got the runtime, its size is {:?}", wasm.len());

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

	log::info!("Saving runtime to {:?}", outfile);
	let mut buffer = File::create(outfile)?;
	buffer.write_all(wasm)?;
	Ok(())
}

/// Compare 2 runtimes. It compares their versions first
/// then their metata.
pub fn diff(src_a: Source, src_b: Source) {
	log::debug!("DIFF: Loading WASM runtimes:");
	println!("  🅰️  {:?}", src_a);
	let runtime_a = WasmTestBed::new(&src_a).expect("Can only diff if the 2 runtimes can load");
	println!("  🅱️  {:?}", src_b);
	let runtime_b = WasmTestBed::new(&src_b).expect("Can only diff if the 2 runtimes can load");

	// ==== RUNTIME
	let runtime_diff = RuntimeSummaryDiffer::new(&runtime_a, &runtime_b);
	runtime_diff.compare();

	// ==== RAW
	if runtime_a.metadata_version() == runtime_b.metadata_version() {
		let metadiff = RawDiffer::new(runtime_a.metadata(), runtime_b.metadata());
		metadiff.compare(RawDifferOptions::default());
	} else {
		println!("Raw comparison of runtimes with different version is not supported.");
	}

	// // ==== PARTIAL
	// if runtime_a.metadata_version() == runtime_b.metadata_version() {
	// 	let partial = MetadataPartialDiffer::new(runtime_a.metadata(), runtime_b.metadata());
	// 	partial.compare_reduced();
	// } else {
	// 	println!("Partial comparison of runtimes with different version is not supported.");
	// }
}

pub fn reduced_diff(src_a: Source, src_b: Source) {
	log::debug!("REDUCED: Loading WASM runtimes:");
	println!("  🅰️  {:?}", src_a);
	let runtime_a = WasmTestBed::new(&src_a).expect("Can only diff if the 2 runtimes can load");
	println!("  🅱️  {:?}", src_b);
	let runtime_b = WasmTestBed::new(&src_b).expect("Can only diff if the 2 runtimes can load");

	let _partial = ReducedDiffer::new(runtime_a.metadata(), runtime_b.metadata());
	let _opts = DiffOptions::default();

	// partial.diff(opts);
	todo!();
}

/// Compress a given runtime into a new file. You cannot compress
/// a runtime that is already compressed.
pub fn compress(input: PathBuf, output: PathBuf) -> Result<(), String> {
	let wasm = WasmLoader::load_from_source(&Source::File(input)).unwrap();

	if wasm.compression().compressed() {
		return Err("The input is already compressed".into());
	}

	let bytes_compressed = Compression::compress(wasm.original_bytes()).unwrap();

	debug!("original   = {:?}", wasm.original_bytes().len());
	debug!("compressed = {:?}", bytes_compressed.len());
	info!("Saving compressed runtime to {:?}", output);

	let mut buffer = File::create(output).unwrap();
	buffer.write_all(&bytes_compressed.to_vec()).unwrap();

	Ok(())
}

/// Decompress a given runtime file. It is fine decompressing an already
/// decompressed runtime, you will just get the same.
pub fn decompress(input: PathBuf, output: PathBuf) -> Result<(), String> {
	let wasm = WasmLoader::load_from_source(&Source::File(input)).unwrap();

	let bytes_decompressed = match wasm.compression().compressed() {
		false => wasm.original_bytes().clone(),
		true => Compression::decompress(wasm.original_bytes()).unwrap(),
	};

	debug!("original     = {:?}", wasm.original_bytes().len());
	debug!("decompressed = {:?}", bytes_decompressed.len());

	info!("Saving decompressed runtime to {:?}", output);
	let mut buffer = File::create(output).unwrap();
	buffer.write_all(&bytes_decompressed.to_vec()).unwrap();

	Ok(())
}

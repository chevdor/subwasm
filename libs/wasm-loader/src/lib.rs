#![feature(external_doc)]
#![doc(include = "../README.md")]

use rpc_client::{OnchainBlock, RpcClient};
use std::io::Read;
use std::{fs, fs::File, path::Path, path::PathBuf};

/// The source of the wasm. It can come from the local file system (`File`) or from a chain (`Chain`).
pub enum Source {
	File(PathBuf),
	Chain(OnchainBlock),
}

pub struct WasmLoader {}

impl WasmLoader {
	/// Load some binary from a file
	fn load_from_file(filename: &Path) -> Result<Vec<u8>, String> {
		let mut f = File::open(&filename).unwrap_or_else(|_| panic!("File {} not found", filename.to_string_lossy()));
		let metadata = fs::metadata(&filename).expect("unable to read metadata");
		let mut buffer = vec![0; metadata.len() as usize];
		f.read_exact(&mut buffer).expect("buffer overflow");

		Ok(buffer)
	}

	/// Load wasm from a node
	fn load_from_node(reference: OnchainBlock) -> Result<Vec<u8>, String> {
		match RpcClient::fetch_wasm(reference) {
			Ok(wasm) => Ok(wasm),
			Err(e) => Err(e),
		}
	}

	/// Load the binary wasm from a file or from a running node via rpc
	pub fn load(source: Source) -> Result<Vec<u8>, String> {
		match source {
			Source::File(f) => Self::load_from_file(&f),
			Source::Chain(n) => Self::load_from_node(n),
		}
	}
}

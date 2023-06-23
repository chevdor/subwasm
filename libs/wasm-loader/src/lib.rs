#![allow(clippy::derive_partial_eq_without_eq)]

mod compression;
pub mod error;
mod node_endpoint;
mod onchain_block;
mod source;

pub use compression::Compression;
pub use error::*;
pub use node_endpoint::NodeEndpoint;
pub use onchain_block::{BlockRef, OnchainBlock};
pub use source::Source;

use log::*;
use serde::Deserialize;
use std::fmt::Debug;
use std::io::Read;
use std::{fs::File, path::Path};
use subrpcer::state;
use tungstenite::Message;

const CODE: &str = "0x3a636f6465"; // :code in hex
pub const CODE_BLOB_BOMB_LIMIT: usize = 50 * 1024 * 1024;
pub type WasmBytes = Vec<u8>;

pub enum CompressedMaybe {
	Uncompressed(Vec<u8>),
	Compressed((Vec<u8>, Vec<u8>)),
}

/// The WasmLoader is there to load wasm whether from a file, a node
/// or from raw bytes. The WasmLoader cannot execute any call into the wasm.
pub struct WasmLoader {
	bytes: CompressedMaybe,
	compression: Compression,
}

impl WasmLoader {
	/// Fetch the wasm blob from a node
	fn fetch_wasm_from_rpc(reference: &OnchainBlock) -> Result<WasmBytes> {
		#[derive(Deserialize)]
		struct Response {
			result: String,
		}

		fn map_err<O, E1, E2>(r: std::result::Result<O, E1>, e: E2) -> std::result::Result<O, E2>
		where
			E1: Debug,
		{
			r.map_err(|e_| {
				eprintln!("{e_:?}");
				e
			})
		}

		let block_ref = reference.block_ref.as_ref();
		let data = state::get_storage(0, CODE, block_ref);
		let wasm_hex = match &reference.endpoint {
			NodeEndpoint::Http(url) => {
				map_err(ureq::post(url).send_json(data), WasmLoaderError::HttpClient(url.to_string()))?
					.into_json::<Response>()
					.expect("unexpected response from node")
					.result
			}
			NodeEndpoint::WebSocket(url) => {
				let mut ws = map_err(tungstenite::connect(url), WasmLoaderError::WsClient(url.to_string()))?.0;

				map_err(
					ws.write_message(Message::Binary(serde_json::to_vec(&data).expect("invalid data"))),
					WasmLoaderError::WsClient(url.to_string()),
				)?;

				let mut wasm_hex = None;

				// One for Ping, one for response.
				for _ in 0..2_u8 {
					let Message::Text(t) = map_err(ws.read_message(), WasmLoaderError::WsClient(url.to_string()))? else {
						continue

					};

					wasm_hex = serde_json::from_str::<Response>(&t).map(|r| r.result).ok();
				}

				wasm_hex.expect("unexpected response from node")
			}
		};
		let wasm = array_bytes::hex2bytes(wasm_hex).expect("Decoding bytes");

		Ok(wasm)
	}

	/// Load some binary from a file
	fn load_from_file(filename: &Path) -> WasmBytes {
		let mut f = File::open(filename).unwrap_or_else(|_| panic!("File {} not found", filename.to_string_lossy()));
		let mut buffer = Vec::new();
		f.read_to_end(&mut buffer).expect("failed loading file");
		log::debug!("read data from file, buffer size: {:?}", buffer.len());
		buffer
	}

	pub fn compression(&self) -> Compression {
		self.compression
	}

	/// Load wasm from a node
	fn load_from_node(reference: &OnchainBlock) -> Result<WasmBytes> {
		WasmLoader::fetch_wasm_from_rpc(reference)
	}

	/// Returns the 'usable' uncompressed bytes. You get either the raw bytes if the
	/// wasm was not compressed, or the decompressed bytes if the runtime
	/// was compressed.
	/// See also `original_bytes` if you need the compressed bytes.
	pub fn uncompressed_bytes(&self) -> &WasmBytes {
		match &self.bytes {
			CompressedMaybe::Compressed(b) => &b.0,
			CompressedMaybe::Uncompressed(b) => b,
		}
	}

	/// Return the wasm bytes as retrieved.
	/// See `uncompressed_bytes` if you need to use the WASM.
	pub fn original_bytes(&self) -> &WasmBytes {
		match &self.bytes {
			CompressedMaybe::Compressed(b) => &b.1,
			CompressedMaybe::Uncompressed(b) => b,
		}
	}

	pub fn load_from_bytes(bytes: CompressedMaybe, compression: Compression) -> Result<Self> {
		Ok(Self { bytes, compression })
	}

	/// Load the binary wasm from a file or from a running node via rpc
	pub fn load_from_source(source: &Source) -> Result<Self> {
		log::debug!("Loading from {:?}", source);
		let bytes = match source {
			Source::File(f) => Ok(Self::load_from_file(f)),
			Source::Chain(n) => Self::load_from_node(n),
		}?;
		log::debug!("Loaded {:?} bytes", bytes.len());

		debug!("code size before decompression: {:?}", bytes.len());
		let bytes_decompressed = sp_maybe_compressed_blob::decompress(&bytes, CODE_BLOB_BOMB_LIMIT)?;

		let compression = Compression::new(&bytes, &bytes_decompressed);

		// .map_err(|e| format!("Decompression error: {:?}", e))?;
		debug!(
			"code size after decompression {:?}  {:?}",
			bytes_decompressed.len(),
			bytes_decompressed[0..64].to_vec()
		);

		match compression.compressed() {
			true => Self::load_from_bytes(
				CompressedMaybe::Compressed((bytes_decompressed.to_vec(), bytes.to_vec())),
				compression,
			),
			false => Self::load_from_bytes(CompressedMaybe::Uncompressed(bytes.to_vec()), compression),
		}
	}
}

#[cfg(test)]
pub mod tests {
	use super::*;
	use std::env;

	#[cfg(test)]
	pub fn ensure_local_wasm() -> String {
		use assert_cmd::Command;
		use std::path::PathBuf;

		const MAX_RETRIES: u8 = 10;
		const WASM_FILE: &str = "/tmp/runtime.wasm";
		let mut retry = 0;

		let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Failed loading wasm");

		if PathBuf::from(WASM_FILE).exists() {
			return WASM_FILE.to_string();
		} else {
			while retry < MAX_RETRIES {
				let assert = cmd.args(["get", "wss://rpc.polkadot.io:443", "--output", WASM_FILE]).assert();

				if assert.try_success().is_ok() {
					return String::from(WASM_FILE);
				}

				retry += 1
			}
		}

		panic!("Failed fetching a runtime")
	}

	fn get_ws_node(archive: bool) -> String {
		if !archive {
			env::var("POLKADOT_WS").unwrap_or_else(|_| "ws://localhost:9944".to_string())
		} else {
			env::var("POLKADOT_WS_ARCHIVE").unwrap_or_else(|_| "ws://localhost:9944".to_string())
		}
	}

	#[test]
	#[ignore = "need node"]
	fn fetch_should_work() {
		assert!(WasmLoader::fetch_wasm_from_rpc(
			&OnchainBlock::new("https://rpc.polkadot.io", None).expect("Can parse RPC node")
		)
		.is_ok());
		assert!(WasmLoader::fetch_wasm_from_rpc(
			&OnchainBlock::new("wss://rpc.polkadot.io", None).expect("Can parse RPC node")
		)
		.is_ok());
	}

	#[test]
	#[ignore = "need node"]
	fn it_fetches_a_wasm_from_node_via_ws() {
		let url = get_ws_node(false);
		println!("Connecting to {:?}", &url);
		let reference = OnchainBlock { endpoint: NodeEndpoint::WebSocket(url), block_ref: None };
		let loader = WasmLoader::load_from_source(&Source::Chain(reference)).expect("Failed loading wasm");
		let wasm = loader.uncompressed_bytes();
		println!("uncompressed wasm size: {:?}", wasm.len());
		assert!(wasm.len() > 1_000_000);
	}

	#[test]
	#[ignore = "need node"]
	fn it_fetches_the_compressed_runtime() {
		let url = get_ws_node(false);
		println!("Connecting to {:?}", &url);
		let reference = OnchainBlock { endpoint: NodeEndpoint::WebSocket(url), block_ref: None };
		let loader = WasmLoader::load_from_source(&Source::Chain(reference)).expect("Failed loading wasm");
		let uncompressed_bytes = loader.uncompressed_bytes();
		let original_bytes = loader.original_bytes();
		println!("uncompressed wasm size: {:?}", uncompressed_bytes.len());
		println!("original wasm size: {:?}", original_bytes.len());
		assert!(uncompressed_bytes.len() > 1_000_000);
		assert!(uncompressed_bytes.len() >= original_bytes.len());
	}

	#[test]
	#[ignore = "need archive node"]
	fn it_fetches_wasm_from_a_given_block() {
		const POLKADOT_BLOCK20: &str = "0x4d6a0bca208b85d41833a7f35cf73d1ae6974f4bad8ab576e2c3f751d691fe6c"; // Polkadot Block #20

		let url = get_ws_node(true);
		println!("Connecting to {:?}", &url);
		let latest = OnchainBlock { endpoint: NodeEndpoint::WebSocket(url.clone()), block_ref: None };
		let older =
			OnchainBlock { endpoint: NodeEndpoint::WebSocket(url), block_ref: Some(POLKADOT_BLOCK20.to_string()) };

		let loader_latest = WasmLoader::load_from_source(&Source::Chain(latest)).expect("Failed loading wasm");
		let wasm_latest = loader_latest.uncompressed_bytes();

		let loader_older = WasmLoader::load_from_source(&Source::Chain(older)).expect("Failed loading wasm");
		let wasm_older = loader_older.uncompressed_bytes();

		println!("wasm latest size: {:?}", wasm_latest.len());
		println!("wasm older size: {:?}", wasm_older.len());
		assert!(wasm_latest.len() > 1_000_000);
		assert!(wasm_older.len() > 1_000_000);
		assert!(wasm_older.len() != wasm_latest.len()); // this likely changed...
	}
}

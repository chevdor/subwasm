mod compression;
mod error;
mod node_endpoint;
mod onchain_block;
mod source;

pub use compression::Compression;
use error::WasmLoaderError;
use jsonrpsee::{
	http_client::{traits::Client, Error, HttpClientBuilder, JsonValue},
	ws_client::WsClientBuilder,
};
use log::debug;
pub use node_endpoint::NodeEndpoint;
pub use onchain_block::{BlockRef, OnchainBlock};
pub use source::Source;

use std::io::Read;
use std::{fs, fs::File, path::Path};
use tokio::runtime::Runtime;

const CODE: &str = "0x3a636f6465"; // :code in hex
pub const CODE_BLOB_BOMB_LIMIT: usize = 50 * 1024 * 1024;
pub type WasmBytes = Vec<u8>;

pub enum CompressedMaybe {
	Uncompressed(Vec<u8>),
	Compressed((Vec<u8>, Vec<u8>)),
}

/// The WasmLoader is there to load wasm whether from a file, a node
/// or from raw bytes. The WasmLoader cannot execute any call into the wasm.
///
pub struct WasmLoader {
	bytes: CompressedMaybe,
	compression: Compression,
}

impl WasmLoader {
	/// Fetch the wasm blob from a node
	fn fetch_wasm(reference: &OnchainBlock) -> Result<WasmBytes, WasmLoaderError> {
		let block_ref = reference.block_ref.as_ref();
		let params = match block_ref {
			Some(x) => vec![JsonValue::from(CODE), JsonValue::from(x.to_string())],
			None => vec![CODE.into()],
		};

		// Create the runtime
		let rt = Runtime::new().unwrap();
		// TODO: See https://github.com/paritytech/jsonrpsee/issues/298
		let response: Result<String, Error> = match &reference.endpoint {
			NodeEndpoint::Http(url) => {
				let client = HttpClientBuilder::default().build(url).map_err(|_e| WasmLoaderError::HttpClient())?;
				rt.block_on(client.request("state_getStorage", params.into()))
			}
			NodeEndpoint::WebSocket(url) => {
				let client = rt.block_on(WsClientBuilder::default().build(url)).map_err(|_e| {
					println!("{:?}", _e);
					WasmLoaderError::WsClient()
				})?;
				rt.block_on(client.request("state_getStorage", params.into()))
			}
		};

		let wasm = response.unwrap();
		let bytes = hex::decode(wasm.trim_start_matches("0x")).expect("Decoding bytes");
		Ok(bytes)
	}

	/// Load some binary from a file
	fn load_from_file(filename: &Path) -> WasmBytes {
		let mut f = File::open(&filename).unwrap_or_else(|_| panic!("File {} not found", filename.to_string_lossy()));
		let metadata = fs::metadata(&filename).expect("unable to read metadata");
		let mut buffer = vec![0; metadata.len() as usize];
		f.read_exact(&mut buffer).expect("buffer overflow");

		buffer
	}

	pub fn compression(&self) -> Compression {
		self.compression
	}

	/// Load wasm from a node
	fn load_from_node(reference: &OnchainBlock) -> Result<WasmBytes, WasmLoaderError> {
		match WasmLoader::fetch_wasm(reference) {
			Ok(wasm) => Ok(wasm),
			Err(e) => Err(e),
		}
	}

	/// Returns the 'usable' bytes. You get either the raw bytes if the
	/// wasm was not compressed, or the decompressed bytes if the runtime
	/// was compressed.
	pub fn bytes(&self) -> &WasmBytes {
		match &self.bytes {
			CompressedMaybe::Compressed(b) => &b.0,
			CompressedMaybe::Uncompressed(b) => &b,
		}
	}

	pub fn uncompressed_bytes(&self) -> &WasmBytes {
		match &self.bytes {
			CompressedMaybe::Compressed(b) => &b.1,
			CompressedMaybe::Uncompressed(b) => &b,
		}
	}

	pub fn load_from_bytes(bytes: CompressedMaybe, compression: Compression) -> Result<Self, WasmLoaderError> {
		// TODO: Check the bytes for magic number and version
		Ok(Self { bytes, compression })
	}

	/// Load the binary wasm from a file or from a running node via rpc
	pub fn load_from_source(source: &Source) -> Result<Self, WasmLoaderError> {
		let bytes = match source {
			Source::File(f) => Ok(Self::load_from_file(f)),
			Source::Chain(n) => Self::load_from_node(n),
		}?;

		debug!("code size before decompression: {:?}", bytes.len());
		let bytes_decompressed = sp_maybe_compressed_blob::decompress(&bytes, CODE_BLOB_BOMB_LIMIT).unwrap();

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
mod tests {
	use super::*;
	use std::env;

	fn get_http_node() -> String {
		env::var("POLKADOT_HTTP").unwrap_or("http://localhost:9933".to_string())
	}

	fn get_ws_node() -> String {
		env::var("POLKADOT_WS").unwrap_or("ws://localhost:9944".to_string())
	}

	#[test]
	#[ignore = "needs node"]
	fn it_fetches_a_wasm_from_node_via_http() {
		let url = String::from(get_http_node());
		println!("Connecting to {:?}", &url);
		let reference = OnchainBlock { endpoint: NodeEndpoint::Http(url), block_ref: None };

		let loader = WasmLoader::load_from_source(&Source::Chain(reference)).unwrap();
		let wasm = loader.bytes();

		println!("uncompressed wasm size: {:?}", wasm.len());
		assert!(wasm.len() > 1_000_000);
	}

	#[test]
	#[ignore = "needs node"]
	fn it_fetches_a_wasm_from_node_via_ws() {
		let url = String::from(get_ws_node());
		println!("Connecting to {:?}", &url);
		let reference = OnchainBlock { endpoint: NodeEndpoint::WebSocket(url), block_ref: None };
		let loader = WasmLoader::load_from_source(&Source::Chain(reference)).unwrap();
		let wasm = loader.bytes();
		println!("uncompressed wasm size: {:?}", wasm.len());
		assert!(wasm.len() > 1_000_000);
	}

	#[test]
	#[ignore = "needs node"]
	fn it_fetches_wasm_from_a_given_block() {
		const POLKADOT_BLOCK20: &str = "0x4d6a0bca208b85d41833a7f35cf73d1ae6974f4bad8ab576e2c3f751d691fe6c"; // Polkadot Block #20

		let url = String::from(get_ws_node());
		println!("Connecting to {:?}", &url);
		let latest = OnchainBlock { endpoint: NodeEndpoint::WebSocket(url.clone()), block_ref: None };
		let older =
			OnchainBlock { endpoint: NodeEndpoint::WebSocket(url), block_ref: Some(POLKADOT_BLOCK20.to_string()) };

		let loader_latest = WasmLoader::load_from_source(&Source::Chain(latest)).unwrap();
		let wasm_latest = loader_latest.bytes();

		let loader_older = WasmLoader::load_from_source(&Source::Chain(older)).unwrap();
		let wasm_older = loader_older.bytes();

		println!("wasm latest size: {:?}", wasm_latest.len());
		println!("wasm older size: {:?}", wasm_older.len());
		assert!(wasm_latest.len() > 1_000_000);
		assert!(wasm_older.len() > 1_000_000);
		assert!(wasm_older.len() != wasm_latest.len()); // this likely changed...
	}
}

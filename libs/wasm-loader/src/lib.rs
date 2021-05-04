mod node_endpoint;
mod onchain_block;
mod source;

use jsonrpsee::{
	http_client::{traits::Client, Error, HttpClientBuilder, JsonValue},
	ws_client::WsClientBuilder,
};
pub use node_endpoint::NodeEndpoint;
pub use onchain_block::{BlockRef, OnchainBlock};
pub use source::Source;

use std::io::Read;
use std::{fs, fs::File, path::Path};
use tokio::runtime::Runtime;

pub struct WasmLoader {}

impl WasmLoader {
	/// Load some binary from a file
	fn load_from_file(filename: &Path) -> Vec<u8> {
		let mut f = File::open(&filename).unwrap_or_else(|_| panic!("File {} not found", filename.to_string_lossy()));
		let metadata = fs::metadata(&filename).expect("unable to read metadata");
		let mut buffer = vec![0; metadata.len() as usize];
		f.read_exact(&mut buffer).expect("buffer overflow");

		buffer
	}

	pub fn fetch_wasm(reference: &OnchainBlock) -> Result<Vec<u8>, Error> {
		// let code = "0x3a636f6465".to_string(); // :code in hex
		let block_ref = reference.block_ref.as_ref();
		let params = match block_ref {
			Some(x) => vec![JsonValue::from("0x3a636f6465"), JsonValue::from(x.to_string())],
			None => vec!["0x3a636f6465".into()],
		};

		// Create the runtime
		let rt = Runtime::new().unwrap();
		// TODO: See https://github.com/paritytech/jsonrpsee/issues/298
		let response: Result<String, Error> = match &reference.endpoint {
			NodeEndpoint::Http(url) => {
				let client = HttpClientBuilder::default().build(url)?;
				rt.block_on(client.request("state_getStorage", params.into()))
			}
			NodeEndpoint::WebSocket(url) => {
				let client = rt.block_on(WsClientBuilder::default().build(&url))?;
				rt.block_on(client.request("state_getStorage", params.into()))
			}
		};

		let wasm = response.unwrap();
		let bytes = hex::decode(wasm.trim_start_matches("0x")).expect("Decoding bytes");
		Ok(bytes)
	}

	/// Load wasm from a node
	fn load_from_node(reference: &OnchainBlock) -> Result<Vec<u8>, String> {
		match WasmLoader::fetch_wasm(reference) {
			Ok(wasm) => Ok(wasm),
			Err(e) => Err(e.to_string()), // TODO: fix that
		}
	}

	/// Load the binary wasm from a file or from a running node via rpc
	pub fn load(source: &Source) -> Result<Vec<u8>, String> {
		match source {
			Source::File(f) => Ok(Self::load_from_file(&f)),
			Source::Chain(n) => Self::load_from_node(n),
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
		let wasm = WasmLoader::fetch_wasm(&reference).unwrap();
		println!("wasm size: {:?}", wasm.len());
		assert!(wasm.len() > 1_000_000);
	}

	#[test]
	#[ignore = "needs node"]
	fn it_fetches_a_wasm_from_node_via_ws() {
		let url = String::from(get_ws_node());
		println!("Connecting to {:?}", &url);
		let reference = OnchainBlock { endpoint: NodeEndpoint::WebSocket(url), block_ref: None };
		let wasm = WasmLoader::fetch_wasm(&reference).unwrap();
		println!("wasm size: {:?}", wasm.len());
		assert!(wasm.len() > 1_000_000);
	}

	#[test]
	#[ignore = "needs node"]
	fn it_fetches_wasm_from_a_given_block() {
		const HASH: &str = "0x4d6a0bca208b85d41833a7f35cf73d1ae6974f4bad8ab576e2c3f751d691fe6c"; // Polkadot #20

		let url = String::from(get_ws_node());
		println!("Connecting to {:?}", &url);
		let latest = OnchainBlock { endpoint: NodeEndpoint::WebSocket(url.clone()), block_ref: None };
		let older = OnchainBlock { endpoint: NodeEndpoint::WebSocket(url), block_ref: Some(HASH.to_string()) };
		let wasm_latest = WasmLoader::fetch_wasm(&latest).unwrap();
		let wasm_older = WasmLoader::fetch_wasm(&older).unwrap();
		println!("wasm latest size: {:?}", wasm_latest.len());
		println!("wasm older size: {:?}", wasm_older.len());
		assert!(wasm_latest.len() > 1_000_000);
		assert!(wasm_older.len() > 1_000_000);
		assert!(wasm_older.len() != wasm_latest.len()); // this likely changed...
	}
}

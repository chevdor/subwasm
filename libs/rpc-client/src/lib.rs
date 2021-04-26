#![feature(external_doc)]
#![doc(include = "../README.md")]

mod onchain_block;
use frame_metadata_subsee::RuntimeMetadataPrefixed;
pub use onchain_block::{BlockRef, OnchainBlock};
use std::time::Duration;

// use substrate_api_client::Api;
// We cannot use substrate-api-client for both http and ws (see https://github.com/scs/substrate-api-client/issues/115)
// so we currently use only http.

/// A [`NodeEndpoint`] can be either `Http` or `WebSocket`. Please note however that only `Http` is currently supported.
#[derive(Debug, Clone)]
pub enum NodeEndpoint {
	Http(String),
	WebSocket(String),
}

/// A very simple RPC client that only supports HTTP.
pub struct RpcClient {}

impl RpcClient {
	fn fetch_wasm_via_http(url: &str, block_ref: Option<BlockRef>) -> Result<Vec<u8>, String> {
		let code = "0x3a636f6465".to_string(); // :code in hex
		let params = match block_ref {
			Some(x) => vec![code, x],
			None => vec![code],
		};

		let data = ureq::json!({
			"jsonrpc": "2.0",
			"method": "state_getStorage",
			"params": &params,
			"id": 1
		});

		let builder = ureq::builder().timeout_connect(Duration::from_secs(10));
		let resp = builder
			.build()
			.post(url)
			.set("Content-Type", "application/json")
			.send_json(data)
			.expect("error fetching metadata from the substrate node");

		let json: serde_json::Value = resp.into_json().expect("Serializing to json");
		let hex_data = json["result"].as_str().expect("metadata result field should be a string");

		let bytes = hex::decode(hex_data.trim_start_matches("0x")).expect("Decoding bytes");
		Ok(bytes)
	}

	pub fn fetch_wasm(reference: OnchainBlock) -> Result<Vec<u8>, String> {
		let block_ref = reference.block_ref;
		match &reference.url {
			NodeEndpoint::Http(url) => Self::fetch_wasm_via_http(&url, block_ref),
			NodeEndpoint::WebSocket(_url) => {
				todo!();
			}
		}
	}

	pub fn fetch_metadata(reference: OnchainBlock) -> Result<RuntimeMetadataPrefixed, String> {
		match reference.url {
			NodeEndpoint::Http(url) => Self::fetch_metadata_via_http(&url, reference.block_ref),
			NodeEndpoint::WebSocket(_url) => {
				todo!();
			}
		}
	}

	/// Fetch the metadata directly from a running node via rpc
	fn fetch_metadata_via_http(url: &str, block_ref: Option<BlockRef>) -> Result<RuntimeMetadataPrefixed, String> {
		let params = match block_ref {
			Some(x) => vec![x],
			None => vec![],
		};

		let resp = ureq::post(url)
			.set("Content-Type", "application/json")
			.send_json(ureq::json!({
				"jsonrpc": "2.0",
				"method": "state_getMetadata",
				"params": &params,
				"id": 1
			}))
			.expect("error fetching metadata from the substrate node");

		let json: serde_json::Value = resp.into_json().expect("Serializing json");
		let hex_data = json["result"].as_str().expect("metadata result field should be a string");

		let bytes = hex::decode(hex_data.trim_start_matches("0x")).expect("Decoding bytes");
		println!("Length: {:?}\n{:02x?}", bytes.len(), bytes[0..128].to_vec());
		let decoded = codec::Decode::decode(&mut &bytes[..]).expect("Decoding runtime metadata");
		Ok(decoded)
	}
}

#[cfg(test)]
mod tests {
	use std::env;

	use super::*;
	const HASH: &str = "0x4d6a0bca208b85d41833a7f35cf73d1ae6974f4bad8ab576e2c3f751d691fe6c"; // Polkadot #20
	fn get_node() -> String { env::var("POLKADOT_HTTP").unwrap_or("http://localhost:9933".to_string()) }

	#[test]
	fn it_fetches_a_wasm_from_node_via_http() {
		let url = String::from(get_node());
		let reference = OnchainBlock { url: NodeEndpoint::Http(url), block_ref: None };
		let wasm = RpcClient::fetch_wasm(reference).unwrap();
		println!("wasm size: {:?}", wasm.len());
		println!("wasm: {:02x?}...", wasm[0..32].to_vec());

		assert!(wasm.len() > 0);
	}

	#[test]
	fn it_fetches_a_wasm_from_node_via_http_latest() {
		let url = String::from(get_node());
		let reference = OnchainBlock { url: NodeEndpoint::Http(url), block_ref: Some(HASH.to_string()) };
		let wasm = RpcClient::fetch_wasm(reference).unwrap();

		println!("wasm size: {:?}", wasm.len());
		println!("wasm: {:02x?}...", wasm[0..32].to_vec());

		assert!(wasm.len() > 0);
	}

	#[test]
	#[should_panic]
	#[ignore = "No WS yet"]
	fn it_fetches_a_wasm_from_node_via_ws() {
		let url = String::from(get_node());
		let reference = OnchainBlock { url: NodeEndpoint::Http(url), block_ref: Some(HASH.to_string()) };
		let wasm = RpcClient::fetch_wasm(reference).unwrap();
		println!("wasm size: {:?}", wasm.len());
		assert!(wasm.len() > 0);
	}

	#[test]
	fn it_fetches_metadata_from_node() {
		let polkadot_http = NodeEndpoint::Http(get_node());
		let reference = OnchainBlock { url: polkadot_http, block_ref: None };
		let _metadata = RpcClient::fetch_metadata(reference);
	}
}

use crate::{error::WasmTestbedError, NodeEndpoint};
use std::str::FromStr;
pub type BlockRef = String; // TODO: to change

/// This structure points to a node url and an optional block reference.
#[derive(Debug, Clone, PartialEq)]
pub struct OnchainBlock {
	pub endpoint: NodeEndpoint,
	pub block_ref: Option<BlockRef>,
}

impl FromStr for OnchainBlock {
	type Err = WasmTestbedError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let endpoint = match s {
			url if url.starts_with("ws") => Some(NodeEndpoint::WebSocket(url.to_string())),
			url if url.starts_with("http") => Some(NodeEndpoint::Http(url.to_string())),
			_ => None,
		}
		.expect(&format!("Invalid endpoint url: {}", s));

		Ok(OnchainBlock { endpoint, block_ref: None })
	}
}

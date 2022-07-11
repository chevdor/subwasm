use crate::{error::WasmLoaderError, NodeEndpoint};
use std::str::FromStr;
pub type BlockRef = String; // TODO: to change

/// This structure points to a node url and an optional block reference.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OnchainBlock {
	pub endpoint: NodeEndpoint,
	pub block_ref: Option<BlockRef>,
}

impl OnchainBlock {
	pub fn new(url: &str, block_ref: Option<BlockRef>) -> Self {
		Self { endpoint: NodeEndpoint::from_str(url).unwrap(), block_ref }
	}
}

impl FromStr for OnchainBlock {
	type Err = WasmLoaderError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let endpoint = NodeEndpoint::from_str(s).unwrap();
		Ok(OnchainBlock { endpoint, block_ref: None })
	}
}

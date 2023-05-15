use crate::{
	error::{self, *},
	NodeEndpoint,
};
use std::str::FromStr;

pub type BlockRef = String;

/// This structure points to a node url and an optional block reference.
#[derive(Debug, Clone, PartialEq)]
pub struct OnchainBlock {
	pub endpoint: NodeEndpoint,
	pub block_ref: Option<BlockRef>,
}

impl OnchainBlock {
	pub fn new(url: &str, block_ref: Option<BlockRef>) -> error::Result<Self> {
		Ok(Self { endpoint: NodeEndpoint::from_str(url)?, block_ref })
	}
}

impl FromStr for OnchainBlock {
	type Err = WasmLoaderError;

	fn from_str(s: &str) -> Result<Self> {
		let endpoint = NodeEndpoint::from_str(s)?;
		Ok(OnchainBlock { endpoint, block_ref: None })
	}
}

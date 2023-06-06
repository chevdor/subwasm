use crate::{
	error::{self, WasmLoaderError},
	NodeEndpoint,
};
use std::str::FromStr;
pub type BlockRef = String;

use error::*;
use url::Url;

/// This structure points to a node url and an optional block reference.
#[derive(Debug, Clone, PartialEq)]
pub struct OnchainBlock {
	pub endpoint: NodeEndpoint,
	pub block_ref: Option<BlockRef>,
}

impl OnchainBlock {
	pub fn new(url: &str, block_ref: Option<BlockRef>) -> Result<Self> {
		let endpoint = NodeEndpoint::from_str(url)?;
		Ok(Self { endpoint, block_ref })
	}
}

impl FromStr for OnchainBlock {
	type Err = WasmLoaderError;

	fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
		let endpoint = NodeEndpoint::from_str(s)?;
		Ok(endpoint.into())
	}
}

impl TryFrom<Url> for OnchainBlock {
	type Error = WasmLoaderError;

	fn try_from(url: Url) -> std::result::Result<Self, Self::Error> {
		let endpoint = NodeEndpoint::from_str(url.as_str())?;
		Ok(endpoint.into())
	}
}

impl From<NodeEndpoint> for OnchainBlock {
	fn from(endpoint: NodeEndpoint) -> Self {
		OnchainBlock { endpoint, block_ref: None }
	}
}

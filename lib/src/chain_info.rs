use error::*;
use rand::seq::SliceRandom;
use std::str::FromStr;
use thiserror::Error;
use url::Url;
use wasm_loader::NodeEndpoint;

use crate::{chain_urls::get_chain_urls, error, SubwasmLibError};
#[derive(Debug, PartialEq)]
pub enum EndpointType {
	Http,
	WesbSocket,
}

impl PartialEq<NodeEndpoint> for EndpointType {
	fn eq(&self, other: &NodeEndpoint) -> bool {
		match other {
			NodeEndpoint::Http(_) => self == &EndpointType::Http,
			NodeEndpoint::WebSocket(_) => self == &EndpointType::WesbSocket,
		}
	}
}

#[derive(Debug, Clone)]
pub struct ChainInfo {
	pub name: String,
	pub endpoints: Vec<NodeEndpoint>,
}

impl ChainInfo {
	/// Returns one random url from the list of know nodes
	pub fn get_random_url(&self, filter: Option<EndpointType>) -> Result<Url> {
		let endpoints = &self.endpoints;
		let filtered = endpoints
			.iter()
			.filter(|&ep| //true,
					if let Some(endpoint_type) = &filter {
							endpoint_type == ep
					} else   {
						true
					}
				)
			.collect::<Vec<&NodeEndpoint>>()
			.choose(&mut rand::thread_rng())
			.map(|x| x.to_string());

		if let Some(url) = filtered {
			Url::try_from(url.as_str()).map_err(|e| e.into())
		} else {
			Err(SubwasmLibError::NotFound("No node found for filter {filter:?}".to_string()))
		}
	}
}

#[derive(Debug, Error)]
pub enum ChainInfoError {
	#[error("Unsupported chain: {0}")]
	ChainUsupported(String),
	#[error("Chain not found: {0}")]
	ChainNotFound(String),
}

impl From<SubwasmLibError> for ChainInfoError {
	fn from(e: SubwasmLibError) -> Self {
		Self::ChainNotFound(e.to_string())
	}
}

impl FromStr for ChainInfo {
	type Err = ChainInfoError;

	fn from_str(name: &str) -> std::result::Result<Self, Self::Err> {
		let name = name.to_lowercase();
		let endpoints = get_chain_urls(name.as_str())?;

		if !endpoints.is_empty() {
			Ok(Self { name, endpoints })
		} else {
			Err(ChainInfoError::ChainUsupported(name))
		}
	}
}

impl From<&str> for ChainInfo {
	fn from(s: &str) -> Self {
		Self::from_str(s).expect("Failed parsing url")
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_gets_chain_endpoints() {
		assert!(ChainInfo::from_str("local").is_ok());
		assert!(ChainInfo::from_str("local").unwrap().endpoints.len() == 1);
		assert!(ChainInfo::from_str("polkadot").is_ok());
		assert!(ChainInfo::from_str("PolkaDOT").is_ok());
		assert!(!ChainInfo::from_str("polkadot").unwrap().endpoints.is_empty());
		assert!(ChainInfo::from_str("foobar").is_err());
	}

	#[test]
	fn it_returns_a_url() {
		let info = ChainInfo::from_str("polkadot").unwrap();
		let _endpoint = info.get_random_url(None).unwrap();
	}

	#[test]
	fn it_returns_a_http_url() {
		let info = ChainInfo::from_str("local").unwrap();
		let endpoint = info.get_random_url(Some(EndpointType::Http)).unwrap();
		assert!(endpoint.to_string().starts_with("http"));
	}

	#[test]
	fn it_returns_a_ws_url() {
		let info = ChainInfo::from_str("polkadot").unwrap();
		let endpoint = info.get_random_url(Some(EndpointType::WesbSocket)).unwrap();
		assert!(endpoint.to_string().starts_with("ws"));
	}

	#[test]
	fn test_chain_info() {
		let ci = ChainInfo::from_str("polkadot").unwrap();
		assert!(!ci.endpoints.is_empty());
	}

	#[test]
	fn test_chain_info_from_url() {
		let ci = ChainInfo::from_str("wss://rpc.polkadot.io:443").unwrap();
		assert!(!ci.endpoints.is_empty());
	}
}

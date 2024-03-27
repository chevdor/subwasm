use crate::{chain_urls::get_chain_urls, error};
use error::*;
use rand::seq::SliceRandom;
use std::str::FromStr;
use thiserror::Error;
use url::Url;
use wasm_loader::NodeEndpoint;

/// An enumeration of the type of endpoints
#[derive(Debug, PartialEq)]
pub enum EndpointType {
	/// http or https endpoint
	Http,

	/// WebSocket endpoint, with or without SSL
	WebSocket,
}

impl PartialEq<NodeEndpoint> for EndpointType {
	fn eq(&self, other: &NodeEndpoint) -> bool {
		match other {
			NodeEndpoint::Http(_) => self == &EndpointType::Http,
			NodeEndpoint::WebSocket(_) => self == &EndpointType::WebSocket,
		}
	}
}

/// A struct to hold the name of a chain and the list of its endpoints.
#[derive(Debug, Clone)]
pub struct ChainInfo {
	/// Name of the chain
	pub name: String,

	/// List of endpoints for the chain [ChainInfo::name]
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

/// Custom error
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

	/// Get a ChainInfo from a name or alias
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
		assert_eq!(1, ChainInfo::from_str("local").expect("Failed getting ChainInfo from string").endpoints.len());
		assert!(ChainInfo::from_str("polkadot").is_ok());
		assert!(ChainInfo::from_str("PolkaDOT").is_ok());
		assert!(!ChainInfo::from_str("polkadot").expect("Failed getting ChainInfo from string").endpoints.is_empty());
		assert!(ChainInfo::from_str("foobar").is_err());
	}

	#[test]
	fn it_returns_a_url() {
		let info = ChainInfo::from_str("polkadot").expect("Failed getting ChainInfo from string");
		let _endpoint = info.get_random_url(None).expect("Failed getting url");
	}

	#[test]
	fn it_returns_a_http_url() {
		let info = ChainInfo::from_str("local").expect("Failed getting ChainInfo from string");
		let endpoint = info.get_random_url(Some(EndpointType::Http)).expect("Failed getting url");
		assert!(endpoint.to_string().starts_with("http"));
	}

	#[test]
	fn it_returns_a_ws_url() {
		let info = ChainInfo::from_str("polkadot").expect("Failed getting ChainInfo from string");
		let endpoint = info.get_random_url(Some(EndpointType::WebSocket)).expect("Failed getting url");
		assert!(endpoint.to_string().starts_with("ws"));
	}

	#[test]
	fn test_chain_info() {
		let ci = ChainInfo::from_str("polkadot").expect("Failed getting ChainInfo from string");
		assert!(!ci.endpoints.is_empty());
	}
}

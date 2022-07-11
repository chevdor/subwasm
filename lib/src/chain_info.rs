use rand::seq::SliceRandom;
use std::{fmt::Display, str::FromStr};
use wasm_loader::NodeEndpoint;

use crate::chain_urls::get_chain_urls;
#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug)]
pub struct ChainInfo {
	pub name: String,
	pub endpoints: Vec<NodeEndpoint>,
}

impl ChainInfo {
	/// Returns one random url from the list of know nodes
	pub fn get_random_url(&self, filter: Option<EndpointType>) -> Option<String> {
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
		filtered
	}
}

#[derive(Debug)]
pub enum Error {
	ChainUsupported(String),
}

impl Display for Error {
	fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Error::ChainUsupported(s) => write!(fmt, "Chain not supported: {}", s),
		}
	}
}

impl FromStr for ChainInfo {
	type Err = Error;

	fn from_str(name: &str) -> Result<Self, Self::Err> {
		let name = name.to_lowercase();
		let urls: Option<Vec<_>> = get_chain_urls(name.as_str());

		if let Some(endpoints) = urls {
			Ok(Self { name, endpoints })
		} else {
			Err(Error::ChainUsupported(name))
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
		assert!(endpoint.starts_with("http"));
	}

	#[test]
	fn it_returns_a_ws_url() {
		let info = ChainInfo::from_str("polkadot").unwrap();
		let endpoint = info.get_random_url(Some(EndpointType::WesbSocket)).unwrap();
		assert!(endpoint.starts_with("ws"));
	}
}

use rand::seq::SliceRandom;
use std::{fmt::Display, str::FromStr};
use wasm_loader::NodeEndpoint;
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
		write!(fmt, "Chain not supported: {}", self)
	}
}

impl FromStr for ChainInfo {
	type Err = Error;

	fn from_str(name: &str) -> Result<Self, Self::Err> {
		let name = name.to_lowercase();
		let urls: Option<Vec<_>> = match name.as_str() {
			"polkadot" => Some(vec![
				"wss://rpc.polkadot.io",
				"wss://polkadot.api.onfinality.io/public-ws",
				"wss://polkadot.elara.patract.io",
			]),
			"kusama" => Some(vec![
				"wss://kusama-rpc.polkadot.io",
				"wss://kusama.api.onfinality.io/public-ws",
				"wss://kusama.elara.patract.io",
			]),
			"westend" => Some(vec![
				"wss://westend-rpc.polkadot.io",
				"wss://westend.api.onfinality.io/public-ws",
				"wss://westend.elara.patract.io",
			]),
			"rococo" => Some(vec![
				"wss://rococo-rpc.polkadot.io",
				"wss://rococo.api.onfinality.io/public-ws",
				// TODO: enable again once https://github.com/paritytech/jsonrpsee/issues/337 is fixed
				// "wss://rococo.elara.patract.io",
			]),
			"local" => Some(vec!["http://localhost:9933"]),
			_ => None,
		}
		.map(|s| s.into_iter().map(|s| NodeEndpoint::from_str(s).expect("Valid chain name")).collect());

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
		assert!(ChainInfo::from_str("polkadot").unwrap().endpoints.len() > 0);
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

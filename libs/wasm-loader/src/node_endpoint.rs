use crate::error::WasmLoaderError;
use std::str::FromStr;

/// A [`NodeEndpoint`] can be either `Http` or `WebSocket`.
#[derive(Debug, Clone, PartialEq)]
pub enum NodeEndpoint {
	Http(String),
	WebSocket(String),
}

impl NodeEndpoint {}

impl ToString for NodeEndpoint {
	fn to_string(&self) -> String {
		String::from(match self {
			Self::Http(s) => s,
			Self::WebSocket(s) => s,
		})
	}
}

impl FromStr for NodeEndpoint {
	type Err = WasmLoaderError;

	fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
		match s {
			url if url.starts_with("ws") => Ok(NodeEndpoint::WebSocket(url.to_string())),
			url if url.starts_with("http") => Ok(NodeEndpoint::Http(url.to_string())),
			_ => Err(WasmLoaderError::NotSupported(format!("Unsuported endpoint: {s}"))),
		}
	}
}

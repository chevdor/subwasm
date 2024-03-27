use url::Url;

use crate::error::{self};
use error::*;
use std::str::FromStr;

/// A [`NodeEndpoint`] can be either `Http` or `WebSocket`.
#[derive(Debug, Clone, PartialEq)]
pub enum NodeEndpoint {
	Http(String),
	WebSocket(String),
}

impl NodeEndpoint {
	pub fn as_url(&self) -> Result<Url> {
		match self {
			NodeEndpoint::Http(u) | NodeEndpoint::WebSocket(u) => {
				Url::try_from(u.as_str()).map_err(|_e| WasmLoaderError::UrlParsingError(u.to_string()))
			}
		}
	}
}

impl ToString for NodeEndpoint {
	fn to_string(&self) -> String {
		String::from(match self {
			Self::Http(s) | Self::WebSocket(s) => s,
		})
	}
}

impl TryFrom<Url> for NodeEndpoint {
	type Error = WasmLoaderError;

	fn try_from(url: Url) -> std::result::Result<Self, Self::Error> {
		Self::from_str(url.as_str())
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

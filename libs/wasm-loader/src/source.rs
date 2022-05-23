use std::{
	fmt::{Display, Error},
	path::PathBuf,
	str::FromStr,
};

use crate::{NodeEndpoint, OnchainBlock};

/// The source of the wasm. It can come from the local file system (`File`) or from a chain (`Chain`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Source {
	File(PathBuf),
	Chain(OnchainBlock),
}

impl Display for Source {
	fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Source::File(f) => write!(fmt, "{:?}", f),
			Source::Chain(c) => write!(fmt, "{:?}", c),
		}
	}
}

impl FromStr for Source {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let endpoint = match s {
			url if url.starts_with("ws") => Some(NodeEndpoint::WebSocket(url.to_string())),
			url if url.starts_with("http") => Some(NodeEndpoint::Http(url.to_string())),
			_ => None,
		};

		if let Some(endpoint) = endpoint {
			let reference = OnchainBlock { endpoint, block_ref: None };
			Ok(Source::Chain(reference))
		} else {
			Ok(Source::File(PathBuf::from(s)))
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_converts_from_ws_url() {
		let urls = vec!["ws://localhost:9933", "wss://localhost:9933"];

		for url in urls {
			let src = Source::from_str(url).unwrap();
			match src {
				Source::Chain(r) => match r.endpoint {
					NodeEndpoint::WebSocket(ws) => assert_eq!(ws, url),
					_ => unreachable!(),
				},
				_ => unreachable!(),
			}
		}
	}

	#[test]
	fn it_converts_from_http() {
		let urls = vec!["http://localhost:9933", "https://localhost:9933"];

		for url in urls {
			let src = Source::from_str(url).unwrap();
			match src {
				Source::Chain(r) => match r.endpoint {
					NodeEndpoint::Http(http) => assert_eq!(http, url),
					_ => unreachable!(),
				},
				_ => unreachable!(),
			}
		}
	}

	#[test]
	fn it_converts_from_path() {
		let urls = vec!["/foo/bar.wasm", "/ws/foo/bar.wasm"];

		for url in urls {
			assert!(Source::from_str(url) == Ok(Source::File(PathBuf::from(url))));
		}
	}
}

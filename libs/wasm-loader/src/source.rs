use std::{fmt::Display, path::PathBuf, str::FromStr};

use crate::{error, NodeEndpoint, OnchainBlock};
use error::*;

/// The source of the wasm. It can come from the local file system (`File`) or from a chain (`Chain`).
#[derive(Debug, Clone, PartialEq)]
pub enum Source {
	/// A file on your local filesystem
	File(PathBuf),

	/// A remote endpoint we can connect to
	Chain(OnchainBlock),
}

impl Source {
	pub fn get_source_type(s: &str) -> Result<Source> {
		let path = PathBuf::from(s);

		if path.exists() {
			return Ok(Source::File(path));
		}

		if let Ok(endpoint) = NodeEndpoint::from_str(s) {
			return Ok(Self::Chain(endpoint.into()));
		}

		Err(WasmLoaderError::UnknownSource(s.to_string()))
	}
}

impl Display for Source {
	fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Source::File(f) => write!(fmt, "{f:?}"),
			Source::Chain(c) => write!(fmt, "{c:?}"),
		}
	}
}

impl FromStr for Source {
	type Err = WasmLoaderError;

	/// This can handle the path of an existing file or a string that
	/// can be parsed as a valid endpoint (http://... or ws://...)
	fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
		Self::get_source_type(s)
	}
}

#[cfg(test)]
mod tests_source {
	use super::*;
	use std::{env::temp_dir, fs::File};

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
		let mut dir = temp_dir();
		let path = dir.display().to_string();
		let file_name = "{subwasm_fake_runtime}.wasm".to_string();
		dir.push(file_name);
		let _fake_wasm = File::create(dir).expect("We should be able to create a tmpdir");
		let files = vec![path];

		for file in files {
			assert!(Source::from_str(&file) == Ok(Source::File(PathBuf::from(file))));
		}
	}

	#[test]
	fn it_catches_unknown() {
		let v = vec!["foo", "bar"];

		for value in v {
			assert!(Source::from_str(value).is_err());
		}
	}
}

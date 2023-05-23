use error::*;
use std::fmt::Display;
use std::path::PathBuf;
use std::str::FromStr;
use wasm_loader::OnchainBlock;
use wasm_loader::Source as WasmLoaderSource;

use crate::chain_urls::get_chain_urls;
use crate::error;

/// The wasmloader provides a basic Source struct that
/// can handle only a file or RPC endpoint.
/// This Enum here is fancier and will allow more sources.
#[derive(Debug, Clone, PartialEq)]
pub enum Source {
	/// A file on your local filesystem
	File(PathBuf),

	/// A remote endpoint we can connect to
	Chain(OnchainBlock),

	/// A chain alias such as "westend" or "wnd"
	Alias(String),
	// curl / wget
	// Github?

	// For now, we can use curl/wget
	// URL // TODO: url such as s3, that should work for github as well
}

impl TryFrom<&str> for Source {
	type Error = SubwasmLibError;

	fn try_from(s: &str) -> std::result::Result<Self, Self::Error> {
		Ok(WasmLoaderSource::from_str(s)?.into())
	}
}

impl From<WasmLoaderSource> for Source {
	fn from(s: WasmLoaderSource) -> Self {
		match s {
			WasmLoaderSource::File(f) => Self::File(f),
			WasmLoaderSource::Chain(c) => Self::Chain(c),
		}
	}
}

impl TryFrom<Source> for WasmLoaderSource {
	type Error = SubwasmLibError;

	fn try_from(val: Source) -> std::result::Result<Self, Self::Error> {
		match val {
			Source::File(f) => Ok(Self::File(f)),
			Source::Chain(c) => Ok(Self::Chain(c)),
			_ => Err(SubwasmLibError::Generic("Cannot convert Source::Alias to WasmLoaderSource".to_string())),
		}
	}
}

impl Source {
	pub fn get_source_type(s: &str) -> Result<Source> {
		// This covers WasmLoaderSource::File and WasmLoaderSource::Chain
		if let Ok(source) = WasmLoaderSource::from_str(s) {
			return Ok(source.into());
		}

		let hit_maybe = get_chain_urls(s);

		if let Ok(_hit) = hit_maybe {
			return Ok(Source::Alias(s.into()));
		}

		Err(SubwasmLibError::UnknownSource(s.into()))
	}
}

impl Display for Source {
	fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Source::File(file) => write!(fmt, "{file:?}"),
			Source::Chain(chain) => write!(fmt, "chain: {chain:?}"),
			Source::Alias(alias) => write!(fmt, "alias: {alias:?}"),
		}
	}
}

// impl FromStr for Source {
// 	type Err = WasmLoaderError;

// 	fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
// 		Self::get_source_type(s)
// 	}
// }

// #[cfg(test)]
// mod tests_source {
// 	use super::*;
// 	use std::{env::temp_dir, fs::File};

// 	#[test]
// 	fn it_converts_from_ws_url() {
// 		let urls = vec!["ws://localhost:9933", "wss://localhost:9933"];

// 		for url in urls {
// 			let src = Source::from_str(url).unwrap();
// 			match src {
// 				Source::Chain(r) => match r.endpoint {
// 					NodeEndpoint::WebSocket(ws) => assert_eq!(ws, url),
// 					_ => unreachable!(),
// 				},
// 				_ => unreachable!(),
// 			}
// 		}
// 	}

// 	#[test]
// 	fn it_converts_from_http() {
// 		let urls = vec!["http://localhost:9933", "https://localhost:9933"];

// 		for url in urls {
// 			let src = Source::from_str(url).unwrap();
// 			match src {
// 				Source::Chain(r) => match r.endpoint {
// 					NodeEndpoint::Http(http) => assert_eq!(http, url),
// 					_ => unreachable!(),
// 				},
// 				_ => unreachable!(),
// 			}
// 		}
// 	}

// 	#[test]
// 	fn it_converts_from_path() {
// 		let mut dir = temp_dir();
// 		let path = dir.display().to_string();
// 		let file_name = "{subwasm_fake_runtime}.wasm".to_string();
// 		dir.push(file_name);
// 		let _fake_wasm = File::create(dir).expect("We should be able to create a tmpdir");
// 		let files = vec![path];

// 		for file in files {
// 			assert!(Source::from_str(&file) == Ok(Source::File(PathBuf::from(file))));
// 		}
// 	}

// 	#[test]
// 	fn it_catches_unknown() {
// 		let v = vec!["foo", "bar"];

// 		for value in v {
// 			assert!(Source::from_str(value).is_err());
// 		}
// 	}
// }

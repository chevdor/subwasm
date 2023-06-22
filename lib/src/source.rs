use error::*;
use std::{fmt::Display, path::PathBuf, str::FromStr};
use url::Url;
use wasm_loader::{BlockRef, OnchainBlock, Source as WasmLoaderSource};

use crate::chain_urls::get_chain_urls;
use crate::download_runtime;
use crate::error;
use crate::fetch_at_url;
use crate::github_ref::GithubRef;
use crate::is_wasm_from_url;
use crate::ChainInfo;

/// The [wasmloader] provides a basic Source struct that
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

	/// A URL to Github, S3, IPFS, etc...
	URL(Url),

	/// A reference to a version in Github in the form of `<runtime>@<version>`
	Github(GithubRef),
}

impl TryFrom<&str> for Source {
	type Error = SubwasmLibError;

	/// Try to parse a string into a Source. It may fail when passing URLs.
	/// If you run into issues, first use the get command and then pass the path
	/// to handle your runtime as a file.
	fn try_from(s: &str) -> std::result::Result<Self, Self::Error> {
		// GithubRef can be parsed
		if let Ok(gh_ref) = GithubRef::from_str(s) {
			return Ok(Source::Github(gh_ref));
		}

		// name / alias:
		// - we have a match when calling get_chain_urls(&str)
		let chain_url = get_chain_urls(s);
		if chain_url.is_ok_and(|v| !v.is_empty()) {
			return Ok(Source::Alias(s.to_string()));
		}

		// First we deal with the easy case of files
		let src = WasmLoaderSource::from_str(s);
		if let Ok(s) = &src {
			match s {
				WasmLoaderSource::File(f) => return Ok(Source::File(f.to_owned())),
				_ => {}
			}
		}

		// This is where we try to be smart about URLs...
		// At that point, we have a url or an error.
		let url = Url::from_str(s).map_err(|_e| SubwasmLibError::UnknownSource(s.to_string()))?;

		// We may be able to be smarter with the next statement if we get too many
		// "random" URLs detected as RPC endpoints (ie. `Source::Chain(_)`).
		if url.to_string().contains("wasm") {
			// http get:
			// - we got a url
			// - we can HTTP get it
			// - the result looks like a wasm (that will be lose...)
			if is_wasm_from_url(&url).is_ok_and(|x| x) {
				log::debug!("What we got at {url} could be some wasm indeed");
				return Ok(Source::URL(url));
			}
		} else {
			// if src.is_ok_and(|s| matches!(s, WasmLoaderSource::Chain(c))) {
			// 	return Ok(Source::Chain(c));
			// }
			if let Ok(s) = &src {
				match s {
					WasmLoaderSource::Chain(c) => return Ok(Source::Chain(c.to_owned())),
					_ => {}
				}
			}
		}

		Err(SubwasmLibError::UnknownSource(s.to_string()))
	}
}

impl TryFrom<PathBuf> for Source {
	type Error = SubwasmLibError;

	fn try_from(s: PathBuf) -> std::result::Result<Self, Self::Error> {
		Ok(WasmLoaderSource::File(s).into())
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
	/// Ultimately, subwasm only works on local files. However,
	/// it offers convenient mechanisms to fetch the runtime.
	/// This function takes care of the work required to get the file.
	pub fn as_file(&self) -> Result<PathBuf> {
		match self {
			// That one is easy :)
			Source::File(i) => Ok(i.to_owned()),

			// Fetch from a URL and store the file in a tmp dir
			Source::URL(u) => fetch_at_url(u.to_owned(), None),

			// Generate the URL and fetch the file to a tmp dir
			Source::Github(gh) => fetch_at_url(gh.as_url(), None),

			// Use the wasm_loader to download the runtime from a node
			Source::Chain(ocb) => download_runtime(ocb.endpoint.to_owned(), ocb.block_ref.to_owned(), None),

			// Get a URL then try fetching the runtime from a RPC node
			Source::Alias(name) => {
				let url = ChainInfo::from_str(name)?.get_random_url(None)?;
				download_runtime(url.try_into()?, None, None)
			}
		}
	}

	pub fn from_options(
		file: Option<PathBuf>,
		chain: Option<ChainInfo>,
		block: Option<BlockRef>,
		url: Option<Url>,
	) -> Result<Self> {
		log::trace!("Getting source from options:");
		log::trace!(" - file : {file:?}");
		log::trace!(" - chain: {chain:?}");
		log::trace!(" - block: {block:?}");
		log::trace!(" - url  : {url:?}");

		if let Some(f) = file {
			return Ok(Self::File(f));
		}

		if let Some(c) = chain {
			let url = c.get_random_url(None)?;
			let onchain_block = OnchainBlock::new(url.as_str(), block)?;
			return Ok(Self::Chain(onchain_block));
		}

		if let Some(u) = url {
			return Ok(Self::URL(u));
		}

		Err(error::SubwasmLibError::UnknownSource(String::from("No file or chain or url provided!")))
	}

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
			Source::URL(url) => write!(fmt, "url: {url:?}"),
			Source::Github(gh) => write!(fmt, "github: {gh}"),
		}
	}
}

#[cfg(test)]
mod tests_source {
	use super::*;
	use std::{env::temp_dir, fs::File};
	use wasm_loader::NodeEndpoint;

	#[test]
	fn it_converts_from_chain_ws() {
		let urls = vec!["ws://localhost:9933", "wss://localhost:9933"];

		for url in urls {
			let src = Source::try_from(url).expect("Failing parsing source");
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
	fn it_converts_from_chain_http() {
		let urls = vec![
			"http://localhost:9933",
			"https://localhost:9933",
			"https://1rpc.io:443/astr",
			"https://astar.api.onfinality.io:443/public",
			"https://astar.public.blastapi.io:443",
			"https://evm.astar.network:443",
			"https://evm.shibuya.astar.network:443",
			"https://evm.shiden.astar.network:443",
			"https://http-versi-rpc-node-0.parity-versi.parity.io:443",
			"https://http-wococo-pos-rpc-node-0.parity-testnet.parity.io:443",
			"https://shibuya-rpc.dwellir.com:443",
			"https://shibuya.api.onfinality.io:443/public",
			"https://shibuya.public.blastapi.io:443",
			"https://shiden-rpc.dwellir.com:443",
			"https://shiden.api.onfinality.io:443/public",
			"https://shiden.public.blastapi.io:443",
			"https://www.alchemy.com:443/astar];",
		];

		for url in urls {
			let src = Source::try_from(url).expect("Failing parsing source");
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
	fn it_converts_from_alias() {
		let names = vec!["polkadot", "dot"];

		for name in names {
			assert!(matches!(Source::try_from(name).expect("Failing parsing source"), Source::Alias(_)));
		}
	}

	#[test]
	fn it_converts_from_url() {
		let urls = vec!["https://github.com/paritytech/polkadot/releases/download/v0.9.42/kusama_runtime-v9420.compact.compressed.wasm"];

		for url in urls {
			let src = Source::try_from(url).expect("Failing parsing source");
			assert!(matches!(src, Source::URL(_)));
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
			assert_eq!(
				Source::try_from(file.as_str()).expect("Failing parsing source"),
				Source::File(PathBuf::from(file))
			);
		}
	}

	#[test]
	fn it_catches_unknown() {
		let v = vec!["foo", "bar"];

		for value in v {
			assert!(Source::try_from(value).is_err());
		}
	}
}

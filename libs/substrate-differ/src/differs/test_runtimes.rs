#![cfg(test)]

use std::{env, fmt::Display, path::PathBuf, str::FromStr};

use crate::error::SubstrateDifferError;
pub const RUNTIME_V12: &str = "../../data/runtime_v12.wasm";

pub const RUNTIME_V13: &str = "../../data/runtime_v13.wasm";
pub const RUNTIME_V13_1: &str = "../../data/kusama/V13/kusama-9030.wasm";
pub const RUNTIME_V13_2: &str = "../../data/kusama/V13/kusama-9080.wasm";

pub const RUNTIME_V14: &str = "../../data/polkadot/V14/polkadot_runtime.compact.compressed.wasm";
pub const RUNTIME_V14_9100: &str = "../../data/polkadot/V14/9100.wasm";
pub const RUNTIME_POLKADOT_V14_9260: &str = "../../data/polkadot/V14/9260.wasm";
pub const RUNTIME_POLKADOT_V14_9270: &str = "../../data/polkadot/V14/9270.wasm";
pub const RUNTIME_POLKADOT_V14_9280: &str = "../../data/polkadot/V14/9280.wasm";
pub const RUNTIME_POLKADOT_V14_9290: &str = "../../data/polkadot/V14/9290.wasm";
pub const RUNTIME_STATEMINE_V14_9290: &str = "../../data/statemine/V14/9290.wasm";

#[derive(Debug, Clone)]
/// List of the runtime supported in our tests
pub enum Chain {
	Westend,
	AssetHubWestend,

	Kusama,
	AssetHubKusama,

	Polkadot,
	AssetHubPolkadot,

	CollectivesPolkadot,
}

impl Display for Chain {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Chain::Westend => write!(f, "Westend"),
			Chain::AssetHubWestend => write!(f, "Asset Hub Westend"),

			Chain::Kusama => write!(f, "Kusama"),
			Chain::AssetHubKusama => write!(f, "Asset Hub Kusama"),

			Chain::Polkadot => write!(f, "Polkadot"),
			Chain::AssetHubPolkadot => write!(f, "Asset Hub Polkadot"),

			Chain::CollectivesPolkadot => write!(f, "collectives-polkadot"),
		}
	}
}

/// Helper to easily fetch runtimes locally
// TODO: It would speed up the tests to add caching here so we avoid reloading the runtimes over and over again
#[derive(Debug, Clone)]
pub struct RuntimeFile {
	chain: Chain,
	metadata_version: u8,
	spec_version: usize,
}

impl RuntimeFile {
	pub fn new(chain: Chain, metadata_version: u8, spec_version: usize) -> Self {
		Self { chain, metadata_version, spec_version }
	}
}

impl TryInto<PathBuf> for RuntimeFile {
	type Error = SubstrateDifferError;

	/// Try fetching a runtime locally from disk
	fn try_into(self) -> Result<PathBuf, Self::Error> {
		let workspace_root = env::var("CARGO_WORKSPACE_DIR").expect("Failed getting env");
		let candidate = PathBuf::from_str(&format!(
			"{workspace_root}data/{chain}/V{meta}/{spec}.wasm",
			chain = self.chain,
			meta = self.metadata_version,
			spec = self.spec_version
		))
		.expect("Should be infallible");

		if candidate.exists() {
			return Ok(candidate);
		}
		Err(SubstrateDifferError::RuntimeNotFound(candidate))
	}
}

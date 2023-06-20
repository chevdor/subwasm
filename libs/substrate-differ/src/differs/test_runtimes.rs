#![cfg(test)]

use std::{env, fmt::Display, path::PathBuf, str::FromStr};
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

pub enum Chain {
	Statemine,
	Statemint,
	Westmint,
	Polkadot,
	Kusama,
	Westend,
	CollectivesPolkadot,
}

impl Display for Chain {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Chain::Statemine => write!(f, "statemine"),
			Chain::Statemint => write!(f, "statemint"),
			Chain::Westmint => write!(f, "westmint"),
			Chain::Polkadot => write!(f, "polkadot"),
			Chain::Kusama => write!(f, "kusama"),
			Chain::Westend => write!(f, "westend"),
			Chain::CollectivesPolkadot => write!(f, "collectives-polkadot"),
		}
	}
}

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

pub fn get_runtime_file(runtime_file: RuntimeFile) -> Option<PathBuf> {
	let workspace_root = env::var("CARGO_WORKSPACE_DIR").expect("Failed getting env");
	let candidate = PathBuf::from_str(&format!(
		"{workspace_root}data/{chain}/V{meta}/{spec}.wasm",
		chain = runtime_file.chain,
		meta = runtime_file.metadata_version,
		spec = runtime_file.spec_version
	))
	.ok();
	if let Some(c) = candidate {
		if c.exists() {
			return Some(c);
		}
	}

	None
}

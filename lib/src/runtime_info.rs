use crate::error::*;
use ipfs_hasher::IpfsHasher;
use num_format::{Locale, ToFormattedString};
use serde::Serialize;
use sp_version::RuntimeVersion as SubstrateRuntimeVersion;
use std::fmt::Display;
use wasm_loader::Compression;
use wasm_testbed::{ReservedMeta, WasmTestBed};

/// Describe the summary information of a runtime.
///
#[derive(Debug, Serialize)]
pub struct RuntimeInfo {
	/// Size of the runtime, in bytes. Runtimes can be compressed or not. They are
	/// usually compressed and the size will then returned as "store on disk".
	size: usize,

	/// Whether the runtime is compressed or not
	compression: Compression,

	/// The value of this field should never change.
	/// It means `meta` in hex :)
	reserved_meta: ReservedMeta,

	/// Whether the [reserved_meta] is valid for a Substrate runtime
	reserved_meta_valid: bool,

	/// Knowing the metadata version is required to properly decode the metadata
	metadata_version: u8,

	/// This is the core version of the runtime as reported by the runtimes
	core_version: SubstrateRuntimeVersion,

	/// The proposal hash is the hash of the extrinsic as it will appear
	/// on-chain when calling `System.setCode(<runtime>)`
	proposal_hash: String,

	/// This is the hash of the extrinsic to authorize a parachain upgrade
	parachain_authorize_upgrade_hash: String,

	/// This is the IPFS hash of the runtime. That does **not** guaranty the
	/// runtime to be seeded, but if it is, you can fetch it with this hash
	ipfs_hash: String,

	/// The blake2_256 hash of the runtime
	blake2_256: String,
}

impl RuntimeInfo {
	pub fn new(testbed: &WasmTestBed) -> Result<Self> {
		let core_version = testbed.core_version();
		let hasher = IpfsHasher::default();
		let proposal_hash = testbed.proposal_hash()?;
		let blake2_256 = testbed.blake2_256_hash()?;
		let parachain_authorize_upgrade_hash = testbed.parachain_authorize_upgrade_hash()?;
		let ipfs_hash = hasher.compute(testbed.raw_bytes())?;

		Ok(Self {
			size: testbed.size(),
			compression: testbed.compression(),
			reserved_meta: testbed.reserved_meta(),
			reserved_meta_valid: testbed.reserved_meta_valid(),
			metadata_version: *testbed.metadata_version(),
			core_version,
			proposal_hash,
			parachain_authorize_upgrade_hash,
			ipfs_hash,
			blake2_256,
		})
	}

	/// Print the RuntimeInfo either using the Display impl
	/// or serde as json.
	pub fn print(&self, json: bool) -> Result<()> {
		if json {
			let serialized = serde_json::to_string_pretty(self)?;
			println!("{serialized}");
		} else {
			println!("{self}");
		}
		Ok(())
	}

	pub fn print_version(&self, json: bool) -> Result<()> {
		if json {
			let serialized = serde_json::to_string_pretty(&self.core_version)?;
			println!("{serialized}");
		} else {
			println!("specifications : {} v{}", self.core_version.spec_name, self.core_version.spec_version);
			println!("implementation : {} v{}", self.core_version.impl_name, self.core_version.impl_version);
			println!("transaction    : v{}", self.core_version.transaction_version);
			println!("authoring      : v{}", self.core_version.authoring_version);
		}
		Ok(())
	}
}

impl Display for RuntimeInfo {
	fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let size_mb: f64 = self.size as f64 / 1024.0 / 1024.0;
		let width_emoji = 1;
		let width_title = 25;
		const MAX_SIZE_COMPRESSED: f64 = 2_f64;

		writeln!(
			fmt,
			"{:<width_emoji$} {:<width_title$} {:.3?} MB ({} bytes) {warning}",
			"🏋️ ",
			"Runtime size:",
			size_mb,
			self.size.to_formatted_string(&Locale::en),
			warning = if size_mb >= MAX_SIZE_COMPRESSED { "⚠️ HEAVY" } else { "" }
		)?;
		if self.compression.compressed() {
			writeln!(
				fmt,
				"{:<width_emoji$} {:<width_title$} Yes, {:.2}%",
				"🗜 ",
				"Compressed:",
				100f32 - self.compression.compression_ratio() * 100f32
			)?;
		} else {
			writeln!(fmt, "{:<width_emoji$} {:<width_title$} No", "🗜", "Compressed:")?;
		}

		writeln!(
			fmt,
			"{:<width_emoji$} {:<width_title$} {} - {:02X?}",
			"✨",
			"Reserved meta:",
			if self.reserved_meta_valid { "OK" } else { "Unknown!" },
			self.reserved_meta,
		)?;
		writeln!(fmt, "{:<width_emoji$} {:<width_title$} V{:?}", "🎁", "Metadata version:", self.metadata_version)?;
		writeln!(fmt, "{:<width_emoji$} {:<width_title$} {}", "🔥", "Core version:", self.core_version)?;
		writeln!(fmt, "{:<width_emoji$} {:<width_title$} {}", "🗳️ ", "system.setCode hash:", self.proposal_hash)?;
		writeln!(
			fmt,
			"{:<width_emoji$} {:<width_title$} {}",
			"🗳️ ", "authorizeUpgrade hash:", self.parachain_authorize_upgrade_hash
		)?;
		writeln!(fmt, "{:<width_emoji$} {:<width_title$} {}", "🗳️ ", "Blake2-256 hash:", self.blake2_256)?;
		let ipfs_url = format!("https://cf-ipfs.com/ipfs/{cid}", cid = self.ipfs_hash);
		writeln!(fmt, "{:<width_emoji$} {:<width_title$} {ipfs_url}", "📦", "IPFS:")?;
		Ok(())
	}
}

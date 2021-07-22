use ipfs_hasher::IpfsHasher;
use num_format::{Locale, ToFormattedString};
use serde::Serialize;
use std::fmt::Display;
use wasm_loader::Compression;
use wasm_testbed::{ReservedMeta, WasmTestBed};

#[derive(Debug, Serialize)]
pub struct RuntimeInfo {
	size: usize,
	compression: Compression,
	reserved_meta: ReservedMeta,
	reserved_meta_valid: bool,
	metadata_version: u8,
	core_version: String,
	proposal_hash: String,
	parachain_authorize_upgrade_hash: String,
	ipfs_hash: String,
	blake2_256: String,
}

impl RuntimeInfo {
	pub fn new(testbed: &WasmTestBed) -> Self {
		let core_version = match testbed.core_version() {
			Some(v) => v.to_string(),
			None => String::from("n/a"),
		};

		let hasher = IpfsHasher::default();

		Self {
			size: testbed.size(),
			compression: testbed.compression(),
			reserved_meta: testbed.reserved_meta(),
			reserved_meta_valid: testbed.reserved_meta_valid(),
			metadata_version: *testbed.metadata_version(),
			core_version,
			proposal_hash: testbed.proposal_hash(),
			parachain_authorize_upgrade_hash: testbed.parachain_authorize_upgrade_hash(),
			ipfs_hash: hasher.compute(testbed.raw_bytes()),
			blake2_256: testbed.blake2_256_hash(),
		}
	}

	/// Print the RuntimeInfo either using the Display impl
	/// or serde as json.
	pub fn print(&self, json: bool) {
		if json {
			let serialized = serde_json::to_string_pretty(self).unwrap();
			println!("{}", serialized);
		} else {
			println!("{}", self);
		}
	}
}

impl Display for RuntimeInfo {
	fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let size_mb: f64 = self.size as f64 / 1024.0 / 1024.0;

		writeln!(fmt, "🏋️  Runtime size:\t\t{:.3?} MB ({} bytes)", size_mb, self.size.to_formatted_string(&Locale::en))?;
		if self.compression.compressed() {
			writeln!(fmt, "🗜  Compressed:\t\t\tYes, {:.2}%", 100f32 - self.compression.compression_ratio() * 100f32)?;
		} else {
			writeln!(fmt, "🗜  Compressed:\t\t\tNo")?;
		}

		writeln!(
			fmt,
			"✨ Reserved meta:\t\t{} - {:02X?}",
			if self.reserved_meta_valid { "OK" } else { "Unknown!" },
			self.reserved_meta,
		)?;
		writeln!(fmt, "🎁 Metadata version:\t\tV{:?}", self.metadata_version)?;
		writeln!(fmt, "🔥 Core version:\t\t{}", self.core_version)?;
		writeln!(fmt, "🗳️  system.setCode hash:\t\t{}", self.proposal_hash)?;
		writeln!(fmt, "🗳️  authorizedUpgrade hash:\t{}", self.parachain_authorize_upgrade_hash)?;
		writeln!(fmt, "#️⃣  Blake2-256 hash:\t\t{}", self.blake2_256)?;
		let ipfs_url = format!("https://www.ipfs.io/ipfs/{cid}", cid = self.ipfs_hash);
		writeln!(fmt, "📦 IPFS hash:\t\t\t{} ({url})", self.ipfs_hash, url = ipfs_url)?;
		Ok(())
	}
}

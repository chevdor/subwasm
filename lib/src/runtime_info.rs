use std::fmt::Display;

use num_format::{Locale, ToFormattedString};
use serde::Serialize;
use wasm_testbed::{ReservedMeta, WasmTestBed};

#[derive(Debug, Serialize)]
pub struct RuntimeInfo {
	size: usize,
	reserved_meta: ReservedMeta,
	reserved_meta_valid: bool,
	metadata_version: u8,
	core_version: String,
	proposal_hash: String,
}

impl RuntimeInfo {
	pub fn new(testbed: &WasmTestBed) -> Self {
		let core_version = match testbed.core_version() {
			Some(v) => v.to_string(),
			None => String::from("n/a"),
		};

		Self {
			size: testbed.size(),
			reserved_meta: testbed.reserved_meta(),
			reserved_meta_valid: testbed.reserved_meta_valid(),
			metadata_version: *testbed.metadata_version(),
			core_version,
			proposal_hash: testbed.proposal_hash(),
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
		writeln!(fmt, "🏋️  Runtime Size:\t{:.3?} MB ({} bytes)", size_mb, self.size.to_formatted_string(&Locale::en))?;
		writeln!(
			fmt,
			"✨ Reserved meta:\t{} - {:02X?}",
			if self.reserved_meta_valid { "OK" } else { "Unknown!" },
			self.reserved_meta,
		)?;
		writeln!(fmt, "🎁 Metadata version:\tV{:?}", self.metadata_version)?;
		writeln!(fmt, "🔥 Core version:\t{}", self.core_version)?;
		writeln!(fmt, "🗳️  Proposal hash:\t{}", self.proposal_hash)
	}
}

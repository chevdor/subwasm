use std::fmt::Display;

use super::prelude::*;
use serde::Serialize;

/// Reduced Storage
#[derive(Debug, PartialEq, Eq, Serialize, Hash)]
pub struct Storage {
	pub name: String,
	// Brought back down to a String to allow new runtimes adding more variants
	// modifier: String,
	// TODO: Check how to handle the following
	// ty: String,
	// Here we don't really care about the default value but its hash
	// TODO
	pub default_value_hash: Hash,
	pub docs: Documentation,
}

impl Display for Storage {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let _ = f.write_fmt(format_args!("{}", self.name));

		Ok(())
	}
}

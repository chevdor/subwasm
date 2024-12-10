use super::prelude::*;
use comparable::Comparable;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Reduced Storage
#[derive(Debug, PartialEq, Deserialize, Serialize, Hash, Comparable, PartialOrd, Ord, Eq, Clone)]
#[self_describing]
pub struct Storage {
	pub name: String,
	// String to allow new runtimes adding more variants
	pub modifier: String,

	// Excluding the storage type for now as a single id change in the registry leads to detected diffs
	// pub ty: String,
	pub default_value: Value,

	#[comparable_ignore]
	pub docs: Documentation,
}

impl Storage {
	pub fn new(
		name: &str,
		modifier: String,
		// ty: String,
		default_value: Vec<u8>,
		docs: Documentation,
	) -> Self {
		let name = name.into();
		Self {
			name,
			modifier,
			// ty,
			default_value,
			docs,
		}
	}
}

impl Display for Storage {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let displayable_value = DisplayableVec::new(&self.default_value, None).init().to_short_string();
		f.write_fmt(format_args!("{:<8} {}: {}", self.modifier, self.name, displayable_value))
	}
}

impl Display for StorageChange {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("--- STOR {self:?}"))
	}
}

// pub fn print_storage_changes(changes: &Vec<StorageChange>) {
// 	println!("storage change start");
// 	for c in changes {
// 		println!("{c}");
// 	}
// }

#[cfg(test)]
mod test_reduced_storage {
	use super::*;

	#[test]
	fn test_storage() {
		let s = Storage::new(
			"transfer",
			"pub".to_string(),
			// "String".to_string(),
			vec![12, 42],
			vec![],
		);
		println!("s = {s:?}");
		assert_eq!([12, 42], s.default_value.as_slice());
	}
}

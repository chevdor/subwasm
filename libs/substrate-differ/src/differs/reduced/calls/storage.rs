use super::prelude::*;
use comparable::Comparable;
use serde::Serialize;
use std::fmt::Display;

/// Reduced Storage
#[derive(Debug, PartialEq, Serialize, Hash, Comparable, PartialOrd, Ord, Eq, Clone)]
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

	/// Lots of the default values are arrays of zeroes. This helper shows those long
	/// arrays in a compressed form more appropriate to display.
	fn format_compress_vec(&self) -> String {
		const LIMIT: usize = 1;
		if self.default_value.len() > LIMIT && self.default_value.iter().all(|x| x == &0) {
			return format!("[0; {}]", self.default_value.len());
		}
		format!("{:?}", self.default_value)
	}
}

impl Display for Storage {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("mod:{} name:{} value:{}", self.modifier, self.name, self.format_compress_vec()))
	}
}

#[cfg(test)]
mod test_reduced_storage {
	use super::*;

	#[test]
	fn test_storage() {
		let s = Storage::new("transfer", "pub".to_string(), "String".to_string(), vec![12, 42], vec![]);
		println!("s = {s:?}");
		assert_eq!([12, 42], s.default_value.as_slice());
	}
}

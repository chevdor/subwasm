use super::prelude::*;
use comparable::Comparable;
use serde::Serialize;
use std::fmt::Display;

/// Reduced Storage
#[derive(Debug, PartialEq, Eq, Serialize, Hash, Comparable, PartialOrd, Ord)]
pub struct Storage {
	pub name: String,
	// Brought back down to a String to allow new runtimes adding more variants
	// modifier: String,
	// TODO: Check how to handle the following
	// ty: String,
	// Here we don't really care about the default value but its hash
	// TODO
	pub default_value: Value,

	#[comparable_ignore]
	pub docs: Documentation,
}

impl Storage {
	pub fn new(name: &str, default_value: Vec<u8>, docs: Documentation) -> Self {
		let name = name.into();
		Self { name, default_value, docs }
	}
}

impl Display for Storage {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let _ = f.write_fmt(format_args!("{}", self.name));

		Ok(())
	}
}

#[cfg(test)]
mod test_reduced_storage {
	use super::*;

	#[test]
	fn test_storage() {
		let s = Storage::new("transfer", vec![12, 42], vec![]);
		println!("s = {:?}", s);
		assert_eq!([12, 42], s.default_value.as_slice());
	}
}

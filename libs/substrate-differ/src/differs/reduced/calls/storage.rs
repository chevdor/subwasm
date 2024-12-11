use super::hashed_type::*;
use super::prelude::*;
use comparable::{Changed, Comparable};
use frame_metadata::v14::{StorageEntryType, StorageHasher};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Reduced Storage
#[derive(Debug, PartialEq, Deserialize, Serialize, Hash, Comparable, PartialOrd, Ord, Eq, Clone)]
#[self_describing]
pub struct Storage {
	pub name: String,
	// String to allow new runtimes adding more variants
	pub modifier: String,

	// The Storage type (Plain, or Map).  It includes the hashers for the map.
	pub ty: StorageType,

	pub default_value: Value,

	#[comparable_ignore]
	pub docs: Documentation,
}

impl Storage {
	pub fn new(name: &str, modifier: String, ty: StorageType, default_value: Vec<u8>, docs: Documentation) -> Self {
		let name = name.into();
		Self { name, modifier, ty, default_value, docs }
	}
}

impl Display for Storage {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let displayable_value = DisplayableVec::new(&self.default_value, None).init().to_short_string();
		f.write_fmt(format_args!("{}: {} ({:<8} {})", self.name, self.ty, self.modifier, displayable_value))
	}
}

impl Display for StorageChange {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			StorageChange::Name(name) => f.write_fmt(format_args!("Name changed: {} -> {}", name.0, name.1)),
			StorageChange::Modifier(modifier) => {
				f.write_fmt(format_args!("Modifier changed: {} -> {}", modifier.0, modifier.1))
			}
			StorageChange::Ty(ty) => f.write_fmt(format_args!("Type changed: {ty}")),
			StorageChange::DefaultValue(value) => f.write_fmt(format_args!("Default value changed: {value:?}")),
		}
	}
}

// pub fn print_storage_changes(changes: &Vec<StorageChange>) {
// 	println!("storage change start");
// 	for c in changes {
// 		println!("{c}");
// 	}
// }

#[derive(Debug, PartialEq, Serialize, Deserialize, Hash, Comparable, PartialOrd, Ord, Eq, Clone)]
#[self_describing]
pub enum StorageType {
	Plain(HashedType),
	Map { hashers: Vec<String>, key: HashedType, value: HashedType },
}

impl StorageType {
	pub fn from_v14_metadata(registry: &PortableRegistry, entry: &StorageEntryType<PortableForm>) -> Self {
		match entry {
			StorageEntryType::Plain(ty) => {
				let ty = resolve_type(registry, ty.id, None);
				StorageType::Plain(ty)
			}
			StorageEntryType::Map { hashers, key, value } => {
				let hashers = hashers
					.iter()
					.map(|h| match h {
						StorageHasher::Blake2_128 => "blake2_128".to_string(),
						StorageHasher::Blake2_256 => "blake2_256".to_string(),
						StorageHasher::Blake2_128Concat => "blake2_128_concat".to_string(),
						StorageHasher::Twox128 => "twox_128".to_string(),
						StorageHasher::Twox256 => "twox_256".to_string(),
						StorageHasher::Twox64Concat => "twox_64_concat".to_string(),
						StorageHasher::Identity => "identity".to_string(),
					})
					.collect();
				let key = resolve_type(registry, key.id, None);
				let value = resolve_type(registry, value.id, None);
				StorageType::Map { hashers, key, value }
			}
		}
	}
}

impl Display for StorageType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			StorageType::Plain(ty) => f.write_fmt(format_args!("{}", ty)),
			StorageType::Map { hashers, key, value } => {
				let hashers = hashers.join(", ");
				f.write_fmt(format_args!("(map<{}>: key: {} => value: {})", hashers, key, value))
			}
		}
	}
}

impl Display for StorageTypeChange {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			StorageTypeChange::BothPlain(ty) => f.write_fmt(format_args!("Plain: {ty}"))?,
			StorageTypeChange::BothMap { hashers, key, value } => {
				if let Changed::Changed(hashers) = hashers {
					f.write_fmt(format_args!("Hashers changed: {hashers:?}"))?;
				}
				if let Changed::Changed(key) = key {
					f.write_fmt(format_args!("Key changed: {key}"))?;
				}
				if let Changed::Changed(value) = value {
					f.write_fmt(format_args!("Value changed: {value}"))?;
				}
			}
			StorageTypeChange::Different(old, new) => f.write_fmt(format_args!("Old: {old}, New: {new}"))?,
		}
		Ok(())
	}
}

#[cfg(test)]
mod test_reduced_storage {
	use super::*;

	#[test]
	fn test_storage() {
		let s = Storage::new("transfer", "pub".to_string(), StorageType::Plain("u32".into()), vec![12, 42], vec![]);
		println!("s = {s:?}");
		assert_eq!([12, 42], s.default_value.as_slice());
	}
}

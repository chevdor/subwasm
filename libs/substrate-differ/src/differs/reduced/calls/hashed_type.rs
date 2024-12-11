use super::prelude::*;
use comparable::Comparable;
use scale_info::{TypeDef, TypeDefPrimitive};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, PartialEq, Serialize, Deserialize, Hash, Comparable, PartialOrd, Ord, Eq, Clone)]
#[self_describing]
pub struct HashedType {
	pub ty: String,
	#[comparable_ignore]
	pub hashed: String,
}

impl From<&str> for HashedType {
	fn from(ty: &str) -> Self {
		Self { ty: ty.into(), hashed: "".into() }
	}
}

impl std::fmt::Display for HashedType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("{}", self.ty))
	}
}

impl std::fmt::Display for HashedTypeChange {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("{} -> {}", self.ty.0, self.ty.1))
	}
}

fn hash_type_impl(registry: &PortableRegistry, id: u32, hasher: &mut blake3::Hasher, seen: &mut HashSet<u32>) {
	if seen.contains(&id) {
		// Avoid infinite recursion.
		hasher.update(b"Recursion");
		return;
	}
	seen.insert(id);
	let ty = if let Some(ty) = registry.resolve(id) {
		if ty.path.is_empty() {
			log::trace!("Hashing type: {:?}", ty.type_def);
		} else {
			log::trace!("Hashing type: {}", ty.path);
		}
		ty
	} else {
		// This shouldn't happen, since the `id` should be in the registry.
		log::error!("Missing type id: {}", id);
		// Just hash the `id` as a fallback.
		hasher.update(format!("Unknown_{id}").as_bytes());
		return;
	};

	match &ty.type_def {
		TypeDef::Composite(def) => {
			hasher.update(b"Composite");
			for field in &def.fields {
				hash_type_impl(registry, field.ty.id, hasher, seen);
			}
		}
		TypeDef::Variant(def) => {
			hasher.update(b"Variant");
			for (idx, variant) in def.variants.iter().enumerate() {
				hasher.update(&idx.to_le_bytes());
				for field in &variant.fields {
					hash_type_impl(registry, field.ty.id, hasher, seen);
				}
			}
		}
		TypeDef::Sequence(def) => {
			hasher.update(b"Sequence");
			hash_type_impl(registry, def.type_param.id, hasher, seen);
		}
		TypeDef::Array(def) => {
			hasher.update(b"Array");
			hash_type_impl(registry, def.type_param.id, hasher, seen);
			hasher.update(&def.len.to_le_bytes());
		}
		TypeDef::Tuple(def) => {
			hasher.update(b"Tuple");
			for field in &def.fields {
				hash_type_impl(registry, field.id, hasher, seen);
			}
		}
		TypeDef::Primitive(def) => {
			hasher.update(b"Primitive");
			hasher.update(format!("{:?}", def).as_bytes());
		}
		TypeDef::Compact(def) => {
			hasher.update(b"Compact");
			hash_type_impl(registry, def.type_param.id, hasher, seen);
		}
		TypeDef::BitSequence(def) => {
			hasher.update(b"BitSequence");
			hash_type_impl(registry, def.bit_store_type.id, hasher, seen);
			hash_type_impl(registry, def.bit_order_type.id, hasher, seen);
		}
	}
}

/// Recursively generate a hash of the SCALE type.
///
/// Only the types shape is hashed, not the name or fields names.
///
/// A types `TypeDef` is hashed as follows:
/// * Composite (structs) only the hash of the field types are hashed.
/// * Variant (enums) only the hash of the variant types are hashed.
/// * Sequence (Vec) only the hash of the element type is hashed.
/// * Array only the hash of the element type and the length is hashed.
/// * Tuple only the hash of the tuple elements are hashed.
/// * Primitive the hash of the primitive type is hashed.
/// * Compact only the hash of the compact type is hashed.
///
/// The `TypeDef` variant name is also included in the hashed data.
///
/// If the type isn't found in the registry then hash `Unknown_{id}`.
pub fn hash_type(registry: &PortableRegistry, id: u32) -> String {
	let mut seen = HashSet::new();
	let mut hasher = blake3::Hasher::new();
	hash_type_impl(registry, id, &mut hasher, &mut seen);
	hasher.finalize().to_hex().to_string()
}

fn resolve_type_impl(registry: &PortableRegistry, id: u32) -> Option<String> {
	let ty = registry.resolve(id)?;
	let full_name = ty.path.to_string();
	if full_name.is_empty() {
		// If `path` is empty we need to build the type name manually (i.e. for sequences Vec<T>, arrays [T; n], tuples (T1, T2,), etc..).
		match &ty.type_def {
			TypeDef::Array(def) => {
				let ty = resolve_type_impl(registry, def.type_param.id)?;
				Some(format!("[{}; {}]", ty, def.len))
			}
			TypeDef::Sequence(def) => {
				let ty = resolve_type_impl(registry, def.type_param.id)?;
				Some(format!("Vec<{}>", ty))
			}
			TypeDef::Compact(def) => {
				let ty = resolve_type_impl(registry, def.type_param.id)?;
				Some(format!("Compact<{}>", ty))
			}
			TypeDef::Tuple(def) => {
				let fields =
					def.fields.iter().map(|f| resolve_type_impl(registry, f.id)).collect::<Option<Vec<String>>>()?;
				Some(format!("({})", fields.join(", ")))
			}
			TypeDef::Primitive(def) => Some(
				match def {
					TypeDefPrimitive::Bool => "bool",
					TypeDefPrimitive::Char => "char",
					TypeDefPrimitive::Str => "String",
					TypeDefPrimitive::U8 => "u8",
					TypeDefPrimitive::U16 => "u16",
					TypeDefPrimitive::U32 => "u32",
					TypeDefPrimitive::U64 => "u64",
					TypeDefPrimitive::U128 => "u128",
					TypeDefPrimitive::U256 => "u256",
					TypeDefPrimitive::I8 => "i8",
					TypeDefPrimitive::I16 => "i16",
					TypeDefPrimitive::I32 => "i32",
					TypeDefPrimitive::I64 => "i64",
					TypeDefPrimitive::I128 => "i128",
					TypeDefPrimitive::I256 => "i256",
				}
				.to_string(),
			),
			TypeDef::BitSequence(def) => {
				let bit_store = resolve_type_impl(registry, def.bit_store_type.id)?;
				let bit_order = resolve_type_impl(registry, def.bit_order_type.id)?;
				Some(format!("BitSequence<{}, {}>", bit_store, bit_order))
			}
			_ => {
				log::trace!("This type should have a path or ident name: {:?}", ty);
				None
			}
		}
	} else if ty.type_params.is_empty() {
		// Simple type name (not a generic).
		Some(full_name)
	} else {
		// Resolve the type parameters.
		let params = ty
			.type_params
			.iter()
			.map(|p| p.ty.and_then(|ty| resolve_type_impl(registry, ty.id)).unwrap_or_else(|| p.name.to_string()))
			.collect::<Vec<_>>();
		Some(format!("{full_name}<{}>", params.join(", ")))
	}
}

/// Try to resolve a type to it's actual type name.
pub fn resolve_type(registry: &PortableRegistry, id: u32, fallback: Option<&str>) -> HashedType {
	let ty = resolve_type_impl(registry, id)
		.unwrap_or_else(|| fallback.map(|s| s.to_string()).unwrap_or_else(|| format!("Unknown_{id}")));
	HashedType { ty, hashed: hash_type(registry, id) }
}

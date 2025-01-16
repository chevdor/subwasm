use super::prelude::*;
use comparable::{Changed, Comparable, StringChange};
use scale_info::{Type, TypeDef, TypeDefPrimitive};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, sync::Arc};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct HashedType {
	pub ty: String,
	pub hashed: String,
	#[serde(skip)]
	pub registry_ty: Option<(u32, Arc<PortableRegistry>)>,
}

impl HashedType {
	/// Split the type name into a tuple of the path and the ident.
	pub fn path_and_ident(&self) -> (String, String) {
		let (name, generics) = match self.ty.split_once('<') {
			Some((name, generics)) => (name, format!("<{}", generics)),
			None => (self.ty.as_str(), "".to_string()),
		};
		let n = name.rsplit_once("::");
		match n {
			Some((prefix, ident)) => (prefix.to_string(), format!("{}{}", ident, generics)),
			None => ("".to_string(), self.ty.clone()),
		}
	}

	/// Return only the ident part of the type name.
	pub fn ident(&self) -> String {
		self.path_and_ident().1
	}

	pub fn get_type_ref(&self) -> Option<TypeRef> {
		self.registry_ty.as_ref().and_then(|(id, registry)| TypeRef::new(*id, registry).ok())
	}
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum HashedTypeChange {
	NameChanged(StringChange),
	HashChanged(StringChange),
	TypeChanged(String),
	NameAndHashChanged(StringChange, StringChange),
}

impl Comparable for HashedType {
	type Desc = HashedType;

	fn describe(&self) -> Self::Desc {
		self.clone()
	}

	type Change = HashedTypeChange;

	fn comparison(&self, other: &Self) -> Changed<Self::Change> {
		let name = match self.ty.comparison(&other.ty) {
			Changed::Unchanged => None,
			Changed::Changed(change) => Some(change),
		};
		// If the hash is the same then the type is the same.
		let (hashed, ty) = match self.hashed.comparison(&other.hashed) {
			Changed::Unchanged => (None, None),
			Changed::Changed(hashed) => {
				// The hash changed, check if the type is compatible.
				match (self.get_type_ref(), other.get_type_ref()) {
					(Some(old), Some(new)) => {
						// Check if the types are compatible.
						match compatible_types(&old, &new) {
							// Ignore hash changes for compatible types.
							Ok(()) => (None, None),
							// Display the type change if the types are incompatible.
							Err(err) => (None, Some(err)),
						}
					}
					// If the type references are missing then just show the hash change.
					_ => (Some(hashed), None),
				}
			}
		};
		match (name, hashed, ty) {
			// Nothing changed.
			(None, None, None) => Changed::Unchanged,
			// If the type changed, then only that matters.
			(_, _, Some(ty)) => Changed::Changed(HashedTypeChange::TypeChanged(ty)),
			// Only the name changed.
			(Some(name), None, None) => Changed::Changed(HashedTypeChange::NameChanged(name)),
			// Only the hash changed.
			(None, Some(hashed), None) => Changed::Changed(HashedTypeChange::HashChanged(hashed)),
			// Both name and hash changed.
			(Some(name), Some(hashed), None) => Changed::Changed(HashedTypeChange::NameAndHashChanged(name, hashed)),
		}
	}
}

/// Only show the full type names if the prefix is different.
pub fn type_name_changed(name: &StringChange) -> String {
	let n0 = name.0.rsplit_once("::");
	let n1 = name.1.rsplit_once("::");
	match (n0, n1) {
		(Some((prefix0, ident0)), Some((prefix1, ident1))) if prefix0 == prefix1 => {
			format!("{} -> {}", ident0, ident1)
		}
		_ => format!("{} -> {}", name.0, name.1),
	}
}

impl From<&str> for HashedType {
	fn from(ty: &str) -> Self {
		Self { ty: ty.into(), hashed: "".into(), registry_ty: None }
	}
}

impl std::fmt::Display for HashedType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("{}", self.ident()))
	}
}

impl std::fmt::Display for HashedTypeChange {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::NameChanged(name) => f.write_fmt(format_args!("Name changed: {}", type_name_changed(name))),
			Self::HashChanged(_hash) => f.write_fmt(format_args!("Hash changed")),
			Self::TypeChanged(ty) => f.write_fmt(format_args!("Type changed: {ty}")),
			Self::NameAndHashChanged(name, _hash) => {
				f.write_fmt(format_args!("Name and Hash changed: {}", type_name_changed(name)))
			}
		}
	}
}

fn hash_type_impl(registry: &Arc<PortableRegistry>, id: u32, hasher: &mut blake3::Hasher, seen: &mut HashSet<u32>) {
	if seen.contains(&id) {
		// Avoid infinite recursion.
		hasher.update(b"Recursion");
		return;
	}
	seen.insert(id);
	let ty = if let Some(ty) = registry.resolve(id) {
		ty
	} else {
		// This shouldn't happen, since the `id` should be in the registry.
		log::error!("Missing type id: {}", id);
		// Just hash the `id` as a fallback.
		hasher.update(format!("Unknown_{id}").as_bytes());
		return;
	};

	// For some top-level Substrate runtime types (like `*::runtime::RuntimeCall`), just use a fixed hash.
	// This is so that adding pallets or extrinsics doesn't change the hash of the runtime.
	if let Some(ident) = ty.path.ident() {
		match ident.as_ref() {
			"RuntimeCall" | "RuntimeEvent" => {
				hasher.update(ident.as_bytes());
				return;
			}
			_ => {}
		}
	}

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
pub fn hash_type(registry: &Arc<PortableRegistry>, id: u32) -> String {
	let mut seen = HashSet::new();
	let mut hasher = blake3::Hasher::new();
	hash_type_impl(registry, id, &mut hasher, &mut seen);
	hasher.finalize().to_hex().to_string()
}

pub struct TypeRef<'a> {
	pub id: u32,
	pub ty: &'a Type<PortableForm>,
	pub registry: &'a Arc<PortableRegistry>,
}

impl<'a> TypeRef<'a> {
	pub fn new(id: u32, registry: &'a Arc<PortableRegistry>) -> Result<Self, String> {
		let ty = registry.resolve(id).ok_or_else(|| format!("Missing type: {id}"))?;
		Ok(Self { id, ty, registry })
	}

	pub fn resolve(&self, id: u32) -> Result<Self, String> {
		Self::new(id, self.registry)
	}

	/// Get the colapsed type definition.
	///
	/// If the type is a tuple or composite with only one field then return the type definition of that field.
	/// Otherwise return the type definition of the type itself.
	///
	/// For example `OldWeight(pub u64)` is colapsed to just `u64`.
	pub fn colapsed_type_def(&self) -> &TypeDef<PortableForm> {
		match &self.ty.type_def {
			TypeDef::Composite(composite) if composite.fields.len() == 1 => {
				let id = composite.fields[0].ty.id;
				if let Some(ty) = self.registry.resolve(id) {
					&ty.type_def
				} else {
					&self.ty.type_def
				}
			}
			TypeDef::Tuple(tuple) if tuple.fields.len() == 1 => {
				let id = tuple.fields[0].id;
				if let Some(ty) = self.registry.resolve(id) {
					&ty.type_def
				} else {
					&self.ty.type_def
				}
			}
			_ => &self.ty.type_def,
		}
	}
}

/// Recursively compare two types from different registries to check if they are compatible.
///
/// This comparision is checking if the old type's SCALE encoding is compatible with the new type's SCALE encoding.
///
/// Adding a variant to an enum is not considered a breaking change, but changing the type of a field in a variant is.
///
/// Returns `Ok(())` if the types are compatible, otherwise returns an error message.
pub fn compatible_types(old: &TypeRef, new: &TypeRef) -> Result<(), String> {
	let mut seen = HashSet::new();
	compatible_types_impl(old, new, &mut seen)?;
	Ok(())
}

fn compatible_types_impl(old: &TypeRef, new: &TypeRef, seen: &mut HashSet<u32>) -> Result<(), String> {
	let id = old.id;
	if seen.contains(&id) {
		// Avoid infinite recursion.
		return Ok(());
	}
	seen.insert(id);

	// Compare type definitions.
	compatible_type_defs(old, new, seen).map_err(|err| {
		// Resolve the type name for the error message.
		let name = resolve_type_impl(old.registry, id).unwrap_or_else(|| format!("Unknown_{}", id));
		format!("{name} -> {err}")
	})?;

	Ok(())
}

fn compatible_type_defs(old_ref: &TypeRef, new_ref: &TypeRef, seen: &mut HashSet<u32>) -> Result<(), String> {
	// Get the colapsed type definition.
	let old_def = old_ref.colapsed_type_def();
	let new_def = new_ref.colapsed_type_def();

	match (old_def, new_def) {
		(TypeDef::Composite(old), TypeDef::Composite(new)) => {
			if old.fields.len() != new.fields.len() {
				return Err("Different number of fields".into());
			}
			for (old_field, new_field) in old.fields.iter().zip(new.fields.iter()) {
				compatible_types_impl(&old_ref.resolve(old_field.ty.id)?, &new_ref.resolve(new_field.ty.id)?, seen)?;
			}
		}
		(TypeDef::Variant(old), TypeDef::Variant(new)) => {
			// Allow the new enum to have more variants than the old one.
			if old.variants.len() > new.variants.len() {
				return Err("Variants removed from enum".into());
			}
			// Check if all the variants in the old enum are compatible with the new enum.
			// If the new enum has more variants than the old one, then the extra variants are ignored.
			for (old_variant, new_variant) in old.variants.iter().zip(new.variants.iter()) {
				if old_variant.fields.len() != new_variant.fields.len() {
					return Err("Different number of fields in variant".into());
				}
				for (old_field, new_field) in old_variant.fields.iter().zip(new_variant.fields.iter()) {
					compatible_types_impl(
						&old_ref.resolve(old_field.ty.id)?,
						&new_ref.resolve(new_field.ty.id)?,
						seen,
					)?;
				}
			}
		}
		(TypeDef::Sequence(old), TypeDef::Sequence(new)) => {
			compatible_types_impl(&old_ref.resolve(old.type_param.id)?, &new_ref.resolve(new.type_param.id)?, seen)?;
		}
		(TypeDef::Array(old), TypeDef::Array(new)) => {
			if old.len != new.len {
				return Err("Different array length".into());
			}
			compatible_types_impl(&old_ref.resolve(old.type_param.id)?, &new_ref.resolve(new.type_param.id)?, seen)?;
		}
		(TypeDef::Tuple(old), TypeDef::Tuple(new)) => {
			if old.fields.len() != new.fields.len() {
				return Err("Different number of tuple fields".into());
			}
			for (old_field, new_field) in old.fields.iter().zip(new.fields.iter()) {
				compatible_types_impl(&old_ref.resolve(old_field.id)?, &new_ref.resolve(new_field.id)?, seen)?;
			}
		}
		(TypeDef::Primitive(old), TypeDef::Primitive(new)) => {
			if old != new {
				return Err("Different primitive type".into());
			}
		}
		(TypeDef::Compact(old), TypeDef::Compact(new)) => {
			compatible_types_impl(&old_ref.resolve(old.type_param.id)?, &new_ref.resolve(new.type_param.id)?, seen)?;
		}
		(TypeDef::BitSequence(old), TypeDef::BitSequence(new)) => {
			compatible_types_impl(
				&old_ref.resolve(old.bit_store_type.id)?,
				&new_ref.resolve(new.bit_store_type.id)?,
				seen,
			)?;
			compatible_types_impl(
				&old_ref.resolve(old.bit_order_type.id)?,
				&new_ref.resolve(new.bit_order_type.id)?,
				seen,
			)?;
		}
		_ => {
			return Err("Different type definition".into());
		}
	}

	Ok(())
}

fn resolve_type_impl(registry: &Arc<PortableRegistry>, id: u32) -> Option<String> {
	let ty = registry.resolve(id)?;
	let full_name = ty.path.ident().unwrap_or_default();
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
pub fn resolve_type(registry: &Arc<PortableRegistry>, id: u32, fallback: Option<&str>) -> HashedType {
	let ty = resolve_type_impl(registry, id)
		.unwrap_or_else(|| fallback.map(|s| s.to_string()).unwrap_or_else(|| format!("Unknown_{id}")));
	HashedType { ty, hashed: hash_type(registry, id), registry_ty: Some((id, registry.clone())) }
}

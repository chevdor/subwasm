use super::prelude::*;
use comparable::{Comparable, VecChange};
use scale_info::{TypeDef, TypeDefPrimitive};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Hash, Comparable, PartialOrd, Ord, Eq, Clone)]
#[self_describing]
pub struct ScaleType {
	pub ty: String,
}

impl From<&str> for ScaleType {
	fn from(ty: &str) -> Self {
		Self { ty: ty.into() }
	}
}

impl std::fmt::Display for ScaleType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("{}", self.ty))
	}
}

impl std::fmt::Display for ScaleTypeChange {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("{} -> {}", self.ty.0, self.ty.1))
	}
}

/// Signature of a reduced call or event.
#[derive(Debug, PartialEq, Serialize, Deserialize, Hash, Comparable, PartialOrd, Ord, Eq, Clone)]
#[self_describing]
pub struct Signature {
	pub args: Vec<Arg>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Hash, Comparable, PartialOrd, Ord, Eq, Clone)]
#[self_describing]
pub struct Arg {
	pub name: String,
	pub ty: ScaleType,
}

impl std::fmt::Display for Signature {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut first = true;
		for arg in &self.args {
			if !first {
				f.write_str(", ")?;
			}
			first = false;
			f.write_fmt(format_args!("{arg}"))?;
		}
		Ok(())
	}
}

impl std::fmt::Display for SignatureChange {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str("Sig Changed: ")?;
		let mut first = true;
		if self.args.len() > 1 {
			f.write_str("[ ")?;
		}
		for arg in &self.args {
			if !first {
				f.write_str(", ")?;
			}
			first = false;
			match arg {
				VecChange::Added(idx, arg) => f.write_fmt(format_args!("[+] {idx}: {arg}\n"))?,
				VecChange::Removed(idx, arg) => f.write_fmt(format_args!("[-] {idx}: {arg}\n"))?,
				VecChange::Changed(idx, args) => {
					f.write_fmt(format_args!("[≠] {idx}: "))?;
					if args.len() > 1 {
						f.write_str("[ ")?;
					}
					let mut first1 = true;
					for arg in args {
						if !first1 {
							f.write_str(", ")?;
						}
						first1 = false;
						f.write_fmt(format_args!("{arg}"))?;
					}
					if args.len() > 1 {
						f.write_str(" ]")?;
					}
				}
			}
		}
		if self.args.len() > 1 {
			f.write_str(" ]")?;
		}
		Ok(())
	}
}

impl std::fmt::Display for Arg {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if self.name.is_empty() {
			f.write_fmt(format_args!("{}", self.ty))?;
		} else {
			f.write_fmt(format_args!("{}: {}", self.name, self.ty))?;
		}
		Ok(())
	}
}

impl std::fmt::Display for ArgChange {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Name(name) => f.write_fmt(format_args!("Name changed: {} -> {}", name.0, name.1))?,
			Self::Ty(ty) => f.write_fmt(format_args!("Type changed: {}", ty))?,
		}
		Ok(())
	}
}

fn registry_resolve_type_internal(registry: &PortableRegistry, id: u32) -> Option<ScaleType> {
	let ty = registry.resolve(id)?;
	let full_name = ty.path.to_string();
	let ty_name = if full_name.is_empty() {
		// If `path` is empty we need to build the type name manually (i.e. for sequences Vec<T>, arrays [T; n], tuples (T1, T2,), etc..).
		match &ty.type_def {
			TypeDef::Array(def) => {
				let ty = registry_resolve_type_internal(registry, def.type_param.id)?;
				format!("[{}; {}]", ty.ty, def.len)
			}
			TypeDef::Sequence(def) => {
				let ty = registry_resolve_type_internal(registry, def.type_param.id)?;
				format!("Vec<{}>", ty.ty)
			}
			TypeDef::Compact(def) => {
				let ty = registry_resolve_type_internal(registry, def.type_param.id)?;
				format!("Compact<{}>", ty.ty)
			}
			TypeDef::Tuple(def) => {
				let fields = def
					.fields
					.iter()
					.map(|f| registry_resolve_type_internal(registry, f.id).map(|t| t.ty))
					.collect::<Option<Vec<String>>>()?;
				format!("({})", fields.join(", "))
			}
			TypeDef::Primitive(def) => match def {
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
			_ => {
				return None;
			}
		}
	} else if ty.type_params.is_empty() {
		// Simple type name (not a generic).
		full_name
	} else {
		// Resolve the type parameters.
		let params = ty
			.type_params
			.iter()
			.map(|p| {
				p.ty.and_then(|ty| registry_resolve_type_internal(registry, ty.id).map(|t| t.ty))
					.unwrap_or_else(|| p.name.to_string())
			})
			.collect::<Vec<_>>();
		format!("{full_name}<{}>", params.join(", "))
	};
	Some(ScaleType { ty: ty_name })
}

/// Try to resolve a type to it's actual type name.
pub fn registry_resolve_type(registry: &PortableRegistry, id: u32, fallback: Option<&str>) -> ScaleType {
	registry_resolve_type_internal(registry, id).unwrap_or_else(|| {
		let ty = fallback.map(|s| s.to_string()).unwrap_or_else(|| format!("Unknown_{id}"));
		ScaleType { ty }
	})
}

pub fn fields_to_args(registry: &PortableRegistry, fields: &[Field<PortableForm>]) -> Vec<Arg> {
	fields
		.iter()
		.map(|f| {
			// Resolve the field type.  (fallback to the `type_name` if the type cannot be resolved).
			let ty = registry_resolve_type(registry, f.ty.id, f.type_name.as_deref());
			Arg { name: f.name.clone().unwrap_or_default(), ty }
		})
		.collect()
}

use std::sync::Arc;

use super::hashed_type::*;
use super::prelude::*;
use comparable::{Comparable, VecChange};
use serde::{Deserialize, Serialize};

/// Signature of a reduced call or event.
#[derive(Debug, PartialEq, Serialize, Deserialize, Comparable, Clone)]
#[self_describing]
pub struct Signature {
	pub args: Vec<Arg>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Comparable, Clone)]
#[self_describing]
pub struct Arg {
	pub name: String,
	pub ty: HashedType,
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

pub fn fields_to_args(registry: &Arc<PortableRegistry>, fields: &[Field<PortableForm>]) -> Vec<Arg> {
	fields
		.iter()
		.map(|f| {
			// Resolve the field type.  (fallback to the `type_name` if the type cannot be resolved).
			let ty = resolve_type(registry, f.ty.id, f.type_name.as_deref());
			Arg { name: f.name.clone().unwrap_or_default(), ty }
		})
		.collect()
}

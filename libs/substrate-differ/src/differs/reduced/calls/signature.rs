use comparable::{Comparable, VecChange};
use serde::{Deserialize, Serialize};

pub type ArgType = String;

/// Signature of a reduced call
#[derive(Debug, PartialEq, Serialize, Deserialize, Hash, Comparable, PartialOrd, Ord, Eq, Clone)]
#[self_describing]
pub struct Signature {
	pub args: Vec<Arg>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Hash, Comparable, PartialOrd, Ord, Eq, Clone)]
#[self_describing]
pub struct Arg {
	pub name: String,
	pub ty: ArgType,
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
			Self::Name(name) => f.write_fmt(format_args!("Name: {} -> {}", name.0, name.1))?,
			Self::Ty(ty) => f.write_fmt(format_args!("Type: {} -> {}", ty.0, ty.1))?,
		}
		Ok(())
	}
}

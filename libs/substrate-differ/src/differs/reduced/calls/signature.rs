use comparable::Comparable;
use serde::Serialize;
use std::fmt::Display;

pub type ArgType = String;

/// Signature of a reduced call
#[derive(Debug, PartialEq, Serialize, Hash, Comparable, PartialOrd, Ord, Eq, Clone)]
pub struct Signature {
	pub args: Vec<Arg>,
}

#[derive(Debug, PartialEq, Serialize, Hash, Comparable, PartialOrd, Ord, Eq, Clone)]
pub struct Arg {
	pub name: String,
	pub ty: ArgType,
}

impl Display for Signature {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.args.iter().for_each(|arg| {
			let _ = f.write_fmt(format_args!("{}: {}, ", arg.name, arg.ty));
			// let _ = f.write_fmt(format_args!("{}", arg));
		});
		let _ = f.write_str(") ");
		Ok(())
	}
}

// impl Display for SignatureChange {
// 	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// 		f.write_fmt(format_args!("SIG {self}"))
// 	}
// }

impl Display for Arg {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("{}: {}", self.name, self.ty))
	}
}

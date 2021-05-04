use std::{fmt::Display, path::PathBuf};

use crate::OnchainBlock;

/// The source of the wasm. It can come from the local file system (`File`) or from a chain (`Chain`).
#[derive(Debug, Clone)]
pub enum Source {
	File(PathBuf),
	Chain(OnchainBlock),
}

impl Display for Source {
	fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Source::File(f) => write!(fmt, "{:?}", f),
			Source::Chain(c) => write!(fmt, "{:?}", c),
		}
	}
}

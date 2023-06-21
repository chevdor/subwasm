use std::path::PathBuf;

use thiserror::Error;
pub type Result<T> = std::result::Result<T, SubstrateDifferError>;

#[derive(Error, Debug, Clone)]
pub enum SubstrateDifferError {
	#[error("Unknown error")]
	HashError(),

	#[error("SerializationError")]
	SerializationError(),

	#[error("RegistryError for {0} {1}")]
	RegistryError(String, u32),

	#[error("Could not find runtime at {0}")]
	RuntimeNotFound(PathBuf),

	#[error("Generic error: {0}")]
	Generic(String),

	#[error("Unknown error")]
	Unknown(),
}

impl From<String> for SubstrateDifferError {
	fn from(s: String) -> Self {
		Self::Generic(s)
	}
}

// impl Into<String> for SubstrateDifferError {
//     fn into(self) -> String {
//         format!("{self:?}")
//     }
// }

impl From<SubstrateDifferError> for String {
	fn from(e: SubstrateDifferError) -> Self {
		format!("{e:?}")
	}
}

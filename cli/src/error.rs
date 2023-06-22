//! Top level custom error for the `subwasm` cli.
//!

use subwasmlib::SubwasmLibError;
use thiserror::Error;

/// Result alias
pub type Result<T> = std::result::Result<T, SubwasmError>;

/// `subwasm` cli custom errors
#[derive(Error, Debug)]
pub enum SubwasmError {
	/// Error parsing the source
	#[error("SourceParseError {0}")]
	SourceParseError(String),

	/// Generic error
	#[error("Generic lib error: {0}")]
	Generic(SubwasmLibError),
}

impl From<SubwasmLibError> for SubwasmError {
	fn from(e: SubwasmLibError) -> Self {
		Self::Generic(e)
	}
}

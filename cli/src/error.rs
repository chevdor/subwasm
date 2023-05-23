use subwasmlib::SubwasmLibError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, SubwasmError>;

#[derive(Error, Debug)]
pub enum SubwasmError {
	#[error("SourceParseError {0}")]
	SourceParseError(String),

	#[error("Generic lib error: {0}")]
	Generic(SubwasmLibError),
}

impl From<SubwasmLibError> for SubwasmError {
	fn from(e: SubwasmLibError) -> Self {
		Self::Generic(e)
	}
}

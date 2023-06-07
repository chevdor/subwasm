use subwasmlib::SubwasmLibError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, SubwasmError>;

#[derive(Error, Debug)]
pub enum SubwasmError {
	#[error("You need to pass exactly 2 sources, you passed {0}")]
	WrongNumberOfSources(usize),

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

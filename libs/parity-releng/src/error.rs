use thiserror::Error;

pub type Result<T> = std::result::Result<T, ParityRelengError>;

#[derive(Error, Debug)]
pub enum ParityRelengError {
	#[error("Generic error: {0}")]
	Generic(String),

	#[error("Unknown error")]
	Unknown(),
}

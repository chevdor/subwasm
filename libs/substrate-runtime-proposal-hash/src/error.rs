use blake2::digest::{InvalidBufferSize, InvalidOutputSize};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, RuntimePropHashError>;

#[derive(Error, Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum RuntimePropHashError {
	#[error("HashComputing")]
	HashComputing(),

	#[error("Failure while fecthing the ENV: `{0}`")]
	MissingEnvironmentVariable(&'static str),

	#[error("Failure while fecthing the ENV: `{0}`")]
	HexDecoding(String),

	#[error("Unknown")]
	Unknown(),
}

impl From<InvalidOutputSize> for RuntimePropHashError {
	fn from(_e: InvalidOutputSize) -> Self {
		Self::HashComputing()
	}
}

impl From<InvalidBufferSize> for RuntimePropHashError {
	fn from(_e: InvalidBufferSize) -> Self {
		Self::HashComputing()
	}
}

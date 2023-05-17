use thiserror::Error;

pub type Result<T> = std::result::Result<T, RuntimePropHashError>;

#[derive(Error, Debug, Clone)]
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

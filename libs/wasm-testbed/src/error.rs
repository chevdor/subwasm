// use std::fmt;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, WasmTestbedError>;

#[derive(Error, Debug, Clone)]
pub enum WasmTestbedError {
	#[error("Error while loading source: `{0}`")]
	Loading(String),

	#[error("Error while calling method: `{0}`")]
	Calling(String),

	#[error("Error computing a hash")]
	Hashing(),

	#[error("Failed decoding bytes: {0:?}")]
	Decoding(Vec<u8>),

	#[error("This runtime is not supported")]
	UnsupportedRuntime,
}

// impl fmt::Display for WasmTestbedError {
// 	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
// 		match self {
// 			WasmTestbedError::Unsupported => write!(f, "This runtime is not supported"),

// 			WasmTestbedError::Decoding(bytes) => {
// 				write!(f, "The runtime could not be decoded. Here are the first bytes:\n{:02x?}", bytes[0..64].to_vec())
// 			}

// 			WasmTestbedError::Calling(method) => write!(f, "Failed calling: {method:?}"),
// 			WasmTestbedError::Loading(src) => write!(f, "Failed Loading: {src:?}"),
// 		}
// 	}
// }

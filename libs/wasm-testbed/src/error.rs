use std::{error::Error, fmt};

pub type Result<T> = std::result::Result<T, WasmTestbedError>;

#[derive(Debug, Clone)]
pub enum WasmTestbedError {
	Unsupported,
	Loading(String),
	Calling(String),
	Decoding(Vec<u8>),
}

impl Error for WasmTestbedError {}

impl fmt::Display for WasmTestbedError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			WasmTestbedError::Unsupported => write!(f, "This runtime is not supported"),

			WasmTestbedError::Decoding(bytes) => {
				write!(f, "The runtime could not be decoded. Here are the first bytes:\n{:02x?}", bytes[0..64].to_vec())
			}

			WasmTestbedError::Calling(method) => write!(f, "Failed calling: {method:?}"),
			WasmTestbedError::Loading(src) => write!(f, "Failed Loading: {src:?}"),
		}
	}
}

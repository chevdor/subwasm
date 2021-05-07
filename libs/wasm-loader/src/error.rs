use std::fmt;

// pub type Result<T> = std::result::Result<T, WasmTestbedError>;

#[derive(Debug, Clone)]
pub enum WasmTestbedError {
	EndpointParsing(String),
	OnchainBlockParsing(String),
	NotSupported(String),
}

impl fmt::Display for WasmTestbedError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			WasmTestbedError::EndpointParsing(s) | WasmTestbedError::OnchainBlockParsing(s) => {
				write!(f, "Parsing issue: {:?}", s)
			}
			WasmTestbedError::NotSupported(s) => write!(f, "Unsupported: {:?}", s),
		}
	}
}

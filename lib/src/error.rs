use crate::ChainInfoError;
use ipfs_hasher::error::IpfsHasherError;
use thiserror::Error;
use url::{ParseError, Url};
use wasm_loader::WasmLoaderError;
use wasm_testbed::WasmTestbedError;

/// Result type alias
pub type Result<T> = std::result::Result<T, SubwasmLibError>;

/// Custom error
#[derive(Error, Debug)]
pub enum SubwasmLibError {
	/// The provided pallet was not found in this runtime
	#[error("The following pallet was not found: `{0}`")]
	PalletNotFound(String),

	/// A named item was not found
	#[error("The following item was not found: `{0}`")]
	NotFound(String),

	/// No metadata could be found
	#[error("Not metadata found")]
	NoMetadataFound(),

	/// You are likely trying to compress a runtime that is already compressed
	#[error("The input is already compressed")]
	AlreadyCompressed(),

	/// An error occurred with the compression
	#[error("The compression failed")]
	CompressionFailed(),

	/// An error occurred with the decompression
	#[error("The decompression failed")]
	DecompressionFailed(),

	#[error("Unsupported variant")]
	UnsupportedVariant(),

	/// Subwasm only supports metadata version 12 and above
	#[error("Unsupported Runtime version. Subwasm supports V12 and above")]
	UnsupportedRuntimeVersion(),

	/// Error parsing (name, hint)
	#[error("Error parsing `{0}`.{1}")]
	Parsing(String, String),

	/// I/O error
	#[error("i/o error")]
	Io,

	/// The provided endpoint was not found
	#[error("Endpoint not found for `{0}`")]
	EndpointNotFound(String),

	/// An error occurred while computing a hash
	#[error("Hash error")]
	HashError(),

	/// Generic error
	#[error("Generic error: {0}")]
	Generic(String),

	/// Filtering is not available for all formats
	#[error("Cannot filter with this format")]
	UnsupportedFilter(),

	/// No runtime could be found at the provided URL
	#[error("Could not find a valid runtime at {0}")]
	NoRuntimeAtUrl(Url),

	/// The source cannot be resolved
	#[error("Cannot resolve `{0}` to a known Source")]
	UnknownSource(String),

	/// Unknown error
	#[error("Unknown error")]
	Unknown(),
}

impl From<std::io::Error> for SubwasmLibError {
	fn from(_e: std::io::Error) -> Self {
		SubwasmLibError::Unknown()
	}
}

impl From<WasmTestbedError> for SubwasmLibError {
	fn from(_e: WasmTestbedError) -> Self {
		SubwasmLibError::Generic("WasmTestbedError".to_string())
	}
}

impl From<WasmLoaderError> for SubwasmLibError {
	fn from(_e: WasmLoaderError) -> Self {
		SubwasmLibError::Generic("WasmLoaderError".to_string())
	}
}

impl From<serde_json::Error> for SubwasmLibError {
	fn from(_e: serde_json::Error) -> Self {
		SubwasmLibError::Generic("SerdeJsonError".to_string())
	}
}

impl From<IpfsHasherError> for SubwasmLibError {
	fn from(_e: IpfsHasherError) -> Self {
		SubwasmLibError::HashError()
	}
}

impl From<ChainInfoError> for SubwasmLibError {
	fn from(_e: ChainInfoError) -> Self {
		SubwasmLibError::NotFound("Chain not found: {e:?}".to_string())
	}
}

impl From<ParseError> for SubwasmLibError {
	fn from(e: ParseError) -> Self {
		SubwasmLibError::Parsing("url".to_string(), e.to_string())
	}
}

use ipfs_hasher::error::IpfsHasherError;
use thiserror::Error;
use url::{ParseError, Url};
use wasm_loader::WasmLoaderError;
use wasm_testbed::WasmTestbedError;

use crate::ChainInfoError;

pub type Result<T> = std::result::Result<T, SubwasmLibError>;

#[derive(Error, Debug)]
pub enum SubwasmLibError {
	#[error("The following pallet was not found: `{0}`")]
	PalletNotFound(String),

	#[error("The following item was not found: `{0}`")]
	NotFound(String),

	#[error("Not metadata found")]
	NoMetadataFound(),

	#[error("The input is already compressed")]
	AlreadyCompressed(),

	#[error("The compression failed")]
	CompressionFailed(),

	#[error("The decompression failed")]
	DecompressionFailed(),

	#[error("Unsupported variant")]
	UnsupportedVariant(),

	#[error("Unsupported Runtime version. Subwasm supports V12 and above")]
	UnsupportedRuntimeVersion(),

	/// (name, hint)
	#[error("Error parsing `{0}`.{1}")]
	Parsing(String, String),

	#[error("i/o error")]
	Io,

	#[error("Endpoint not found for `{0}`")]
	EndpointNotFound(String),

	#[error("Hash error")]
	HashError(),

	#[error("Generic error: {0}")]
	Generic(String),

	#[error("Cannot filter with this format")]
	UnsupportedFilter(),

	#[error("Could not find a valid runtime at {0}")]
	NoRuntimeAtUrl(Url),

	#[error("Cannot resolve `{0}` to a known Source")]
	UnknownSource(String),

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

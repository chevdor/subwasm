use substrate_differ::SubstrateDifferError;
use thiserror::Error;
use wasm_loader::WasmLoaderError;
use wasm_testbed::WasmTestbedError;

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

	#[error("The was a hashing error")]
	Hashing(),

	#[error("The compression failed")]
	CompressionFailed(),

	#[error("The decompression failed")]
	DecompressionFailed(),

	#[error("Unsupported variant")]
	UnsupportedVariant(),

	#[error("Unsupported Runtime version. Subwasm supports V12 and above")]
	UnsupportedRuntimeVersion(),

	#[error("Registry error")]
	Registry(),

	/// (name, hint)
	#[error("Error parsing `{0}`.{1}")]
	Parsing(String, String),

	#[error("i/o error")]
	Io,

	#[error("Endpoint not found for `{0}`")]
	EndpointNotFound(String),

	#[error("Generic error")]
	Generic(String),

	#[error("Cannot filter with this format")]
	UnsupportedFilter(),

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

impl From<SubstrateDifferError> for SubwasmLibError {
	fn from(_e: SubstrateDifferError) -> Self {
		SubwasmLibError::Generic("SubstrateDifferError".to_string())
	}
}

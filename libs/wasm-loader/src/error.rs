// use std::fmt;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, WasmLoaderError>;

#[derive(Error, Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum WasmLoaderError {
	#[error("Issue parsing endpoint: `{0}`")]
	EndpointParsing(String),

	#[error("Issue parsing block: `{0}`")]
	OnchainBlockParsing(String),

	#[error("Not supported: `{0}`")]
	NotSupported(String),

	#[error("Unknown source: `{0}`")]
	UnknownSource(String),

	#[error("Compression failed and returned nothing")]
	CompressionError(),

	#[error("Decompression failed")]
	DecompressionFailed(),

	#[error("URL Error: {0}")]
	UrlParsingError(String),

	#[error("HTTP Client error, url: `{0}`")]
	HttpClient(String),

	#[error("WebSocket Client error, url: `{0}`")]
	WsClient(String),
}

impl From<sp_maybe_compressed_blob::Error> for WasmLoaderError {
	fn from(_e: sp_maybe_compressed_blob::Error) -> Self {
		Self::CompressionError()
	}
}

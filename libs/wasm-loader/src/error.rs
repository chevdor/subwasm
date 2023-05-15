// use std::fmt;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, WasmLoaderError>;

#[derive(Error, Debug, Clone)]
pub enum WasmLoaderError {
	#[error("Issue parsing endpoint: `{0}`")]
	EndpointParsing(String),

	#[error("Issue parsing block: `{0}`")]
	OnchainBlockParsing(String),

	#[error("Not supported: `{0}`")]
	NotSupported(String),

	#[error("Compression failed and returned nothing")]
	CompressionError(),

	#[error("Decompression failed")]
	DecompressionError(),

	#[error("HTTP Client error, url: `{0}`")]
	HttpClient(String),

	#[error("WebSocket Client error, url: `{0}`")]
	WsClient(String),
}

// impl fmt::Display for WasmLoaderError {
// 	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
// 		match self {
// 			WasmLoaderError::EndpointParsing(s) | WasmLoaderError::OnchainBlockParsing(s) => {
// 				write!(f, "Parsing issue: {s:?}")
// 			}

// 			WasmLoaderError::NotSupported(s) => write!(f, "Unsupported: {s:?}"),
// 			WasmLoaderError::HttpClient() => write!(f, "HttpClient Error"),
// 			WasmLoaderError::WsClient() => write!(f, "WsClient Error"),
// 		}
// 	}
// }

// impl std::error::Error for WasmLoaderError {
// 	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
// 		None
// 	}

// 	fn description(&self) -> &str {
// 		"description() is deprecated; use Display"
// 	}

// 	fn cause(&self) -> Option<&dyn std::error::Error> {
// 		self.source()
// 	}
// }

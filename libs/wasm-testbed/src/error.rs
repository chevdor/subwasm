use sc_executor_common::error::WasmError;
// use std::fmt;
use substrate_runtime_proposal_hash::error::RuntimePropHashError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, WasmTestbedError>;

#[derive(Error, Debug, Clone)]
pub enum WasmTestbedError {
	#[error("Error while loading source: `{0}`")]
	Loading(String),

	#[error("Error while calling method: `{0}`")]
	Calling(String),

	#[error("Failed decoding bytes: {0:?}")]
	Decoding(Vec<u8>),

	#[error("Hash Error")]
	HashError(),

	#[error("This runtime is not supported")]
	UnsupportedRuntime,
}

impl From<RuntimePropHashError> for WasmTestbedError {
	fn from(_e: RuntimePropHashError) -> Self {
		WasmTestbedError::HashError()
	}
}

impl From<WasmError> for WasmTestbedError {
	fn from(_e: WasmError) -> Self {
		WasmTestbedError::HashError()
	}
}

impl From<scale::Error> for WasmTestbedError {
	fn from(_e: scale::Error) -> Self {
		WasmTestbedError::HashError()
	}
}

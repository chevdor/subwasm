use thiserror::Error;

pub type Result<T> = std::result::Result<T, IpfsHasherError>;

#[derive(Error, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum IpfsHasherError {
	#[error("The following pallet was not found: `{0}`")]
	PalletNotFound(String),

	#[error("Unknown error")]
	HashError(),

	#[error("Unknown error")]
	Unknown(),
}

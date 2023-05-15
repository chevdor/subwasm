use thiserror::Error;

pub type Result<T> = std::result::Result<T, IpfsHasherError>;

#[derive(Error, Debug)]
pub enum IpfsHasherError {
	#[error("An unknown error occured")]
	Unknown,
}

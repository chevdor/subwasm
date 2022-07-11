use std::fmt;

#[derive(Debug, Clone)]
pub enum Error {
	// Generic,
}

impl fmt::Display for Error {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		write!(fmt, "{:?}", self)
	}
}

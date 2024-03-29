use crate::error;
use error::*;
use serde_json::Value;
use std::fmt::Debug;

/// A trait that implements Serialize by default
pub trait Signature {
	fn serialize(&self) -> Result<Value>;
}

impl Debug for dyn Signature {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self.serialize() {
			Ok(s) => write!(f, "{s:?}"),
			Err(e) => write!(f, "{e:?}"),
		}
	}
}

impl<S: serde::ser::Serialize> Signature for S {
	fn serialize(&self) -> Result<Value> {
		serde_json::to_value(self).map_err(|_| SubstrateDifferError::SerializationError())
	}
}

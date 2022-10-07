use serde_json::Value;
use std::fmt::Debug;

/// A trait that implements Serialize by default
pub trait Signature {
	fn serialize(&self) -> Value;
}

impl Debug for dyn Signature {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self.serialize().to_string())
	}
}

impl<S: serde::ser::Serialize> Signature for S {
	fn serialize(&self) -> Value {
		serde_json::to_value(self).unwrap()
	}
}

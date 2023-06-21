/// This function reports whether the changes between 2 runtimes APIs require
/// a `transaction_version` bump.
///
/// Typically, a `transaction_version` bump is required if a chance could result
/// in a user calling the wrong call.
///
/// That could happen for instance if the ID of a Call or a Pallet has changed.
///
/// NOTE: It is possible to have cases where it is not required to bump the
/// `transaction_version`, nonetheless, the runtimes could be incompatible.
/// This can happen for instance if the **signature** of a Call has changed.
///
/// See also [Compatible]
pub trait RequireTransactionVersionBump {
	fn require_tx_version_bump(&self) -> bool;
}

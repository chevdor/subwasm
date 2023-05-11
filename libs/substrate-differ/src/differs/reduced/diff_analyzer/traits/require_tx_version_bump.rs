/// This function reports whether the 2 runtimes APIs are require_tx_version_bump or not.
/// This helps decide whether the runtime's `transaction_version` should be bumped.
pub trait RequireTransactionVersionBump {
	fn require_tx_version_bump(&self) -> bool;
}

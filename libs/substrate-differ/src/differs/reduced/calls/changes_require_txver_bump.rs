use comparable::VecChange;

use crate::differs::reduced::diff_analyzer::RequireTransactionVersionBump;

use super::call::*;
use super::constant::*;
use super::error::*;
use super::event::*;
use super::signature::*;
use super::storage::*;

impl RequireTransactionVersionBump for CallChange {
	fn require_tx_version_bump(&self) -> bool {
		match self {
			CallChange::Index(_) => false,
			CallChange::Name(_) => true,
			CallChange::Signature(s) => s.require_tx_version_bump(),
		}
	}
}

impl RequireTransactionVersionBump for ConstantChange {
	fn require_tx_version_bump(&self) -> bool {
		true
	}
}

impl RequireTransactionVersionBump for EventChange {
	fn require_tx_version_bump(&self) -> bool {
		true
	}
}

impl RequireTransactionVersionBump for ErrorChange {
	fn require_tx_version_bump(&self) -> bool {
		true
	}
}

impl RequireTransactionVersionBump for StorageChange {
	fn require_tx_version_bump(&self) -> bool {
		true
	}
}

impl RequireTransactionVersionBump for SignatureChange {
	fn require_tx_version_bump(&self) -> bool {
		self.args.iter().map(|arg_changes| arg_changes.require_tx_version_bump()).all(|x| x)
	}
}

impl RequireTransactionVersionBump for VecChange<ArgDesc, Vec<ArgChange>> {
	fn require_tx_version_bump(&self) -> bool {
		match self {
			VecChange::Added(_size, _desc) => false,
			VecChange::Removed(_size, _desc) => false,
			VecChange::Changed(_size, change) => change.require_tx_version_bump(),
		}
	}
}

impl RequireTransactionVersionBump for Vec<ArgChange> {
	fn require_tx_version_bump(&self) -> bool {
		self.iter().map(|c| c.require_tx_version_bump()).all(|x| x)
	}
}

impl RequireTransactionVersionBump for ArgChange {
	fn require_tx_version_bump(&self) -> bool {
		match self {
			// Changing the name is fine
			ArgChange::Name(_) => true,
			// Changing the type is not
			ArgChange::Ty(_) => false,
		}
	}
}

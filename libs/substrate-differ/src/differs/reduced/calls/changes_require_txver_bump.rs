use super::{call::*, constant::*, error::*, event::*, signature::*, storage::*};
use crate::differs::reduced::{diff_analyzer::RequireTransactionVersionBump, prelude::ReducedPalletChange};
use comparable::{MapChange, VecChange};
use log::trace;

impl RequireTransactionVersionBump for ReducedPalletChange {
	fn require_tx_version_bump(&self) -> bool {
		let res = match self {
			ReducedPalletChange::Index(_) => true,

			ReducedPalletChange::Calls(x) => x
				.iter()
				.map(|i| match i {
					MapChange::Added(_k, _d) => false,
					MapChange::Removed(_k) => true,
					MapChange::Changed(_k, c) => c.iter().map(|cc| cc.require_tx_version_bump()).any(|x| x),
				})
				.any(|x| x),

			ReducedPalletChange::Name(_) => false,
			ReducedPalletChange::Events(_x) => false,
			ReducedPalletChange::Errors(_x) => false,
			ReducedPalletChange::Storages(_x) => false,
			ReducedPalletChange::Constants(_x) => false,
		};
		trace!("TxBump | Pallet: {res}");
		res
	}
}

impl RequireTransactionVersionBump for CallChange {
	fn require_tx_version_bump(&self) -> bool {
		let res = match self {
			CallChange::Index(_) => true,
			CallChange::Name(_) => false,
			CallChange::Signature(s) => s.require_tx_version_bump(),
		};
		trace!("TxBump | CallChange: {res}");
		res
	}
}

impl RequireTransactionVersionBump for ConstantChange {
	fn require_tx_version_bump(&self) -> bool {
		let res = false;
		trace!("TxBump | ConstantChange: {res}");
		res
	}
}

impl RequireTransactionVersionBump for EventChange {
	fn require_tx_version_bump(&self) -> bool {
		let res = false;
		trace!("TxBump | EventChange: {res}");
		res
	}
}

impl RequireTransactionVersionBump for ErrorChange {
	fn require_tx_version_bump(&self) -> bool {
		let res = false;
		trace!("TxBump | ErrorChange: {res}");
		res
	}
}

impl RequireTransactionVersionBump for StorageChange {
	fn require_tx_version_bump(&self) -> bool {
		let res = false;
		trace!("TxBump | StorageChange: {res}");
		res
	}
}

impl RequireTransactionVersionBump for SignatureChange {
	fn require_tx_version_bump(&self) -> bool {
		let res = self.args.iter().map(|arg_changes| arg_changes.require_tx_version_bump()).any(|x| x);
		trace!("TxBump | SignatureChange: {res}");
		res
	}
}

impl RequireTransactionVersionBump for VecChange<Arg, Vec<ArgChange>> {
	fn require_tx_version_bump(&self) -> bool {
		let res = match self {
			// If an arg is added/removed, the call will no longer be **compatible** but that does not require a tx_version bump
			VecChange::Added(_size, _desc) => false,
			VecChange::Removed(_size, _desc) => false,

			VecChange::Changed(_size, change) => change.require_tx_version_bump(),
		};
		trace!("TxBump | VecChange<...>: {res}");
		res
	}
}

impl RequireTransactionVersionBump for Vec<ArgChange> {
	fn require_tx_version_bump(&self) -> bool {
		let res = self.iter().map(|c| c.require_tx_version_bump()).any(|x| x);
		trace!("TxBump | Vec<ArgChange>: {res}");
		res
	}
}

impl RequireTransactionVersionBump for ArgChange {
	fn require_tx_version_bump(&self) -> bool {
		let res = match self {
			// Changing the name is fine
			ArgChange::Name(_) => false,

			// Changing the type is not ok for compat but ok for tx_version
			ArgChange::Ty(_) => false,
		};
		trace!("TxBump | ArgChange: {res}");
		res
	}
}

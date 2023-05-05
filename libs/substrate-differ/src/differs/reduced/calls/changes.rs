use comparable::VecChange;

use crate::differs::reduced::diff_analyzer::Compatible;

use super::call::*;
use super::constant::*;
use super::error::*;
use super::event::*;
use super::signature::*;
use super::storage::*;

impl Compatible for CallChange {
	fn compatible(&self) -> bool {
		match self {
			CallChange::Index(_) => false,
			CallChange::Name(_) => true,
			CallChange::Signature(s) => s.compatible(),
		}
	}
}

impl Compatible for ConstantChange {
	fn compatible(&self) -> bool {
		true
	}
}

impl Compatible for EventChange {
	fn compatible(&self) -> bool {
		true
	}
}

impl Compatible for ErrorChange {
	fn compatible(&self) -> bool {
		true
	}
}

impl Compatible for StorageChange {
	fn compatible(&self) -> bool {
		true
	}
}

impl Compatible for SignatureChange {
	fn compatible(&self) -> bool {
		self.args.iter().map(|arg_changes| arg_changes.compatible()).all(|x| x)
	}
}

impl Compatible for VecChange<ArgDesc, Vec<ArgChange>> {
	fn compatible(&self) -> bool {
		match self {
			VecChange::Added(_size, _desc) => false,
			VecChange::Removed(_size, _desc) => false,
			VecChange::Changed(_size, change) => change.compatible(),
		}
	}
}

impl Compatible for Vec<ArgChange> {
	fn compatible(&self) -> bool {
		self.iter().map(|c| c.compatible()).all(|x| x)
	}
}

impl Compatible for ArgChange {
	fn compatible(&self) -> bool {
		match self {
			// Changing the name is fine
			ArgChange::Name(_) => true,
			// Changing the type is not
			ArgChange::Ty(_) => false,
		}
	}
}

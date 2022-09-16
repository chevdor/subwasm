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
		self.args.iter().map(|_arg_changes| false).all(|x| x)
	}
}

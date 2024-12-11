use super::{call::*, constant::*, error::*, event::*, signature::*, storage::*};
use crate::differs::reduced::{diff_analyzer::Compatible, prelude::ReducedPalletChange};
use comparable::{MapChange, VecChange};
use log::trace;

impl Compatible for ReducedPalletChange {
	fn compatible(&self) -> bool {
		let res = match self {
			ReducedPalletChange::Index(_) => false,
			ReducedPalletChange::Name(_) => false,

			ReducedPalletChange::Calls(x) => x.compatible(),
			ReducedPalletChange::Events(x) => x.compatible(),
			ReducedPalletChange::Errors(x) => x.compatible(),
			ReducedPalletChange::Constants(_x) => true,
			ReducedPalletChange::Storages(x) => x.compatible(),
		};

		trace!("Compat. | Pallet: {res}");
		res
	}
}

impl Compatible for CallChange {
	fn compatible(&self) -> bool {
		let res = match self {
			CallChange::Index(_) => false,
			CallChange::Name(_) => false,
			CallChange::Signature(s) => s.compatible(),
		};

		trace!("Compat. | Call: {res}");
		res
	}
}

impl Compatible for ConstantChange {
	fn compatible(&self) -> bool {
		let res = match self {
			ConstantChange::Name(_) => false,
			ConstantChange::Value(_) => true,
			ConstantChange::Ty(_) => false,
		};
		trace!("Compat. | Constant: {res}");
		res
	}
}

impl Compatible for EventChange {
	fn compatible(&self) -> bool {
		let res = match self {
			EventChange::Index(_) => false,
			EventChange::Name(_) => false,
			EventChange::Signature(s) => s.compatible(),
		};
		trace!("Compat. | Event: {res}");
		res
	}
}

impl Compatible for ErrorChange {
	fn compatible(&self) -> bool {
		let res = match self {
			ErrorChange::Index(_) => false,
			ErrorChange::Name(_) => false,
		};
		trace!("Compat. | Error: {res}");
		res
	}
}

impl Compatible for StorageChange {
	fn compatible(&self) -> bool {
		let res = match self {
			StorageChange::Name(_) => false,
			StorageChange::Modifier(_) => false,
			StorageChange::Ty(_) => false,
			StorageChange::DefaultValue(_) => true,
		};
		trace!("Compat. | Storage: {res}");
		res
	}
}

impl Compatible for SignatureChange {
	fn compatible(&self) -> bool {
		let res = self.args.iter().map(|arg_changes| arg_changes.compatible()).all(|x| x);
		trace!("Compat. | Signature: {res}");
		res
	}
}

impl<T: Compatible> Compatible for Vec<T> {
	fn compatible(&self) -> bool {
		let res = self.iter().map(|c| c.compatible()).all(|x| x);
		trace!("Compat. | Vec<T>: {res}");
		res
	}
}

impl Compatible for VecChange<Arg, Vec<ArgChange>> {
	fn compatible(&self) -> bool {
		let res = match self {
			VecChange::Added(_size, _desc) => false,
			VecChange::Removed(_size, _desc) => false,
			VecChange::Changed(_size, change) => change.compatible(),
		};
		trace!("Compat. | VecChange: {res}");
		res
	}
}

impl<Key, Desc, Change: Compatible> Compatible for MapChange<Key, Desc, Change> {
	fn compatible(&self) -> bool {
		let res = match self {
			MapChange::Added(_key, _desc) => true,
			MapChange::Removed(_key) => false,
			MapChange::Changed(_key, change) => change.compatible(),
		};
		trace!("Compat. | MapChange: {res}");
		res
	}
}

impl Compatible for ArgChange {
	fn compatible(&self) -> bool {
		let res = match self {
			ArgChange::Name(_) => false,
			ArgChange::Ty(_) => false,
		};
		trace!("Compat. | ArgChange: {res}");
		res
	}
}

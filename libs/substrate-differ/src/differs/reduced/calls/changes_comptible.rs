use super::{call::*, constant::*, error::*, event::*, hashed_type::HashedTypeChange, signature::*, storage::*};
use crate::differs::reduced::{diff_analyzer::Compatible, prelude::ReducedPalletChange};
use comparable::{Changed, MapChange, StringChange, VecChange};
use log::trace;

impl Compatible for ReducedPalletChange {
	fn compatible(&self) -> bool {
		let res = match self {
			ReducedPalletChange::Index(_) => false,
			ReducedPalletChange::Name(_) => false,
			ReducedPalletChange::StoragePrefix(_) => false,

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
			ErrorChange::Name(_) => true,
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
			StorageChange::Ty(ty) => ty.compatible(),
			StorageChange::DefaultValue(_) => true,
		};
		trace!("Compat. | Storage: {res}");
		res
	}
}

impl Compatible for StorageTypeChange {
	fn compatible(&self) -> bool {
		let res = match self {
			Self::BothPlain(ty) => ty.compatible(),
			Self::BothMap { hashers, key, value } => hashers.is_unchanged() && key.compatible() && value.compatible(),
			Self::Different(_, _) => false,
		};
		trace!("Compat. | StorageType: {res}");
		res
	}
}

impl Compatible for SignatureChange {
	fn compatible(&self) -> bool {
		let res = self.args.compatible();
		trace!("Compat. | Signature: {res}");
		res
	}
}

impl<T: Compatible> Compatible for Changed<T> {
	fn compatible(&self) -> bool {
		let res = match self {
			Changed::Unchanged => true,
			Changed::Changed(x) => x.compatible(),
		};
		trace!("Compat. | {}: {res}", std::any::type_name::<Self>());
		res
	}
}

impl<T: Compatible> Compatible for Vec<T> {
	fn compatible(&self) -> bool {
		let res = self.iter().map(|c| c.compatible()).all(|x| x);
		trace!("Compat. | {}: {res}", std::any::type_name::<Self>());
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
		trace!("Compat. | {}: {res}", std::any::type_name::<Self>());
		res
	}
}

impl Compatible for ArgChange {
	fn compatible(&self) -> bool {
		let res = match self {
			ArgChange::Name(StringChange(old, new)) => {
				// Ignore underscore prefix change.
				if old.trim_start_matches('_') == new.trim_start_matches('_') {
					true
				} else {
					false
				}
			}
			ArgChange::Ty(ty) => ty.compatible(),
		};
		trace!("Compat. | ArgChange: {res}");
		res
	}
}

impl Compatible for HashedTypeChange {
	fn compatible(&self) -> bool {
		let res = match self {
			// The type changed.
			HashedTypeChange::TypeChanged(_) => false,
			// If only the name changed, then it is compatible.  This is to allow types to be renamed or moved to different modules.
			HashedTypeChange::NameChanged(_) => true,
			// If the type hash changed, then it is not compatible.
			HashedTypeChange::HashChanged(_) => false,
			// If both name and hash changed then it is not compatible.
			HashedTypeChange::NameAndHashChanged(_, _) => false,
		};
		trace!("Compat. | HashedTypeChange: {res}");
		res
	}
}

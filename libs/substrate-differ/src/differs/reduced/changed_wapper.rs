use super::{reduced_pallet::*, reduced_runtime_change_wrapper::ReducedRuntimeChangeWrapper};
use crate::differs::reduced::calls::prelude::PalletId;
use comparable::MapChange;
use serde::Serialize;
use std::fmt::Display;

#[derive(Debug, Serialize)]
pub struct ChangedWrapper(pub(crate) ReducedRuntimeChangeWrapper);

impl ChangedWrapper {
	pub fn get_pallets_changes(&self) -> &Vec<MapChange<PalletId, ReducedPalletDesc, Vec<ReducedPalletChange>>> {
		&self.0.changes.pallets
	}

	pub fn get_pallet_changes_by_id(
		&self,
		pallet_id: PalletId,
	) -> Option<&MapChange<PalletId, ReducedPalletDesc, Vec<ReducedPalletChange>>> {
		self.0.changes.pallets.iter().find(|&map_change| {
			matches!(map_change,
				MapChange::Added(id, _) |
				MapChange::Changed(id, _) |
				MapChange::Removed(id) if id == &pallet_id)
		})
	}

	// pub fn get_pallet_by_id(&self,
	// 	pallet_id: PalletId) -> &ReducedPallet {
	// 		self.0.
	// 	}

	// pub fn new(rrw: ReducedRuntimeChangeWrapper) -> Self {
	// 	Self(rrw)
	// }
}

impl From<ReducedRuntimeChangeWrapper> for ChangedWrapper {
	fn from(c: ReducedRuntimeChangeWrapper) -> Self {
		Self(c)
	}
}

impl Display for ChangedWrapper {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("{}", &self.0))

		// // todo: XX12 we may need to do that "down" and have access to the runtimes references
		// println!("DO NOT IMPL DISPLAY HERE, WE DON'T HAVE THE RUNTIMES HERE.");

		// // TODO: See other todo, we need to get rid of that
		// self.get_pallets_changes().iter().for_each(
		// 	|mc: &MapChange<PalletId, ReducedPalletDesc, Vec<ReducedPalletChange>>| match mc {
		// 		MapChange::Added(pallet_id, reduced_pallet) => {
		// 			let _ = writeln!(f, "[+] id: {:>2} - new pallet: {}", pallet_id, reduced_pallet.name);
		// 		}
		// 		MapChange::Removed(pallet_id) => {
		// 			let _ = writeln!(f, "[-] pallet {}", pallet_id);
		// 		}

		// 		MapChange::Changed(pallet_id, changes) => {
		// 			let _ = writeln!(f, "[≠] pallet {}: -> {} change(s)", pallet_id, changes.len());
		// 			changes.iter().for_each(|reduced_pallet_change| {
		// 				let _ = writeln!(f, "{}", reduced_pallet_change);
		// 			});
		// 		}
		// 	},
		// );
		// Ok(())
	}
}

// impl AsRef<ReducedRuntimeChangeWrapper> for ChangedWrapper {
// 	fn as_ref(&self) -> &'a ReducedRuntimeChangeWrapper {
// 		&self.0
// 	}
// }

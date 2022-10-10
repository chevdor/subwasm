use super::{reduced_pallet::*, reduced_runtime::*};
use crate::differs::reduced::calls::prelude::PalletId;
use comparable::MapChange;
use serde::Serialize;
use std::fmt::Display;

#[derive(Debug, Serialize)]
pub struct ReducedRuntimeChangeWrapper(pub(crate) ReducedRuntimeChange);

#[derive(Debug, Serialize)]
pub struct ChangedWrapper(pub(crate) ReducedRuntimeChangeWrapper);

impl ChangedWrapper {
	pub fn get_pallets_changes(&self) -> &Vec<MapChange<PalletId, ReducedPalletDesc, Vec<ReducedPalletChange>>> {
		&self.0 .0.pallets
	}

	pub fn get_pallet_changes_by_id(
		&self,
		pallet_id: PalletId,
	) -> Option<&MapChange<PalletId, ReducedPalletDesc, Vec<ReducedPalletChange>>> {
		self.0.0.pallets.iter().find(|&map_change|
			matches!(map_change,
				MapChange::Added(id, _) |
				MapChange::Changed(id, _) |
				MapChange::Removed(id) if id == &pallet_id))
	}
}

impl From<ReducedRuntimeChangeWrapper> for ChangedWrapper {
	fn from(c: ReducedRuntimeChangeWrapper) -> Self {
		Self(c)
	}
}

impl From<ReducedRuntimeChange> for ReducedRuntimeChangeWrapper {
	fn from(reduced_runtime_change: ReducedRuntimeChange) -> Self {
		Self(reduced_runtime_change)
	}
}

impl Display for ChangedWrapper {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		// TODO: See other todo, we need to get rid of that
		self.get_pallets_changes()
			.iter()
			.for_each(|mc: &MapChange<PalletId, ReducedPalletDesc, Vec<ReducedPalletChange>>| {
			match mc {
				MapChange::Added(pallet_id, reduced_pallet) => {
					let _ = writeln!(f, "[+] id: {:>2} - new pallet: {}", pallet_id, reduced_pallet.name);
				}
				MapChange::Removed(pallet_id) => {
					let _ = writeln!(f, "[-] pallet {}", pallet_id);
				}

				MapChange::Changed(pallet_id, changes) => {
					let _ = writeln!(f, "[≠] pallet {}: -> {} change(s)", pallet_id, changes.len());
					changes.iter().for_each(|reduced_pallet_change| {
						let _ = writeln!(f, "{}", reduced_pallet_change);
					});
				}
			}
		});
		Ok(())
	}
}

impl AsRef<ReducedRuntimeChangeWrapper> for ChangedWrapper {
	fn as_ref(&self) -> &ReducedRuntimeChangeWrapper {
		&self.0
	}
}

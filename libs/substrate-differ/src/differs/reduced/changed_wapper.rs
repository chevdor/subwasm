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
}

impl From<ReducedRuntimeChangeWrapper> for ChangedWrapper {
	fn from(c: ReducedRuntimeChangeWrapper) -> Self {
		Self(c)
	}
}

impl Display for ChangedWrapper {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("{}", &self.0))
	}
}

use super::reduced_runtime::ReducedRuntimeChange;
use super::{reduced_pallet::*, reduced_runtime_change_wrapper::ReducedRuntimeChangeWrapper};
use crate::differs::reduced::calls::prelude::PalletId;
use comparable::MapChange;
use serde::Serialize;
use std::fmt::Display;

type Change = MapChange<PalletId, ReducedPalletDesc, Vec<ReducedPalletChange>>;

#[derive(Debug, Serialize)]
pub struct ChangedWrapper(pub(crate) ReducedRuntimeChangeWrapper);

impl ChangedWrapper {
	/// Loop though the changes and filter to keep only the changes related to pallets
	pub fn get_pallets_changes(&self) -> Vec<&Change> {
		self.0
			.changes
			.iter()
			.filter_map(|c| match c {
				ReducedRuntimeChange::Extrinsic(_) => None,
				ReducedRuntimeChange::Pallets(p) => Some(p),
			})
			.flatten()
			.collect()
	}

	// pub fn get_extrinsic_changes(&self) -> &Vec<Change> {
	// 	// &self.runtime_metadata().changes.pallets
	// 	match &self.runtime_metadata().changes {
	// 		super::reduced_runtime::ReducedRuntimeChange::Extrinsics(_) => &vec![],
	// 		super::reduced_runtime::ReducedRuntimeChange::Pallets(p) => p,
	// 	}
	// }

	pub fn get_pallet_changes_by_id(&self, pallet_id: PalletId) -> Option<&Change> {
		let res: Vec<&Change> = self
			.0
			.changes
			.iter()
			.flat_map(|change| match change {
				ReducedRuntimeChange::Extrinsic(_ex) => None,
				ReducedRuntimeChange::Pallets(pallets) => pallets.iter().find(|&map_change| {
					matches!(map_change,
							MapChange::Added(id, _) |
							MapChange::Changed(id, _) |
							MapChange::Removed(id) if id == &pallet_id)
				}),
			})
			.collect();
		res.into_iter().next()
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

use super::{
	diff_analyzer::{Compatible, RequireTransactionVersionBump},
	reduced_pallet,
};
// use crate::differs::reduced::change_type::Change;
use comparable::MapChange;
use log::*;
use reduced_pallet::*;
use std::fmt::Display;

impl ReducedPalletChange {
	// /// Get only the calls, or the events, etc...
	pub fn filter_changed_items(changes: &[ReducedPalletChange], what: PalletItemType) -> Vec<&ReducedPalletChange> {
		changes
			.iter()
			.filter(|x| match x {
				ReducedPalletChange::Calls(_) => matches!(what, PalletItemType::Call),
				ReducedPalletChange::Events(_) => matches!(what, PalletItemType::Event),
				ReducedPalletChange::Errors(_) => matches!(what, PalletItemType::Error),
				ReducedPalletChange::Constants(_) => matches!(what, PalletItemType::Constant),
				ReducedPalletChange::Storages(_) => matches!(what, PalletItemType::Storage),
				_ => unreachable!(),
			})
			.collect()
	}

	pub fn format<KEY: Display, DESC: std::fmt::Debug, CHANGE: std::fmt::Debug>(
		f: &mut std::fmt::Formatter<'_>,
		changes: &[MapChange<KEY, DESC, Vec<CHANGE>>],
		item_type: PalletItemType,
	) -> std::fmt::Result {
		changes.iter().for_each(|ch| match ch {
			MapChange::Added(item_id, d) => {
				let _ = writeln!(f, "  - [+] {item_type}: {item_id} {d:?}");
			}
			MapChange::Changed(item_id, c) => {
				let _ = writeln!(f, "  - [≠] {item_type}: {item_id} {c:?}");
			}
			MapChange::Removed(item_id) => {
				let _ = writeln!(f, "  - [-] {item_type}: {item_id}");
			}
		});
		Ok(())
	}
}

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
				.all(|x| x),

			ReducedPalletChange::Name(_) => false,
			ReducedPalletChange::Events(_x) => false,
			ReducedPalletChange::Errors(_x) => false,
			ReducedPalletChange::Storages(_x) => false,
			ReducedPalletChange::Constants(_x) => false,
		};

		trace!("Pallet: {res}");
		res
	}
}

impl Compatible for ReducedPalletChange {
	fn compatible(&self) -> bool {
		match self {
			ReducedPalletChange::Index(_) => false,
			ReducedPalletChange::Name(_) => true,

			ReducedPalletChange::Calls(x) => x
				.iter()
				.map(|i| match i {
					MapChange::Added(_k, _d) => true,
					MapChange::Removed(_k) => false,
					MapChange::Changed(_k, c) => c.iter().map(|cc| cc.compatible()).all(|x| x),
				})
				.all(|x| x),
			ReducedPalletChange::Events(_x) => true,
			ReducedPalletChange::Errors(_x) => true,

			ReducedPalletChange::Constants(_x) => true,
			// x.iter()
			// .map(|i| match i {
			// 	MapChange::Added(_k, _d) => true,
			// 	MapChange::Removed(_k) => true,
			// 	MapChange::Changed(_k, c) => c.iter().map(|cc| cc.compatible()).all(|x| x.into()),
			// })
			// .all(|x| x.into()),
			ReducedPalletChange::Storages(_x) => true,
		}
	}
}

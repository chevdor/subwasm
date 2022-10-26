use super::{diff_analyzer::Compatible, reduced_pallet};
// use crate::differs::reduced::change_type::Change;
use comparable::MapChange;
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

impl Display for ReducedPalletChange {
	fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		panic!("DONT CALL ME ! Make a ReducedPalletChangeWrapper instead and call display on it");
		// let constant_changes = ReducedPalletChange::get_changed_items(self, PalletItemType::Constants);

		// match self {
		// 	// CONSTANT
		// 	ReducedPalletChange::Index(_change) => unreachable!(),
		// 	ReducedPalletChange::Name(_change) => unreachable!(),

		// 	ReducedPalletChange::Calls(changes) => {
		// 		ReducedPalletChange::format::<u32, CallDesc, CallChange>(f, changes, PalletItemType::Call)
		// 	}

		// 	ReducedPalletChange::Events(change) => {
		// 		ReducedPalletChange::format::<u32, EventDesc, EventChange>(f, change, PalletItemType::Event)
		// 	}

		// 	ReducedPalletChange::Errors(change) => {
		// 		ReducedPalletChange::format::<u32, ErrorDesc, ErrorChange>(f, change, PalletItemType::Error)
		// 	}

		// 	ReducedPalletChange::Constants(change) => {
		// 		ReducedPalletChange::format::<String, ConstantDesc, ConstantChange>(f, change, PalletItemType::Constant)
		// 	}

		// 	ReducedPalletChange::Storages(change) => {
		// 		ReducedPalletChange::format::<String, StorageDesc, StorageChange>(f, change, PalletItemType::Storage)
		// 	}
		// }
	}
}

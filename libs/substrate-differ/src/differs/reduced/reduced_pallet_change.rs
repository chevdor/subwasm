use super::{diff_analyzer::Compatible, reduced_pallet};
use comparable::MapChange;
use reduced_pallet::*;
use std::fmt::Display;

impl ReducedPalletChange {
	pub fn is_storage_compatible(&self) -> bool {
		match self {
			ReducedPalletChange::Storages(x) => x.compatible(),
			_ => true,
		}
	}

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

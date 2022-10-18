use std::fmt::Display;
use std::rc::Rc;

use super::ComparisonSide;
use super::calls::ExtrinsicId;
use super::reduced_pallet::*;
// use comparable::MapChange;
use serde::Serialize;
use crate::differs::reduced::calls::Call;

#[derive(Debug, Serialize)]
pub struct ReducedPalletChangeWrapper {
	pub(crate) changes: ReducedPalletChange,
	pub(crate) pallet_a: Rc<ReducedPallet>,
	pub(crate) pallet_b: Rc<ReducedPallet>,
}

impl ReducedPalletChangeWrapper {
	pub fn new(changes: ReducedPalletChange, pallet_a: Rc<ReducedPallet>, pallet_b: Rc<ReducedPallet>) -> Self {
		Self { changes, pallet_a, pallet_b }
	}

	fn get_call(&self, id: &ExtrinsicId, side: ComparisonSide) -> Option<&Call> {
		let reduced_pallet = match side {
			ComparisonSide::Left => &self.pallet_a,
			ComparisonSide::Right => &self.pallet_b,
		};
		reduced_pallet.calls.get(id)
	}
}

impl Display for ReducedPalletChangeWrapper {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		todo!("ReducedPalletChangeWrapper::Display");
		// self.changes.pallets.iter().for_each(
		// 	|mc: &MapChange<PalletId, ReducedPalletDesc, Vec<ReducedPalletChange>>| match mc {
		// 		MapChange::Added(pallet_id, reduced_pallet) => {
		// 			let _ =
		// 				writeln!(f, "[+] id: {id:>2} - new pallet: {name}", id = pallet_id, name = reduced_pallet.name);
		// 		}
		// 		MapChange::Removed(pallet_id) => {
		// 			let pallet = self.get_pallet(pallet_id, ComparisonSide::Left);
		// 			let pallet_name = match pallet {
		// 				Some(p) => &p.name,
		// 				None => "n/a",
		// 			};
		// 			let _ = writeln!(f, "[-] pallet {id}: {name}", id = pallet_id, name = pallet_name);
		// 		}

		// 		MapChange::Changed(pallet_id, changes) => {
		// 			let pallet_a = self.get_pallet(pallet_id, ComparisonSide::Left);
		// 			let _pallet_b = self.get_pallet(pallet_id, ComparisonSide::Right);
		// 			let pallet_a_name = match pallet_a {
		// 				Some(p) => &p.name,
		// 				None => "n/a",
		// 			};

		// 			let _ = writeln!(
		// 				f,
		// 				"[≠] pallet {id}: {name_a} -> {count} change(s)",
		// 				id = pallet_id,
		// 				name_a = pallet_a_name,
		// 				count = ReducedRuntimeChangeWrapper::get_changes_count(changes)
		// 			);
		// 			changes.iter().for_each(|reduced_pallet_change| {
		// 				let _ = writeln!(f, "{}", reduced_pallet_change);
		// 			});
		// 		}
		// 	},
		// );
		Ok(())
	}
}

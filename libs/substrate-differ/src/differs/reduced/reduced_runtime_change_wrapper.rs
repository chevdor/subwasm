use crate::differs::reduced::calls::PalletId;
use std::fmt::Display;
use std::rc::Rc;

use super::ComparisonSide;
use super::reduced_pallet::*;
use super::reduced_runtime::*;
use comparable::MapChange;
use serde::Serialize;

/// This struct is important as it brings together the diff as well
/// as references to the runtimes that have been diffed.
/// That allows implementing [Display] for instance, while still having some
/// contextual information about both the compared runtimes.
///
/// For instance, the comparison of the runtimes, may lead to a diff
/// showing that pallet 39 has changed. We need however information about the
/// runtimes to fetch more  informations about this pallet 39, such as its name
/// for instance.
#[derive(Debug, Serialize)]
pub struct ReducedRuntimeChangeWrapper {
	pub(crate) changes: ReducedRuntimeChange,
	pub(crate) runtime_a: Rc<ReducedRuntime>,
	pub(crate) runtime_b: Rc<ReducedRuntime>,
}

impl ReducedRuntimeChangeWrapper {
	pub fn new(changes: ReducedRuntimeChange, runtime_a: Rc<ReducedRuntime>, runtime_b: Rc<ReducedRuntime>) -> Self {
		Self { changes, runtime_a, runtime_b }
	}

	fn get_pallet(&self, id: &PalletId, side: ComparisonSide) -> Option<&ReducedPallet> {
		let reduced_runtime = match side {
			ComparisonSide::Left => &self.runtime_a,
			ComparisonSide::Right => &self.runtime_b,
		};
		reduced_runtime.pallets.get(id)
	}

	/// We cannot just count the number of items in the Vec we get since the upper
	/// levels will only contains Call types. So for instance, if we have only changes on [Calls], the length
	/// of the top level will be 1 (=Call) and contain a Vec for the list of Call changes. So we need to iterate
	/// on the second level to get the total amount of changes.
	fn get_changes_count(changes: &Vec<ReducedPalletChange>) -> usize {
		let val = changes
			.iter()
			.map(|item| match item {
				ReducedPalletChange::Index(_) | ReducedPalletChange::Name(_) => 1,
				ReducedPalletChange::Calls(x) => x.len(),
				ReducedPalletChange::Events(x) => x.len(),
				ReducedPalletChange::Errors(x) => x.len(),
				ReducedPalletChange::Constants(x) => x.len(),
				ReducedPalletChange::Storages(x) => x.len(),
			})
			.fold(0, |acc, x| acc + x);
		val
	}
}

impl Display for ReducedRuntimeChangeWrapper {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.changes.pallets.iter().for_each(
			|mc: &MapChange<PalletId, ReducedPalletDesc, Vec<ReducedPalletChange>>| match mc {
				MapChange::Added(pallet_id, reduced_pallet) => {
					let _ =
						writeln!(f, "[+] id: {id:>2} - new pallet: {name}", id = pallet_id, name = reduced_pallet.name);
				}
				MapChange::Removed(pallet_id) => {
					let pallet = self.get_pallet(pallet_id, ComparisonSide::Left);
					let pallet_name = match pallet {
						Some(p) => &p.name,
						None => "n/a",
					};
					let _ = writeln!(f, "[-] pallet {id}: {name}", id = pallet_id, name = pallet_name);
				}

				MapChange::Changed(pallet_id, changes) => {
					let pallet_a = self.get_pallet(pallet_id, ComparisonSide::Left);
					let _pallet_b = self.get_pallet(pallet_id, ComparisonSide::Right);
					let pallet_a_name = match pallet_a {
						Some(p) => &p.name,
						None => "n/a",
					};

					let _ = writeln!(
						f,
						"[≠] pallet {id}: {name_a} -> {count} change(s)",
						id = pallet_id,
						name_a = pallet_a_name,
						count = ReducedRuntimeChangeWrapper::get_changes_count(changes)
					);
					changes.iter().for_each(|reduced_pallet_change| {
						let _ = writeln!(f, "{}", reduced_pallet_change);
					});
				}
			},
		);
		Ok(())
	}
}

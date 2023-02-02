use crate::differs::reduced::calls::PalletId;
use std::fmt::Display;
use std::rc::Rc;

use super::reduced_pallet::*;
use super::reduced_pallet_change_wrapper::ReducedPalletChangeWrapper;
use super::reduced_runtime::*;
use super::ComparisonSide;
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
	pub(crate) changes: Vec<ReducedRuntimeChange>,
	pub(crate) runtime_a: Rc<ReducedRuntime>,
	pub(crate) runtime_b: Rc<ReducedRuntime>,
}

impl ReducedRuntimeChangeWrapper {
	pub fn new(
		changes: Vec<ReducedRuntimeChange>,
		runtime_a: Rc<ReducedRuntime>,
		runtime_b: Rc<ReducedRuntime>,
	) -> Self {
		Self { changes, runtime_a, runtime_b }
	}

	/// After deciding whether to use the Left or Right [ReducedRuntime], we
	/// try fetching a reference to the pallet with the given `id`.
	///
	/// There are cases where it will return `None`. For instance, requesting the Left
	/// pallet with `id=N` when pallet `N` was introduced first in the Right runtime.
	fn get_pallet(&self, id: &PalletId, side: ComparisonSide) -> Option<&ReducedPallet> {
		let reduced_runtime = match side {
			ComparisonSide::Left => &self.runtime_a,
			ComparisonSide::Right => &self.runtime_b,
		};
		reduced_runtime.pallets.get(id)
	}

	/// We cannot just count the number of items in the Vec we get since the upper
	/// levels will only contains Call types.
	///
	/// For instance, if we have only changes on [Calls], the length
	/// of the top level will be 1 (=Call) and contain a Vec for the list of Call changes. So we need to iterate
	/// on the second level to get the total amount of changes.
	fn get_changes_count(changes: &[ReducedPalletChange]) -> usize {
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
			.sum();
		val
	}
}

impl Display for ReducedRuntimeChangeWrapper {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.changes.iter().for_each(|change| {
			match change {
				ReducedRuntimeChange::Extrinsic(_ex) => {
					let _ = writeln!(f, "EX Change");
				}
				ReducedRuntimeChange::Pallets(pallets) => {
					pallets.iter().for_each(|mc: &MapChange<PalletId, ReducedPalletDesc, Vec<ReducedPalletChange>>| {
						match mc {
							MapChange::Added(pallet_id, reduced_pallet) => {
								let _ = writeln!(
									f,
									"[+] id: {pallet_id:>2} - new pallet: {name}",
									name = reduced_pallet.name
								);
							}
							MapChange::Removed(pallet_id) => {
								let pallet = self.get_pallet(pallet_id, ComparisonSide::Left);
								let pallet_name = match pallet {
									Some(p) => &p.name,
									None => "n/a",
								};
								let _ = writeln!(f, "[-] pallet {pallet_id}: {pallet_name}");
							}

							MapChange::Changed(pallet_id, changes) => {
								let pallet_a = self.get_pallet(pallet_id, ComparisonSide::Left);
								let pallet_b = self.get_pallet(pallet_id, ComparisonSide::Right);
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
									// let pallet_a_rc = pallet_a.map(Rc::new);
									// let pallet_b_rc = pallet_b.map(Rc::new);
									let reduced_pallet_change_wrapper =
											// ReducedPalletChangeWrapper::new(reduced_pallet_change, pallet_a_rc, pallet_b_rc);
											ReducedPalletChangeWrapper::new(reduced_pallet_change, pallet_a, pallet_b);
									let _ = writeln!(f, "{reduced_pallet_change_wrapper}");
								});
							}
						}
					});
				}
			}
		});

		Ok(())
	}
}

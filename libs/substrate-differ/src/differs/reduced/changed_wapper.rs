use super::{reduced_pallet::*, reduced_runtime::*};
use crate::differs::reduced::calls::prelude::PalletId;
use comparable::MapChange;
use serde::Serialize;
use std::fmt::Display;

// TODO: Rename that
#[derive(Debug, Serialize)]
pub struct CompOutput(pub(crate) ReducedRuntimeChange);

#[derive(Debug, Serialize)]
pub struct ChangedWrapper(pub(crate) CompOutput);

impl From<CompOutput> for ChangedWrapper {
	fn from(c: CompOutput) -> Self {
		Self(c)
	}
}

impl From<ReducedRuntimeChange> for CompOutput {
	fn from(rrc: ReducedRuntimeChange) -> Self {
		Self(rrc)
	}
}

pub enum Filter<T> {
	None,
	One(T),
	Some(Vec<T>),
	All,
}

#[derive(Debug)]
pub enum MapChangeFilter {
	Added,
	Changed,
	Removed,
}

pub enum ChangedFilter {
	Unchanged,
	Changed,
	All,
}

impl Display for ChangedWrapper {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.0 .0.pallets.iter().for_each(|mc: &MapChange<PalletId, ReducedPalletDesc, Vec<ReducedPalletChange>>| {
			match mc {
				MapChange::Added(key, reduced_pallet) => {
					let _ = writeln!(f, "[+] id: {:>2} - new pallet: {}", key, reduced_pallet.name);
				}
				MapChange::Removed(key) => {
					let _ = writeln!(f, "[-] {}", key);
				}

				MapChange::Changed(key, changes) => {
					let _ = writeln!(f, "[≠] pallet: {} -> {} change(s)", key, changes.len());
					changes.iter().for_each(|reduced_pallet_change| {
						let _ = writeln!(f, "{}", reduced_pallet_change);
					});
				}
			}
		});
		Ok(())
	}
}

impl AsRef<CompOutput> for ChangedWrapper {
	fn as_ref(&self) -> &CompOutput {
		&self.0
	}
}

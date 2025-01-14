use std::{collections::BTreeMap, fmt::Display};

use super::{calls::prelude::*, prelude::*};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ReducedRuntimeSummary {
	pallets: Vec<ReducedPalletSummary>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReducedPalletSummary {
	id: PalletId,
	name: String,
	calls: usize,
	events: usize,
	errors: usize,
	constants: usize,
	storages: usize,
}

impl Display for ReducedPalletSummary {
	fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			fmt,
			"{name:>32} - {id:<4}  {calls:>8} {events:>8} {errors:>8} {constants:>8} {storages:>8}",
			id = self.id,
			name = self.name,
			calls = self.calls,
			events = self.events,
			errors = self.errors,
			constants = self.constants,
			storages = self.storages,
		)
	}
}

impl Display for ReducedRuntimeSummary {
	fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let _ = writeln!(
			fmt,
			"{name:>32}   {id:<4}  {calls:>8} {events:>8} {errors:>8} {constants:>8} {storages:>8}",
			name = "NAME",
			id = "ID",
			calls = "CALLS",
			events = "EVENTS",
			errors = "ERRORS",
			constants = "CONSTANTS",
			storages = "STORAGE"
		);

		let _ =
			writeln!(fmt, "            ---------------------------------------------------------------------------",);

		let pallets = self.pallets.iter().map(|p| (p.id, p)).collect::<BTreeMap<_, _>>();
		for pallet in pallets.into_values() {
			let _ = writeln!(fmt, "{pallet}");
		}
		Ok(())
	}
}

impl From<&ReducedPallet> for ReducedPalletSummary {
	fn from(p: &ReducedPallet) -> Self {
		Self {
			id: p.index,
			name: p.name.clone(),
			calls: p.calls.len(),
			events: p.events.len(),
			errors: p.errors.len(),
			constants: p.constants.len(),
			storages: p.storages.len(),
		}
	}
}

impl From<&ReducedRuntime> for ReducedRuntimeSummary {
	fn from(reduced_runtime: &ReducedRuntime) -> Self {
		let pallets = reduced_runtime.pallets.values().map(|reduced_pallet| reduced_pallet.into()).collect();
		Self { pallets }
	}
}

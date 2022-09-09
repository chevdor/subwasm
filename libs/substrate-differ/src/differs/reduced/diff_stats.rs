use serde::Serialize;
use std::{collections::BTreeMap, fmt::Display};

use super::changed_wapper::ChangedWrapper;

#[derive(Debug, Serialize, Default)]
pub struct DiffStatsTotal {
	additions: usize,
	changes: usize,
	removals: usize,
}

impl From<ChangedWrapper> for DiffStats {
	fn from(_: ChangedWrapper) -> Self {
		todo!()
	}
}

pub type PalletIdentifier = (u32, String);

/// This struct holds a summary of the diff results.
/// It counts and aggregates the number of changes in pallets.
#[derive(Debug, Serialize, Default)]
pub struct DiffStats {
	totals: DiffStatsTotal,
	pallets: BTreeMap<PalletIdentifier, DiffStatsTotal>,
}

impl Display for DiffStats {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str("Totals:\n")?;
		f.write_fmt(format_args!("{}", self.totals))?;

		if !self.pallets.is_empty() {
			f.write_str("Pallets:\n")?;
			self.pallets.iter().for_each(|((id, name), totals)| {
				let _ = f.write_fmt(format_args!("{:?} {} : {}\n", id, name, totals));
			});
		} else {
			f.write_str("Pallets: none found")?;
		}
		Ok(())
	}
}

impl Display for DiffStatsTotal {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!(" - [+] {}\n", self.additions))?;
		f.write_fmt(format_args!(" - [-] {}\n", self.removals))?;
		f.write_fmt(format_args!(" - [≠] {}\n", self.changes))?;
		Ok(())
	}
}

#[cfg(test)]
mod test_diff_stats {
	use super::*;

	#[test]
	fn test_() {
		let stats = DiffStats::default();
		println!("stats = {:?}", stats);
		println!("{}", stats);
	}
}

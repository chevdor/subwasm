use crate::differs::reduced::calls::prelude::Index;

use super::reduced_pallet::*;
use super::reduced_runtime::*;
use comparable::{Changed, MapChange};
use std::fmt::Display;

pub type CompOutput = Changed<ReducedRuntimeChange>;

#[derive(Debug)]
pub struct ChangedWrapper(CompOutput);

impl From<CompOutput> for ChangedWrapper {
	fn from(c: CompOutput) -> Self {
		Self(c)
	}
}

impl Display for ChangedWrapper {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match &self.0 {
			Changed::Unchanged => f.write_str("UNCHANGED"),
			Changed::Changed(c) => {
				// println!("c = {:?}", c);

				c.pallets
					.iter()
					.map(|mc: &MapChange<Index, ReducedPalletDesc, Vec<ReducedPalletChange>>| match mc {
						MapChange::Added(key, reduced_pallet) => {
							f.write_fmt(format_args!("[+] id: {:>2} - new pallet: {}\n", key, reduced_pallet.name))
						}
						MapChange::Changed(key, changes) => {
							let _ = f.write_fmt(format_args!("[!] {} {} changes\n", key, changes.len()));
							changes
								.iter()
								.map(|reduced_pallet_change| match reduced_pallet_change {
									_ => f.write_fmt(format_args!("  - {:?}\n", reduced_pallet_change)),
								})
								.collect()
						}
						MapChange::Removed(key) => f.write_fmt(format_args!("[-] {}\n", key)),
					})
					.collect()
			}
		}
	}
}

impl AsRef<CompOutput> for ChangedWrapper {
	fn as_ref(&self) -> &CompOutput {
		&self.0
	}
}

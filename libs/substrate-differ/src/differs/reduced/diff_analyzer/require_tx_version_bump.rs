use log::trace;

use super::{traits::RequireTransactionVersionBump, DiffAnalyzer};
use crate::differs::reduced::reduced_runtime::ReducedRuntimeChange;

impl RequireTransactionVersionBump for DiffAnalyzer {
	fn require_tx_version_bump(&self) -> bool {
		if self.changes.0.changes.is_empty() {
			return false;
		}

		let res = self.changes.0.changes.iter().any(|change| {
			match change {
				ReducedRuntimeChange::Pallets(pallets) => pallets.iter().any(|p| match p {
					comparable::MapChange::Added(_key, _desc) => false,
					comparable::MapChange::Removed(_key) => false,
					comparable::MapChange::Changed(_key, change) => change.iter().any(|x| x.require_tx_version_bump()),
				}),
				ReducedRuntimeChange::Extrinsic(_extrinsic) => {
					eprintln!("Extrinsic diff is not implemented yet but subwasm spotted some changes.");
					eprintln!("This is normal if you compare different chains.");
					// Until implemented, we want this path to be transparent
					false
				}
				ReducedRuntimeChange::Imports(_) => false, // import changes don't affect tx format
			}
		});
		trace!("TxBump | Analyzer: {res}");
		res
	}
}

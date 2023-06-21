use log::trace;

use super::{traits::RequireTransactionVersionBump, DiffAnalyzer};
use crate::differs::reduced::reduced_runtime::ReducedRuntimeChange;

impl RequireTransactionVersionBump for DiffAnalyzer {
	fn require_tx_version_bump(&self) -> bool {
		if self.changes.0.changes.is_empty() {
			return false;
		}

		let res = self
			.changes
			.0
			.changes
			.iter()
			.map(|change| {
				match change {
					ReducedRuntimeChange::Pallets(pallets) => pallets
						.iter()
						.map(|p| match p {
							comparable::MapChange::Added(_key, _desc) => false,
							comparable::MapChange::Removed(_key) => false,
							comparable::MapChange::Changed(_key, change) => {
								change.iter().map(|x| x.require_tx_version_bump()).any(|x| x)
							}
						})
						.any(|x| x),
					ReducedRuntimeChange::Extrinsic(_extrinsic) => {
						eprintln!("Extrinsic diff is not implemented yet but subwasm spotted some changes.");
						eprintln!("This is normal if you compare different chains.");
						// todo!("Extrinsic diff not implemented yet and usually does not change")
						// 		extrinsic.iter().map(|p| match p {
						// 	ReducedExtrinsicChange::Version(version) => {
						// 		// match versiopn {
						// 		// }
						// 		true
						// 	},
						// 	ReducedExtrinsicChange::SignedExtensions(signed_extensions) => {
						// 		// match signed_extensions {
						// 			// }
						// 		true
						// 	},
						// }).any(|x| x),
						// }

						// Until implemented, we want this path to be transparent
						false
					}
				}
			})
			.any(|x| x);
		trace!("TxBump | Analyzer: {res}");
		res
	}
}

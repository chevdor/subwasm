use super::{traits::RequireTransactionVersionBump, DiffAnalyzer};
use crate::differs::reduced::reduced_runtime::ReducedRuntimeChange;

impl RequireTransactionVersionBump for DiffAnalyzer {
	fn require_tx_version_bump(&self) -> bool {
		self.changes
			.0
			.changes
			.iter()
			.map(|change| {
				match change {
					ReducedRuntimeChange::Pallets(pallets) => pallets
						.iter()
						.map(|p| match p {
							comparable::MapChange::Added(_key, _ddesc) => true,
							comparable::MapChange::Removed(_key) => false,
							comparable::MapChange::Changed(_key, change) => {
								change.iter().map(|x| x.require_tx_version_bump()).all(|x| x)
							}
						})
						.all(|x| x),
					ReducedRuntimeChange::Extrinsic(_extrinsic) => {
						todo!("Extrinsic diff not implemented yet and usually does not change")
						// 		extrinsic.iter().map(|p| match p {
						// 	ReducedExtrinsicChange::Version(version) => {
						// 		// match versiopn {

						// 		// }
						// 		// TODO
						// 		true
						// 	},
						// 	ReducedExtrinsicChange::SignedExtensions(signed_extensions) => {
						// 		// match signed_extensions {

						// 			// }
						// 		// TODO
						// 		true
						// 	},
						// }).all(|x| x),
						// }
					}
				}
			})
			.all(|x| x)
	}
}

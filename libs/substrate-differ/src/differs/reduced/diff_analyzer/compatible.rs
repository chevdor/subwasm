use super::{traits::Compatible, DiffAnalyzer};
use crate::differs::reduced::reduced_runtime::ReducedRuntimeChange;

impl Compatible for DiffAnalyzer {
	fn compatible(&self) -> bool {
		if self.changes.0.changes.is_empty() {
			return true;
		}

		self.changes
			.0
			.changes
			.iter()
			.map(|change| match change {
				ReducedRuntimeChange::Pallets(pallets) => pallets
					.iter()
					.map(|p| match p {
						comparable::MapChange::Added(_key, _desc) => true,
						comparable::MapChange::Removed(_key) => false,
						comparable::MapChange::Changed(_key, change) => {
							change.iter().map(|x| x.compatible()).all(|x| x)
						}
					})
					.all(|x| x),
				ReducedRuntimeChange::Extrinsic(_extrinsic) => {
					// TODO  todo!("Extrinsic diff not implemented yet and usually does not change")

					// Until implemented, we want this path to be transparent
					true
				}
			})
			.all(|x| x)
	}
}

use crate::differs::raw::change_type::ChangeType;

#[derive(Debug)]
pub struct DiffResult<'meta, T: PartialEq> {
	/// Define the nature of the change if there was one
	pub change_type: ChangeType,

	/// First item compared
	pub left: &'meta T,

	/// Second item compared
	pub right: &'meta T,
	// /// Some notes about the changes
	// notes: Option<Vec<String>>,
}

impl<'meta, T: PartialEq> DiffResult<'meta, T> {
	pub fn new(change_type: ChangeType, left: &'meta T, right: &'meta T) -> Self {
		Self { change_type, left, right }
	}
}

// TODO: make it for tests only
// impl<T: PartialEq> Default for DiffResult<T> {
// 	fn default() -> Self {
// 		Self {
// 			change_type: ChangeType::Unchanged,
// 			left: None,
// 			right: None,
// 			// notes: None
// 		}
// 	}
// }

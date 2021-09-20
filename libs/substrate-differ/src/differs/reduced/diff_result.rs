use crate::differs::raw::change_type::ChangeType;

pub struct DiffResult<T: 'static + PartialEq> {
	/// Define the nature of the change if there was one
	change_type: ChangeType,

	/// First item compared
	left: &'static T,

	/// Second item compared
	right: &'static T,
	// /// Some notes about the changes
	// notes: Option<Vec<String>>,
}

impl<T: PartialEq> DiffResult<T> {
	pub fn new(change_type: ChangeType, left: &'static T, right: &'static T) -> Self {
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

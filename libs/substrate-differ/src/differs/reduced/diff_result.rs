// use super::change_type::Change;
// use std::fmt::Display;

// #[derive(Debug)]
// pub struct DiffResult<'meta, T: PartialEq> {
// 	/// Define the nature of the change if there was one
// 	pub change: Change<'meta, T>,
// 	// /// First item compared
// 	// pub left: Option<&'meta T>,

// 	// /// Second item compared
// 	// pub right: Option<&'meta T>,
// 	// /// Some notes about the changes
// 	// notes: Option<Vec<String>>,
// }

// impl<'meta, T: PartialEq> DiffResult<'meta, T> {
// 	// pub fn new(change: Change<T>, left: &'meta T, right: &'meta T) -> Self {
// 	// 	// Self { change_type, Some(left), Some(right) }
// 	// 	Self { change: change }
// 	// }

// 	pub fn new(change: Change<'meta, T>) -> Self {
// 		// Self { change_type, Some(left), Some(right) }
// 		Self { change }
// 	}
// }

// // #[cfg(test)]
// // impl<T: PartialEq> Default for DiffResult<T> {
// // 	fn default() -> Self {
// // 		Self {
// // 			change_type: ChangeType::Unchanged,
// // 			left: None,
// // 			right: None,
// // 			// notes: None
// // 		}
// // 	}
// // }

// impl<'meta, T: PartialEq> Display for DiffResult<'meta, T> {
// 	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// 		f.write_fmt(format_args!("{}", self.change))
// 	}
// }

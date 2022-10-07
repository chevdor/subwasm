// use std::fmt::Display;

// /// The type of change(s) between items
// ///
// /// Warning: The [RawDiffer](crate::differs::raw::raw_differ::RawDiffer) implemented a similarly named and older version of this Enum.

// pub type ModifiedVariant<'meta, R> = (&'meta R, &'meta R);

// #[derive(Debug)]
// pub enum Change<'meta, R> {
// 	/// An item has been added
// 	Added(&'meta R),

// 	/// An item has been removed
// 	Removed(&'meta R),

// 	/// An item has been modified
// 	Modified(ModifiedVariant<'meta, R>),

// 	/// Both items are identical
// 	Unchanged,
// }

// impl<'meta, R> Display for Change<'meta, R> {
// 	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// 		let s = match self {
// 			Change::Added(_) => "Added",
// 			Change::Removed(_) => "Removed",
// 			Change::Modified(_) => "Modified",
// 			Change::Unchanged => "Unchanged",
// 		};
// 		f.write_str(s)
// 	}
// }

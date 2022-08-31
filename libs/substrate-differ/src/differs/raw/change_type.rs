/// The type of change(s) between items.
///
/// Warning: The new [ReducedDiffer](crate::differs::reduced::reduced_differ::ReducedDiffer) introduces a better version.
/// The 2 versions are currently in use but this version will likely become deprecated in the future.

#[derive(Debug)]
pub enum Change {
	/// An item has been added
	Added,

	/// An item has been removed
	Removed,

	/// An item has been modified
	Modified,

	/// Both items are identical
	Unchanged,
}

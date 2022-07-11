/// The type of change(s) between items
pub enum ChangeType {
	/// An item has been added
	Added,

	/// An item has been removed
	Removed,

	/// An item has been modified
	Modified,

	/// Both items are identical
	Unchanged,
}

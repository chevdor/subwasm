/// The type of change(s) between items
///
/// Warning: The [RawDiffer](crate::differs::raw::raw_differ::RawDiffer) implemented a similarly named and older version of this Enum.

pub type ModifiedVariant<'meta, R> = (&'meta R, &'meta R);

// TODO: Checkout https://crates.io/crates/comparable

#[derive(Debug)]
pub enum Change<'meta, R> {
	/// An item has been added
	Added(&'meta R),

	/// An item has been removed
	Removed(&'meta R),

	/// An item has been modified
	Modified(ModifiedVariant<'meta, R>),

	/// Both items are identical
	Unchanged,
}

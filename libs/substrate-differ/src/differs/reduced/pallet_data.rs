use super::{signature::Signature, Index};

/// PalletData is a a struct describing calls, events, constants, errors and storage.
/// Those are mentioned as `items` below. Items/PalletData are usually handled through
/// as a variant of `PalletItem`. The reduction however, gives them the same common struct.
#[derive(Debug)]
pub struct PalletData {
	/// The name of the Pallet Item
	pub name: String,

	/// An optionnal index, some variants of `PalletItem` don't have an index
	pub index: Option<Index>,

	/// The signature contains what is relevant and critical to the item.
	pub signature: Box<dyn Signature>,

	/// The documentation is usually not critical to the comparison
	/// of runtimes, so it is kept aside.
	pub documentation: Vec<String>,
}

impl PalletData {
	pub fn new(name: String, index: Option<Index>, signature: Box<dyn Signature>, documentation: Vec<String>) -> Self {
		Self { name, index, signature, documentation }
	}
}

impl PartialEq for PalletData {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name
			&& self.index == other.index
			&& self.signature.serialize() == other.signature.serialize()
	}
}

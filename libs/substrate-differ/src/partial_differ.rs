use frame_metadata::RuntimeMetadata;

pub struct MetadataPartialDiffer<'a> {
	r1: &'a RuntimeMetadata,
	r2: &'a RuntimeMetadata,
}

impl<'a> MetadataPartialDiffer<'a> {
	pub fn new(r1: &'a RuntimeMetadata, r2: &'a RuntimeMetadata) -> Self {
		Self { r1, r2 }
	}

	/// This is a raw comparison based on the json serialization of the metadata
	pub fn compare(&self) {
		todo!();
	}
}

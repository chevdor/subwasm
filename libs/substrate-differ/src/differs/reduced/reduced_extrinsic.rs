use comparable::Comparable;
use frame_metadata::{ExtrinsicMetadata, SignedExtensionMetadata};
use scale_info::form::PortableForm;
use serde::Serialize;

#[derive(Debug, PartialEq, Hash, Comparable, Serialize, Clone)]
pub struct ReducedExtrinsic {
	version: u8,
	signed_extensions: Vec<ReducedSignedExtension>,
}

impl ReducedExtrinsic {
	pub fn from(extrinsic: &ExtrinsicMetadata<PortableForm>) -> Self {
		let version = extrinsic.version;
		let signed_extensions = extrinsic.signed_extensions.iter().map(ReducedSignedExtension::from).collect();

		Self { version, signed_extensions }
	}
}

// TODO:
#[derive(Debug, PartialEq, Hash, Comparable, Serialize, Clone)]
pub struct ReducedSignedExtension {
	identifier: String,
	// additional_signed: Unknown,
	// type: ?
}

impl ReducedSignedExtension {
	pub fn from(e: &SignedExtensionMetadata<PortableForm>) -> Self {
		let identifier = e.identifier.clone();
		Self { identifier }
	}
}

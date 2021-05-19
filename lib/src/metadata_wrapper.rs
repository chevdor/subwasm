use frame_metadata::{
	decode_different::{DecodeDifferent, DecodeDifferentArray, DecodeDifferentStr},
	RuntimeMetadata,
};
pub struct MetadataWrapper<'a>(pub &'a RuntimeMetadata);

pub fn get_lowercase_name(name: &DecodeDifferentStr) -> Option<String> {
	let md_name = match &name {
		DecodeDifferent::Decoded(ref dec) => Some(dec.to_string()),
		_ => None,
	};
	md_name.map(|s| s.to_lowercase())
}

impl<'a> MetadataWrapper<'a> {
	/// Display a simple list of the modules
	pub fn display_modules_list(&self) {
		match &self.0 {
			RuntimeMetadata::V12(v12) => {
				let modules = match &v12.modules {
					DecodeDifferentArray::Decoded(modules) => modules,
					DecodeDifferentArray::Encode(_) => panic!("Failed decoding Metadata V12"),
				};

				modules.iter().for_each(|module| println!(" - {:02}: {:?}", module.index, module.name));
			}
			RuntimeMetadata::V13(v13) => {
				let modules = match &v13.modules {
					DecodeDifferentArray::Decoded(modules) => modules,
					DecodeDifferentArray::Encode(_) => panic!("Failed decoding Metadata V13"),
				};

				modules.iter().for_each(|module| println!(" - {:02}: {:?}", module.index, module.name));
			}
			RuntimeMetadata::V14(v14) => {
				v14.pallets.iter().for_each(|pallet| println!(" - {:?}", pallet));
			}
			_ => panic!("Runtime not supported"),
		};
	}

	/// Display a single module
	pub fn display_module(&self, name: &str) {
		match &self.0 {
			RuntimeMetadata::V12(v12) => {
				let modules = match &v12.modules {
					DecodeDifferentArray::Decoded(modules) => modules,
					DecodeDifferentArray::Encode(_) => panic!("Failed decoding Metadata V12"),
				};

				let module_metadata = modules
					.iter()
					.find(|module| {
						let name_str = get_lowercase_name(&module.name);
						name_str == Some(name.to_lowercase())
					})
					.expect("pallet not found in metadata");

				println!(" - {:02}: {:?}", module_metadata.index, module_metadata.name);
			}
			RuntimeMetadata::V13(v13) => {
				let modules = match &v13.modules {
					DecodeDifferentArray::Decoded(modules) => modules,
					DecodeDifferentArray::Encode(_) => panic!("Failed decoding Metadata V13"),
				};

				let module_metadata = modules
					.iter()
					.find(|module| module.name == DecodeDifferent::Decoded(name.to_string()))
					.expect("pallet not found in metadata");

				println!(" - {:02}: {:?}", module_metadata.index, module_metadata.name);
			}
			RuntimeMetadata::V14(v14) => {
				v14.pallets.iter().for_each(|pallet| println!(" - {:?}", pallet));
			}
			_ => panic!("Runtime not supported"),
		};
	}
}

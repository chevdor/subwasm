use frame_metadata::RuntimeMetadata;
use log::debug;

use crate::{convert::convert, display_module};
pub struct MetadataWrapper<'a>(pub &'a RuntimeMetadata);

impl<'a> MetadataWrapper<'a> {
	/// Display a simple list of the modules
	pub fn display_modules_list(&self) {
		match &self.0 {
			RuntimeMetadata::V12(v12) => {
				let modules = convert(&v12.modules);
				modules.iter().for_each(|module| println!(" - {:02}: {}", module.index, convert(&module.name)));
			}
			RuntimeMetadata::V13(v13) => {
				let modules = convert(&v13.modules);
				modules.iter().for_each(|module| println!(" - {:02}: {}", module.index, convert(&module.name)));
			}
			RuntimeMetadata::V14(v14) => {
				v14.pallets.iter().for_each(|pallet| println!(" - {:?}", pallet));
			}
			_ => panic!("Runtime not supported"),
		};
	}

	/// Display a single module
	pub fn display_single_module(&self, filter: &str) {
		debug!("metadata_wapper::display_module with filter: {:?}", filter);

		match &self.0 {
			RuntimeMetadata::V12(v12) => {
				display_module!(convert(&v12.modules), filter);
			}
			RuntimeMetadata::V13(v13) => {
				display_module!(convert(&v13.modules), filter);
			}
			RuntimeMetadata::V14(v14) => {
				v14.pallets.iter().for_each(|pallet| println!(" - {:?}", pallet));
			}
			_ => panic!("Runtime not supported"),
		};
	}
}

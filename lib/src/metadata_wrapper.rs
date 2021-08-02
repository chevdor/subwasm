use frame_metadata::RuntimeMetadata;
use log::debug;

use crate::{convert::convert, display_module, display_v14_meta};
pub struct MetadataWrapper<'a>(pub &'a RuntimeMetadata);

impl<'a> MetadataWrapper<'a> {
	/// Display a simple list of the modules.
	/// Starting with V12, modules are identified by indexes so
	/// the order they appear in the metadata no longer matters and we sort them by indexes.
	pub fn display_modules_list(&self) {
		match &self.0 {
			RuntimeMetadata::V12(v12) => {
				let mut modules = convert(&v12.modules).clone();
				modules.sort_by(|a, b| a.index.cmp(&b.index));
				modules.iter().for_each(|module| println!(" - {:02}: {}", module.index, convert(&module.name)));
			}
			RuntimeMetadata::V13(v13) => {
				let mut modules = convert(&v13.modules).clone();
				modules.sort_by(|a, b| a.index.cmp(&b.index));
				modules.iter().for_each(|module| println!(" - {:02}: {}", module.index, convert(&module.name)));
			}
			RuntimeMetadata::V14(v14) => {
				let mut pallets = v14.pallets.clone();
				// pallets.sort_by(|a,b| a.index.cmp(&b.index));
				pallets.sort_by_key(|p| p.index);
				pallets.iter().for_each(|pallet| println!(" - {:02}: {}", pallet.index, pallet.name));
			}
			_ => panic!("Runtime not supported. Subwasm supports V12 and above."),
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
				let meta = v14
					.pallets
					.iter()
					.find(|pallet| {
						let name_str = pallet.name.to_lowercase();
						name_str == filter.to_lowercase()
					})
					.expect("pallet not found in metadata");

				println!("Module {:02}: {}", meta.index, &meta.name);

				println!("🤙 Calls:");
				display_v14_meta!(v14, meta, calls);

				println!("📢 Events:");
				display_v14_meta!(v14, meta, event);

				println!("⛔️ Errors:");
				display_v14_meta!(v14, meta, error);

				println!("📦 Storage:");
				if let Some(meta) = &meta.storage {
					for entry in &meta.entries {
						println!("- {}", entry.name);
					}
				}

				println!("💎 Constants:");
				for item in &meta.constants {
					println!("- {}", item.name);
				}
			}
			_ => panic!("Runtime not supported"),
		};
	}
}

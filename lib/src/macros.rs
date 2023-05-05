#[macro_export]
macro_rules! display_module {
	($modules: expr, $filter: ident) => {
		let meta = $modules
			.iter()
			.find(|module| {
				let name_str = convert(&module.name).to_lowercase();
				name_str == $filter.to_lowercase()
			})
			.expect("pallet not found in metadata");

		println!("Module {:02}: {}", meta.index, convert(&meta.name));

		println!("🤙 Calls:");
		if let Some(item) = meta.calls.as_ref() {
			let calls = convert(&item);
			for call in calls {
				println!("  - {}", convert(&call.name));
			}
		}

		println!("📢 Events:");
		if let Some(item) = meta.event.as_ref() {
			let events = convert(&item);
			for event in events {
				println!("  - {}", convert(&event.name));
			}
		}
	};
}

#[macro_export]
macro_rules! display_v14_meta {
	($v14: expr, $meta: expr, $type: ident) => {
		if let Some(metadata) = &$meta.$type {
			let type_id = metadata.ty.id;
			// log::debug!("type_id: {:?}", type_id);
			let registry = &$v14.types;

			let type_info = registry.resolve(type_id).unwrap();
			match &type_info.type_def {
				scale_info::TypeDef::Variant(v) => {
					for variant in &v.variants {
						println!("- {:?}: {}", variant.index, variant.name);
					}
				}
				o => panic!("Unsupported variant: {:?}", o),
			}
		}
	};
}

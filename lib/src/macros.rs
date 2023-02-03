#[macro_export]
macro_rules! write_module {
	($modules: expr, $filter: ident, $out: ident) => {
		let meta = $modules
			.iter()
			.find(|module| {
				let name_str = convert(&module.name).to_lowercase();
				name_str == $filter.to_lowercase()
			})
			.expect("pallet not found in metadata");

		write!($out, "Module {:02}: {}\n", meta.index, convert(&meta.name));

		write!($out, "🤙 Calls:\n");
		if let Some(item) = meta.calls.as_ref() {
			let calls = convert(&item);
			for call in calls {
				write!($out, "  - {}\n", convert(&call.name));
			}
		}

		write!($out, "📢 Events:\n");
		if let Some(item) = meta.event.as_ref() {
			let events = convert(&item);
			for event in events {
				write!($out, "  - {}\n", convert(&event.name));
			}
		}
	};
}

#[macro_export]
macro_rules! write_v14_meta {
	($v14: expr, $meta: expr, $type: ident, $out: ident) => {
		if let Some(metadata) = &$meta.$type {
			let type_id = metadata.ty.id();
			// log::debug!("type_id: {:?}", type_id);
			let registry = &$v14.types;

			let type_info = registry.resolve(type_id).unwrap();
			match type_info.type_def() {
				scale_info::TypeDef::Variant(v) => {
					for variant in v.variants() {
						write!($out, "- {:?}: {}\n", variant.index(), variant.name());
					}
				}
				o => panic!("Unsupported variant: {:?}", o),
			}
		}
	};
}

#[macro_export]
macro_rules! display_module {
	($modules: expr, $filter: ident) => {
		$crate::write_module!($modules, $filter, std::io::stdout());
	};
}

#[macro_export]
macro_rules! display_v14_meta {
	($v14: expr, $meta: expr, $type: ident) => {
		$crate::write_v14_meta!($v14, $meta, $type, std::io::stdout());
	};
}

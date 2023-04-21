#[macro_export]
macro_rules! write_module {
	($modules: expr, $filter: ident, $out: ident) => {
		|| -> color_eyre::Result<()> {
			use color_eyre::eyre::eyre;

			let meta = $modules
				.iter()
				.find(|module| {
					let name_str = convert(&module.name).to_lowercase();
					name_str == $filter.to_lowercase()
				})
				.ok_or_else(|| eyre!("pallet not found in metadata"))?;

			writeln!($out, "Module {:02}: {}", meta.index, convert(&meta.name))?;

			writeln!($out, "🤙 Calls:")?;
			if let Some(item) = meta.calls.as_ref() {
				let calls = convert(&item);
				for call in calls {
					writeln!($out, "  - {}", convert(&call.name))?;
				}
			}

			writeln!($out, "📢 Events:")?;
			if let Some(item) = meta.event.as_ref() {
				let events = convert(&item);
				for event in events {
					writeln!($out, "  - {}", convert(&event.name))?;
				}
			}
			Ok(())
		}()?
	};
}

#[macro_export]
macro_rules! write_v14_meta {
	($v14: expr, $meta: expr, $type: ident, $out: ident) => {
		|| -> color_eyre::Result<()> {
			use color_eyre::eyre::eyre;

			if let Some(metadata) = &$meta.$type {
				let type_id = metadata.ty.id;
				// log::debug!("type_id: {:?}", type_id);
				let registry = &$v14.types;

				let type_info = registry.resolve(type_id).unwrap();
				match &type_info.type_def {
					scale_info::TypeDef::Variant(v) => {
						for variant in &v.variants {
							write!($out, "- {:?}: {}\n", variant.index, variant.name)?;
						}
					}
					o => return Err(eyre!("Unsupported variant: {:?}", o)),
				}
			} else {
				return Err(eyre!("No metadata found\n"));
			}
			Ok(())
		}()?
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

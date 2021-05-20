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

		println!("📦 {:02}: {}", meta.index, convert(&meta.name));

		println!("🤙 Calls:");
		if let Some(item) = meta.calls.as_ref() {
			let calls = convert(&item);
			for call in calls {
				println!("  - {}", convert(&call.name));
			}
		}

		println!("⚡️ Events:");
		if let Some(item) = meta.event.as_ref() {
			let events = convert(&item);
			for event in events {
				println!("  - {}", convert(&event.name));
			}
		}
	};
}

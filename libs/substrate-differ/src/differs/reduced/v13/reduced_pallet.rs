impl From<&v13::ModuleMetadata> for ReducedPallet {
	fn from(v13: &v13::ModuleMetadata) -> Self {
		let index = v13.index.into();
		let name = convert(&v13.name).to_string();
		let mut items: Vec<PalletItem> = Vec::new();

		// Calls
		let calls = match &v13.calls.as_ref() {
			Some(items) => {
				let pallet_items: Vec<PalletItem> = convert(items).iter().map(|c| c.into()).collect();
				Some(pallet_items)
			}
			None => None,
		};

		if let Some(mut c) = calls {
			// println!("calls = {:?}", c.len());
			items.append(&mut c);
		}
		// Events
		let events = match &v13.event.as_ref() {
			Some(items) => {
				let pallet_items: Vec<PalletItem> = convert(items).iter().map(|c| c.into()).collect();
				Some(pallet_items)
			}
			None => None,
		};

		if let Some(mut c) = events {
			// println!("events = {:?}", c.len());
			items.append(&mut c);
		}

		// Storage
		let storage = match &v13.storage.as_ref() {
			Some(items) => {
				// let pallet_items: Vec<PalletItem> = convert(items).iter().map(|c| c.into()).collect();
				let pallet_items: Vec<PalletItem> = convert(&convert(items).entries).iter().map(|c| c.into()).collect();
				Some(pallet_items)
			}
			None => None,
		};

		if let Some(mut c) = storage {
			// println!("storage = {:?}", c.len());
			items.append(&mut c);
		}

		// Errors
		let mut errors: Vec<PalletItem> = convert(&v13.errors).iter().map(|c| c.into()).collect();
		// println!("errors = {:?}", errors.len());
		items.append(&mut errors);

		// Constants
		let mut constants: Vec<PalletItem> = convert(&v13.constants).iter().map(|c| c.into()).collect();
		// println!("constants = {:?}", constants.len());
		items.append(&mut constants);

		// let items = if items.is_empty() { None } else { Some(items) };

		Self { index, name, items }
	}
}

use color_eyre::eyre;
use frame_metadata::{v12, RuntimeMetadata, RuntimeMetadataPrefixed}; // TODO checkout v13
use num_format::{Locale, ToFormattedString};
use std::io::prelude::*;
use std::path::Path;
use std::{fs::File, path::PathBuf};
use wasm_loader::{BlockRef, NodeEndpoint, OnchainBlock, Source};

/// Prints magic and version from a raw buffer
pub fn print_magic_and_version(data: &[u8]) {
	let is_substrate_wasm = wasm_testbed::WasmTestBed::is_substrate_wasm(data);
	let version = wasm_testbed::WasmTestBed::get_metadata_version(data);

	println!("✨ Magic number found: {}", if is_substrate_wasm { "YES" } else { "NO" });
	println!("#️⃣ Extracted version : V{:?}", version);
}

/// Display all the metadata or a part of it for a given pallet
pub fn display_metadata(metadata: RuntimeMetadataPrefixed) -> color_eyre::Result<()> {
	let pallet_filter: Option<String> = Some("Identity".to_string());
	// let pallet_filter: Option<String> = None;

	let serialized = if let Some(ref pallet) = pallet_filter {
		match metadata.1 {
			RuntimeMetadata::V12(v12) => {
				let modules = match v12.modules {
					v12::DecodeDifferentArray::Decoded(modules) => modules,
					v12::DecodeDifferentArray::Encode(_) => return Err(eyre::eyre!("Metadata should be Decoded")),
				};
				let pallet_metadata = modules
					.iter()
					.find(|module| module.name == v12::DecodeDifferent::Decoded(pallet.into()))
					.ok_or_else(|| eyre::eyre!("pallet not found in metadata"))?;
				serde_json::to_string_pretty(&pallet_metadata)?
			}
			RuntimeMetadata::V13(_v13) => {
				// let pallet = v13
				// 	.modules
				// 	.iter()
				// 	.find(|m| &m.name == pallet)
				// 	.ok_or_else(|| eyre::eyre!("pallet not found in metadata"))?;
				// serde_json::to_string_pretty(&pallet)?
				todo!("Not yet implemented");
			}
			_ => return Err(eyre::eyre!("Unsupported metadata version")),
		}
	} else {
		serde_json::to_string_pretty(&metadata)?
	};
	println!("{}", serialized);
	Ok(())
}

pub fn display_raw_metadata(metadata: &RuntimeMetadata) -> color_eyre::Result<()> {
	let pallet_filter: Option<String> = None;

	let serialized = if let Some(ref pallet) = pallet_filter {
		match metadata {
			RuntimeMetadata::V12(v12) => {
				let modules = match &v12.modules {
					v12::DecodeDifferentArray::Decoded(modules) => modules,
					v12::DecodeDifferentArray::Encode(_) => return Err(eyre::eyre!("Metadata should be Decoded")),
				};
				let pallet_metadata = modules
					.iter()
					.find(|module| module.name == v12::DecodeDifferent::Decoded(pallet.into()))
					.ok_or_else(|| eyre::eyre!("pallet not found in metadata"))?;
				serde_json::to_string_pretty(&pallet_metadata)?
			}
			// RuntimeMetadata::V13(v13) => {
			// 	let pallet = v13
			// 		.modules
			// 		.iter()
			// 		.find(|m| &m.name == pallet)
			// 		.ok_or_else(|| eyre::eyre!("pallet not found in metadata"))?;
			// 	serde_json::to_string_pretty(&pallet)?
			// }
			_ => return Err(eyre::eyre!("Unsupported metadata version")),
		}
	} else {
		serde_json::to_string_pretty(&metadata)?
	};
	println!("{}", serialized);
	Ok(())
}

pub fn display_infos(metadata: &RuntimeMetadataPrefixed) -> color_eyre::Result<()> {
	match &metadata.1 {
		RuntimeMetadata::V12(_v12) => {
			println!("Detected Substrate Runtime V12");
		}
		RuntimeMetadata::V13(_v13) => {
			println!("Detected Substrate Runtime V13");
		}
		_ => return Err(eyre::eyre!("Unsupported metadata version")),
	};
	Ok(())
}

pub fn display_modules_list(metadata: &RuntimeMetadataPrefixed) -> color_eyre::Result<()> {
	match &metadata.1 {
		RuntimeMetadata::V12(v12) => {
			let modules = match &v12.modules {
				v12::DecodeDifferentArray::Decoded(modules) => modules,
				v12::DecodeDifferentArray::Encode(_) => return Err(eyre::eyre!("Metadata should be Decoded")),
			};

			modules.iter().for_each(|module| println!(" - {:02}: {:?}", module.index, module.name));
		}
		RuntimeMetadata::V13(_v13) => {
			// let _pallet = v13.modules.iter().inspect(|module| println!(" - {:?}{:?}", module.index, module.name));
			// .find(|m| &m.name == pallet)
			todo!("Not yet implemented");
		}
		_ => return Err(eyre::eyre!("Unsupported metadata version")),
	}
	Ok(())
}

/// Fetch the runtime from a node and store the wasm locally
pub fn download_runtime(url: &str, block_ref: Option<BlockRef>, output: Option<PathBuf>) -> color_eyre::Result<()> {
	let url = match url {
		url if url.starts_with("ws") => NodeEndpoint::WebSocket(url.to_string()),
		url if url.starts_with("http") => NodeEndpoint::Http(url.to_string()),
		_ => panic!("The url should either start with http or ws"),
	};

	let reference = OnchainBlock { endpoint: url, block_ref };
	let wasm = wasm_loader::WasmLoader::fetch_wasm(&reference).expect("Getting wasm from the node");
	println!("Got the runtime, its size is {:?}", wasm.len());

	let outfile = match output {
		Some(path) => path,

		// TODO: now that we have it, we could use runtime_<name>_<version>.wasm after querying the version
		_ => {
			let mut i = 0;
			let mut path;

			loop {
				path = format!("runtime_{:03?}.wasm", i);
				i += 1;
				assert!(i < 1000, "Ran out of indexes");
				if !Path::new(&path).exists() {
					break;
				}
			}
			PathBuf::from(path)
		}
	};

	println!("Saving runtime to {:?}", outfile);
	let mut buffer = File::create(outfile)?;
	buffer.write_all(&wasm)?;
	println!("Done");
	Ok(())
}

pub fn print_runtime_infos(src: Source) {
	let sizes = |x| -> (f32, usize) { (x as f32 / 1024.0 / 1024.0, x) };

	println!("⏱️  Loading WASM from {:?}", src);
	let runtime_a = wasm_testbed::WasmTestBed::new(&src).expect("Failed loading runtime");

	// RUNTIME SIZE
	let size = runtime_a.size();

	println!("🏋️  Runtime Size:\t{:.3?} MB ({} bytes)", sizes(size).0, sizes(size).1.to_formatted_string(&Locale::en));

	// METADATA VERSION
	let metadata_a_version = runtime_a.metadata_version();
	println!("🎁 Metadata version:\tV{:?}", metadata_a_version);

	// CORE VERSIONS
	let version_a = runtime_a.core_version().as_ref().expect("Some version");
	println!("🔥 Core version:\t{}", version_a);

	println!("🗳️  Proposal hash:\t{}", runtime_a.proposal_hash());
}

/// Compare 2 runtimes. It compares their versions first
/// then their metata.
pub fn diff(src_a: Source, src_b: Source) {
	let size = |x| -> (f32, usize) { (x as f32 / 1024.0 / 1024.0, x) };

	println!("Loading WASM runtimes:");
	println!("  🅰️  {:?}", src_a);
	let runtime_a = wasm_testbed::WasmTestBed::new(&src_a).expect("Can only diff if the 2 runtimes can load");
	println!("  🅱️  {:?}", src_b);
	let runtime_b = wasm_testbed::WasmTestBed::new(&src_b).expect("Can only diff if the 2 runtimes can load");

	// RUNTIME SIZE
	let size_a = runtime_a.size();
	let size_b = runtime_b.size();

	println!("Checking runtime sizes:");
	if size_a == size_b {
		println!(
			"  ✅  Both size are identical: {:.3?} MB ({} bytes)",
			size(size_a).0,
			size(size_a).1.to_formatted_string(&Locale::en)
		);
	} else {
		println!("  🅰️  {:.3?} MB ({} bytes)", size(size_a).0, size(size_a).1.to_formatted_string(&Locale::en));
		println!("  🅱️  {:.3?} MB ({} bytes)", size(size_b).0, size(size_b).1.to_formatted_string(&Locale::en));
	}

	// METADATA VERSIONS
	let metadata_a_version = runtime_a.metadata_version();
	let metadata_b_version = runtime_b.metadata_version();
	println!("Checking metadata versions:");
	if metadata_a_version == metadata_b_version {
		println!("  ✅ Both metadata versions are identical: V{:?}", metadata_a_version);
	} else {
		println!("Found different metadata versions:");
		println!("  🅰️  V{:?}", metadata_a_version);
		println!("  🅱️  V{:?}", metadata_b_version);
	}

	// CORE VERSIONS
	println!("Checking core versions:");
	let version_a = runtime_a.core_version().as_ref().expect("Some version");
	let version_b = runtime_b.core_version().as_ref().expect("Some version");

	if version_a == version_b {
		print!("  ✅  The 2 core versions are identical: ");
		println!("{}", version_a);
	} else {
		println!("  ❌ The 2 core versions are different: ");
		// println!("{:#?}", version_a);
		println!("  🅰️  {}", version_a);
		// println!("{:#?}", version_b);
		println!("  🅱️  {}", version_b);
	}

	println!("Checking runtime metadata:");
	let metadata_a = runtime_a.metadata();
	let metadata_b = runtime_b.metadata();

	if metadata_a == metadata_b {
		// println!("  {}", version_a);
		println!("  ✅  The metadata are identical");
	} else {
		println!("  ❌  The metadata are different");
	}
}

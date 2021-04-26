#![feature(external_doc)]
#![doc(include = "../README.md")]

use color_eyre::eyre;
use frame_metadata_subsee::{v12, RuntimeMetadata, RuntimeMetadataPrefixed}; // TODO checkout v13
use rpc_client::*;
use std::io::prelude::*;
use std::path::Path;
use std::{fs::File, path::PathBuf};

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
			RuntimeMetadata::V13(v13) => {
				let pallet = v13
					.modules
					.iter()
					.find(|m| &m.name == pallet)
					.ok_or_else(|| eyre::eyre!("pallet not found in metadata"))?;
				serde_json::to_string_pretty(&pallet)?
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
			println!("Detected runtime V12.");
		}
		RuntimeMetadata::V13(_v13) => {
			println!("Detected runtime V13.");
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
		RuntimeMetadata::V13(v13) => {
			let _pallet = v13.modules.iter().inspect(|module| println!(" - {:?}{:?}", module.index, module.name));
			// .find(|m| &m.name == pallet)
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

	let reference = OnchainBlock { url, block_ref };
	let wasm = RpcClient::fetch_wasm(reference).expect("Getting wasm from the node");
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

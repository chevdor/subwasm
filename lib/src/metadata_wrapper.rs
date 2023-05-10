use std::io::Write;

use color_eyre::eyre::eyre;
use frame_metadata::{RuntimeMetadata, RuntimeMetadataPrefixed};
use log::debug;
use scale_info::scale::Encode;

use crate::{convert::convert, write_module, write_v14_meta};

/// The output format for the metadata
#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
	/// Output the metadata in a human readable format
	Human,
	/// Output the metadata in json format
	Json,
	/// Output the metadata in raw scale format
	Scale,
	/// Output the metadata in hex encoded scale format
	HexScale,
	/// Output the metadata in a json object containing the hex encoded scale encoding of the metadata
	JsonScale,
}

impl<S: AsRef<str>> From<S> for OutputFormat {
	fn from(s: S) -> Self {
		match s.as_ref() {
			"human" => OutputFormat::Human,
			"json" => OutputFormat::Json,
			"scale" => OutputFormat::Scale,
			"hex+scale" | "scale+hex" => OutputFormat::HexScale,
			"json+scale" | "scale+json" => OutputFormat::JsonScale,
			_ => panic!("Unknown output format"),
		}
	}
}

pub struct MetadataWrapper<'a>(pub &'a RuntimeMetadataPrefixed);

impl<'a> MetadataWrapper<'a> {
	pub fn write<O: Write>(&self, fmt: OutputFormat, filter: Option<String>, out: &mut O) -> color_eyre::Result<()> {
		debug!("Writing metadata: fmt={:?}, filter={:?}", fmt, filter);

		match fmt {
			OutputFormat::Human => {
				if let Some(filter) = filter {
					self.write_single_module(&filter, out)?;
				} else {
					self.write_modules_list(out)?;
				}
			}
			OutputFormat::Json => {
				if filter.is_some() {
					return Err(eyre!("Cannot filter metadata in json format"));
				} else {
					serde_json::to_writer_pretty(out, &self.0)?;
				}
			}
			OutputFormat::Scale => {
				if filter.is_some() {
					return Err(eyre!("Cannot filter metadata in scale format"));
				} else {
					out.write_all(&self.0.encode())?;
				}
			}
			OutputFormat::HexScale => {
				if filter.is_some() {
					return Err(eyre!("Cannot filter metadata in hex+scale format"));
				} else {
					let encoded = self.0.encode();
					write!(out, "0x{}", hex::encode(encoded))?;
				}
			}
			OutputFormat::JsonScale => {
				if filter.is_some() {
					return Err(eyre!("Cannot filter metadata in json+scale format"));
				} else {
					let encoded = self.0.encode();
					let hex = format!("0x{}", hex::encode(encoded));
					let json = serde_json::to_string_pretty(&serde_json::json!({ "result": hex }))?;
					write!(out, "{json}")?;
				}
			}
		}
		Ok(())
	}

	/// Display a simple list of the modules.
	/// Starting with V12, modules are identified by indexes so
	/// the order they appear in the metadata no longer matters and we sort them by indexes.
	pub fn write_modules_list<O: Write>(&self, out: &mut O) -> color_eyre::Result<()> {
		match &self.0 .1 {
			RuntimeMetadata::V12(v12) => {
				let mut modules = convert(&v12.modules).clone();
				modules.sort_by(|a, b| a.index.cmp(&b.index));
				modules.iter().try_for_each(|module| -> std::io::Result<()> {
					writeln!(out, " - {:02}: {}", module.index, convert(&module.name))
				})?;
			}
			RuntimeMetadata::V13(v13) => {
				let mut modules = convert(&v13.modules).clone();
				modules.sort_by(|a, b| a.index.cmp(&b.index));
				modules.iter().try_for_each(|module| -> std::io::Result<()> {
					writeln!(out, " - {:02}: {}", module.index, convert(&module.name))
				})?;
			}
			RuntimeMetadata::V14(v14) => {
				let mut pallets = v14.pallets.clone();
				// pallets.sort_by(|a,b| a.index.cmp(&b.index));
				pallets.sort_by_key(|p| p.index);
				pallets.iter().try_for_each(|pallet| -> std::io::Result<()> {
					writeln!(out, " - {:02}: {}", pallet.index, pallet.name)
				})?;
			}
			_ => return Err(eyre!("Runtime not supported. Subwasm supports V12 and above.")),
		};
		Ok(())
	}

	/// Display a single module
	pub fn write_single_module<O: Write>(&self, filter: &str, out: &mut O) -> color_eyre::Result<()> {
		debug!("metadata_wapper::write_module with filter: {:?}", filter);

		match &self.0 .1 {
			RuntimeMetadata::V12(v12) => {
				write_module!(convert(&v12.modules), filter, out);
			}
			RuntimeMetadata::V13(v13) => {
				write_module!(convert(&v13.modules), filter, out);
			}
			RuntimeMetadata::V14(v14) => {
				let meta = v14
					.pallets
					.iter()
					.find(|pallet| {
						let name_str = pallet.name.to_lowercase();
						name_str == filter.to_lowercase()
					})
					.ok_or_else(|| eyre!("Pallet not found in metadata"))?;

				writeln!(out, "Module {:02}: {}", meta.index, &meta.name)?;

				writeln!(out, "🤙 Calls:")?;
				write_v14_meta!(v14, meta, calls, out);

				writeln!(out, "📢 Events:")?;
				write_v14_meta!(v14, meta, event, out);

				writeln!(out, "⛔️ Errors:")?;
				write_v14_meta!(v14, meta, error, out);

				writeln!(out, "📦 Storage:")?;
				if let Some(meta) = &meta.storage {
					for entry in &meta.entries {
						writeln!(out, "- {}", entry.name)?;
					}
				}

				writeln!(out, "💎 Constants:")?;
				for item in &meta.constants {
					writeln!(out, "- {}", item.name)?;
				}
			}
			_ => return Err(eyre!("Runtime not supported")),
		};
		Ok(())
	}
}

use crate::{convert::convert, error, utils::print_big_output_safe, write_module, write_v14_meta};
use frame_metadata::{RuntimeMetadata, RuntimeMetadataPrefixed};
use log::debug;
use scale_info::scale::Encode;
use std::io::Write;

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
	/// Returns a ref to the inner `RuntimeMetadataPrefixed`
	pub fn runtime_metadata_prefixed(&self) -> &RuntimeMetadataPrefixed {
		self.0
	}

	/// Returns a ref to the inner `RuntimeMetadata`
	pub fn runtime_metadata(&self) -> &RuntimeMetadata {
		&self.0 .1
	}

	pub fn write<O: Write>(&self, fmt: OutputFormat, filter: Option<String>, out: &mut O) -> error::Result<()> {
		debug!("Writing metadata: fmt={:?}, filter={:?}", fmt, filter);

		match fmt {
			OutputFormat::Human => {
				if let Some(filter) = filter {
					self.write_single_module(&filter, out)?;
				} else {
					self.write_modules_list(out)?;
				}
			}

			OutputFormat::Json if filter.is_none() => {
				let serialized = serde_json::to_string_pretty(self.runtime_metadata())?;
				let _ = print_big_output_safe(&serialized);
			}

			OutputFormat::Scale if filter.is_none() => {
				match out.write_all(&self.runtime_metadata_prefixed().encode()) {
					Ok(_) => {}
					Err(_) => {} // Silence broken pipe errors
				};
			}

			OutputFormat::HexScale if filter.is_none() => {
				let encoded = self.runtime_metadata_prefixed().encode();
				let hexscale = format!("0x{}", hex::encode(encoded));
				let _ = print_big_output_safe(&hexscale);
			}

			OutputFormat::JsonScale if filter.is_none() => {
				let encoded = self.runtime_metadata_prefixed().encode();
				let hex = format!("0x{}", hex::encode(encoded));
				let serialized = serde_json::to_string_pretty(&serde_json::json!({ "result": hex }))?;
				let _ = print_big_output_safe(&serialized);
			}

			OutputFormat::Json | OutputFormat::Scale | OutputFormat::HexScale | OutputFormat::JsonScale
				if filter.is_some() =>
			{
				return Err(error::SubwasmLibError::UnsupportedFilter());
			}
			_ => unreachable!(),
		}
		Ok(())
	}

	/// Display a simple list of the modules.
	/// Starting with V12, modules are identified by indexes so
	/// the order they appear in the metadata no longer matters and we sort them by indexes.
	pub fn write_modules_list<O: Write>(&self, out: &mut O) -> error::Result<()> {
		match self.runtime_metadata() {
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
			_ => return Err(error::SubwasmLibError::UnsupportedRuntimeVersion()),
		};
		Ok(())
	}

	/// Display a single module
	pub fn write_single_module<O: Write>(&self, filter: &str, out: &mut O) -> error::Result<()> {
		debug!("metadata_wapper::write_module with filter: {:?}", filter);

		match &self.runtime_metadata() {
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
					.ok_or_else(|| error::SubwasmLibError::PalletNotFound(filter.to_string()))?;

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
			_ => return Err(error::SubwasmLibError::UnsupportedRuntimeVersion()),
		};
		Ok(())
	}
}

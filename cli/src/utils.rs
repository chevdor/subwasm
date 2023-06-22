//! Utils for the main cli
use log::debug;
use std::path::PathBuf;
use subwasmlib::{fetch_at_url, source::Source, ChainInfo};
use url::Url;
use wasm_loader::BlockRef;

use crate::error;

/// Depending on the options passed by the user we select and return the URL
pub fn select_url(gh_url: Option<Url>, dl_url: Option<Url>) -> Option<Url> {
	match (gh_url, dl_url) {
		(None, Some(u)) => Some(u),
		(Some(u), None) => Some(u),
		_ => None,
	}
}

/// Retrieve one unique source from all the options the user may pass
pub fn get_source(
	file: Option<PathBuf>,
	chain: Option<ChainInfo>,
	block: Option<BlockRef>,
	dl_url: Option<Url>,
) -> error::Result<Source> {
	let source: Source = Source::from_options(file, chain, block, dl_url)?;
	// If the source is a URL, we try to fetch it first

	Ok(match source {
		Source::URL(u) => {
			debug!("Fetching runtime from {}", u);
			let runtime_file = fetch_at_url(u, None)?;
			debug!("Runtime fetched at {:?}", runtime_file.display());
			Source::File(runtime_file)
		}
		s => s,
	})
}

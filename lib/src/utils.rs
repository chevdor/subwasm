use log::debug;
use std::{
	fs,
	path::{Path, PathBuf},
};
use url::Url;
use uuid::Uuid;

use crate::error::{self, *};

// TODO: Once the lined issue is fixed, we can remove the dependency on calm_io
/// There is a bug caused by printing big output to stdout.
///
/// We need to take extra precautions due to the following bug:
/// https://github.com/rust-lang/rust/issues/46016
pub fn print_big_output_safe(s: &str) -> Result<()> {
	// The following fails if piped to another command that truncates the output.
	// println!("{}", s);
	// Typical use cases here are:
	// 	- subwasm meta --chain westend --json | head
	// 	- subwasm meta --chain westend --json | less
	match calm_io::stdoutln!("{}", s) {
		Ok(_) => Ok(()),
		Err(e) => match e.kind() {
			std::io::ErrorKind::BrokenPipe => Ok(()),
			_ => Err(SubwasmLibError::Io),
		},
	}
}

/// Create a `subwasm` folder in a temp directory then
/// generate a temp file name and return it
pub fn get_output_file_tmp() -> Result<PathBuf> {
	let mut target = std::env::temp_dir();
	target.push("subwasm");
	let _ = fs::create_dir(&target);
	let tmp_file_name = format!("{}.wasm", Uuid::new_v4());
	target.push(tmp_file_name);
	Ok(target)
}

/// Use the user's wish if any or make up a target
/// to store the file in the current folder.
///
/// Unless the user provided a `wish`, the generated file will be
/// in the form `runtime_xxx.wasm` where `xxx` is an incrementing number
pub fn get_output_file_local(wish: Option<PathBuf>) -> PathBuf {
	match wish {
		Some(path) => path,

		_ => {
			let mut i = 0;
			let mut path;

			loop {
				path = format!("runtime_{i:03?}.wasm");
				i += 1;
				assert!(i < 1000, "Ran out of indexes");
				if !Path::new(&path).exists() {
					break;
				}
			}
			PathBuf::from(path)
		}
	}
}

/// Given a url for a runtime, attempt to fetch the runtime
/// into a file and provide the path back.
/// If you provide None as `target`, a tmp file will be generated
/// If you want to get a runtime from a RPC node, use `download_runtime`.
pub fn fetch_at_url(url: Url, target: Option<PathBuf>) -> Result<PathBuf> {
	debug!("Fetching from {url}");
	let target = if let Some(target) = target { target } else { get_output_file_tmp()? };

	let mut resp = reqwest::blocking::get(url.to_owned())
		.map_err(|_e| error::SubwasmLibError::Generic("Request error".to_string()))?;
	if resp.status().is_success() {
		let mut out = std::fs::File::create(target.clone()).map_err(|_e| error::SubwasmLibError::Io)?;
		resp.copy_to(&mut out).map_err(|_e| error::SubwasmLibError::Io)?;
		Ok(target)
	} else {
		Err(SubwasmLibError::Generic(format!("Failed fetching url at {url}")))
	}
}

/// This helper is rather lose... If it is wrong, parsing the wasm
/// will fail later.
/// It tries fetching the content from a URL then does some guesswork to
/// determine if what we got could be a wasm file.
pub fn is_wasm_from_url(url: &Url) -> Result<bool> {
	let resp = reqwest::blocking::get(url.to_owned()).map_err(|_e| error::SubwasmLibError::Io)?;

	if !resp.status().is_success() {
		debug!("Error while trying to fetch runtime at {url}");
		return Ok(false);
	}

	// We may not always get the length, but if we do, it can save us from
	// having to fetch the bytes
	if let Some(length) = resp.content_length() {
		// We consider anything less than 500kb unlikely to be a valid runtime
		debug!("The data we got from {url} is {length} bytes long");
		return Ok(length >= 500_000);
	}

	let bytes = resp.bytes();
	if bytes.is_err() {
		return Ok(false);
	}

	if let Ok(data) = bytes {
		if data.len() < 500_000 {
			return Ok(false);
		}

		return Ok(true);
	}

	Err(SubwasmLibError::NoRuntimeAtUrl(url.to_owned()))
}

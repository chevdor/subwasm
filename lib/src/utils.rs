use std::path::PathBuf;
use url::Url;

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

/// Given a url for a runtime, attempt to fetch the runtime
/// into a temp file and provide the path back
pub fn fetch_at_url(url: Url) -> Result<PathBuf> {
	let mut tmp = std::env::temp_dir();
	tmp.push("runtime.wasm");

	let mut resp = reqwest::blocking::get(url).map_err(|_e| error::SubwasmLibError::Io)?;
	let mut out = std::fs::File::create(tmp.clone()).map_err(|_e| error::SubwasmLibError::Io)?;
	resp.copy_to(&mut out).map_err(|_e| error::SubwasmLibError::Io)?;
	Ok(tmp)
}

/// 0.9.42
pub type Version = String;

/// 9420
pub type RuntimeVersion = String;

// TODO: Move that to release crate
pub fn get_runtime_version(v: &Version) -> RuntimeVersion {
	let mut res = format!("{}0", v.replace(".", ""));
	let _zero = res.remove(0);
	res
}

/// Extract runtime and version from <runtime>@<version>
pub fn gh_to_runtime_and_version(gh: &str) -> Result<(String, Version)> {
	let mut parts = gh.split('@');
	if parts.clone().count() != 2 {
		return Err(SubwasmLibError::Generic(
			"Unsupported Github version format, should be <runtime>@<version>".to_string(),
		));
	} else {
		let runtime = parts.next().expect("We did not get the expected 2 parts").to_string();
		let version = parts.next().expect("We did not get the expected 2 parts").to_string();
		Ok((runtime, version))
	}
}

/// There is no garanty that the URL will lead somewhere...
pub fn get_github_artifact_url(runtime_name: String, version: Version) -> Url {
	let runtime_version = get_runtime_version(&version);
	let url = format!("https://github.com/paritytech/polkadot/releases/download/v{version}/{runtime_name}_runtime-v{runtime_version}.compact.compressed.wasm");
	Url::parse(&url).expect("Url should parse")
}

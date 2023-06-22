use crate::SubwasmLibError;
use semver::Version;
use std::{fmt::Display, str::FromStr};
use url::Url;

/// 9420
pub type RuntimeVersion = String;

#[derive(Debug, PartialEq, Clone)]
pub struct GithubRef {
	runtime: String,
	version: Version,
}

impl Display for GithubRef {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("{}@{}", self.runtime, self.version.to_string()))
	}
}

impl GithubRef {
	/// There is no garanty that the URL will lead somewhere...
	pub fn as_url(&self) -> Url {
		let runtime_version = self.get_runtime_version();
		let version = self.version.to_string();
		let runtime_name = self.runtime.clone();
		let url = format!("https://github.com/paritytech/polkadot/releases/download/v{version}/{runtime_name}_runtime-v{runtime_version}.compact.compressed.wasm");
		Url::parse(&url).expect("Url should parse")
	}

	// TODO: Move that to release crate
	fn get_runtime_version(&self) -> RuntimeVersion {
		let mut res = format!("{}0", self.version.to_string().replace(".", ""));
		let _zero = res.remove(0);
		res
	}
}

impl FromStr for GithubRef {
	type Err = SubwasmLibError;

	/// Extract runtime and version from `<runtime>@<version>`
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut parts = s.split('@');
		if parts.clone().count() != 2 {
			return Err(SubwasmLibError::Generic(
				"Unsupported Github version format, should be <runtime>@<version>".to_string(),
			));
		} else {
			let runtime = parts.next().expect("We did not get the expected 2 parts").to_string();
			let version = parts.next().expect("We did not get the expected 2 parts").to_string().replace("v", "");

			let res = Self {
				runtime,
				version: Version::from_str(&version)
					.map_err(|_e| SubwasmLibError::Generic("Version parsing error".to_string()))?,
			};
			Ok(res)
		}
	}
}

#[cfg(test)]
mod test_github_ref {
	use super::*;

	#[test]
	fn test_from_str() {
		assert!(GithubRef::from_str("kusama@1.2.3").is_ok());
		assert!(GithubRef::from_str("kusama@v1.2.3").is_ok());
		assert!(GithubRef::from_str("kusama-1.2.3").is_err());
		assert!(GithubRef::from_str("ksm123").is_err());
		assert!(GithubRef::from_str("123").is_err());
	}

	#[test]
	fn test_as_url() {
		let gh = GithubRef::from_str("kusama@1.2.3").expect("Failed parsing GithubRef");
		assert_eq!("https://github.com/paritytech/polkadot/releases/download/v1.2.3/kusama_runtime-v230.compact.compressed.wasm", gh.as_url().as_str());
	}
}

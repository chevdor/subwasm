use log::debug;
use std::{fmt::Display, str::FromStr};

/// A Filter struct initially planned to filter module/call
/// While module is implemented, the filter on call is not and
/// if we filter on call, we may want to also filter on events, constants
/// etc... and that becomes likely too complex to be comfortable and useful
/// when using the cli vs using json and filtering with jq.
#[derive(Debug, PartialEq, Default)]
pub struct Filter {
	pub module: String,
	pub call: Option<String>,
}

#[derive(Debug)]
pub enum Error {
	Parsing(String),
}

impl Display for Error {
	fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Error::Parsing(m) => write!(fmt, "Filter parsing failed: {}", m),
			// _ => write!(fmt, "Unknown error"),
		}
	}
}

impl FromStr for Filter {
	type Err = Error;

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		let input = input.to_lowercase();
		if input.is_empty() {
			return Err(Error::Parsing("Cannot have a filter without at least a module".to_string()));
		}

		let mut chunks = input.split('.');
		let module = chunks
			.next()
			.map(|s| s.to_string())
			.ok_or_else(|| Error::Parsing("Cannot have a filter without at least a module".to_string()))?;
		let call = chunks.next().map(|s| s.to_string());

		let result = Self { module, call };
		debug!("from_str({}) => {:?}", input, result);
		Ok(result)
	}
}

#[cfg(test)]
mod test_super {
	use super::*;

	#[test]
	fn it_passes_common_tests() {
		assert!(
			Filter::from_str("Module.caLL").unwrap()
				== Filter { module: "module".to_string(), call: Some("call".to_string()) }
		);

		assert!(
			Filter::from_str("module.call.foobar").unwrap()
				== Filter { module: "module".to_string(), call: Some("call".to_string()) }
		);

		assert!(Filter::from_str("module").unwrap() == Filter { module: "module".to_string(), call: None });

		assert!(Filter::from_str("").is_err());
	}
}

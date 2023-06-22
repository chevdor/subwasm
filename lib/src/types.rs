use crate::error::SubwasmLibError;
use log::debug;
use std::str::FromStr;

/// A Filter struct initially planned to filter module/call
/// While module is implemented, the filter on call is not and
/// if we filter on call, we may want to also filter on events, constants
/// etc... and that becomes likely too complex to be comfortable and useful
/// when using the cli vs using json and filtering with jq.
#[derive(Debug, PartialEq, Default)]
pub struct Filter {
	/// The name of the module. This is now called `pallet`.
	pub module: String,

	/// Optional: the name of the call
	pub call: Option<String>,
}

impl FromStr for Filter {
	type Err = SubwasmLibError;

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		let input = input.to_lowercase();
		if input.is_empty() {
			return Err(SubwasmLibError::Parsing(input, "Cannot have a filter without at least a module".to_string()));
		}

		let mut chunks = input.split('.');
		let module = chunks
			.next()
			.map(|s| s.to_string())
			.ok_or_else(|| SubwasmLibError::Generic("Cannot have a filter without at least a module".to_string()))?;
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
			Filter::from_str("Module.caLL").expect("Failed init filter")
				== Filter { module: "module".to_string(), call: Some("call".to_string()) }
		);

		assert!(
			Filter::from_str("module.call.foobar").expect("Failed init filter")
				== Filter { module: "module".to_string(), call: Some("call".to_string()) }
		);

		assert!(
			Filter::from_str("module").expect("Failed init filter")
				== Filter { module: "module".to_string(), call: None }
		);

		assert!(Filter::from_str("").is_err());
	}
}

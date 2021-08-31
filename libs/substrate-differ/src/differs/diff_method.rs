use std::str::FromStr;

#[derive(Debug)]
pub enum DiffMethod {
	/// The metadata is serialized to json and the json representation are compared
	Raw,

	/// The runtimes are reduced first and the reduced runtimes are compared
	Reduced,
}

impl FromStr for DiffMethod {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"raw" | "json" => Ok(DiffMethod::Raw),
			"reduced" | "partial" => Ok(DiffMethod::Reduced),
			_ => Err(format!("Cannot convert '{}' to a known DiffMethod", s)),
		}
	}
}

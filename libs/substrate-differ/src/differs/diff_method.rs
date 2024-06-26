use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum DiffMethod {
	// /// Summary
	/// Summary,

	/// The runtimes are reduced first and the reduced runtimes are compared
	Reduced,
}

impl FromStr for DiffMethod {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			// "summary" => Ok(DiffMethod::Summary),
			"reduced" | "partial" => Ok(DiffMethod::Reduced),
			_ => Err(format!("Cannot convert '{s}' to a known DiffMethod")),
		}
	}
}

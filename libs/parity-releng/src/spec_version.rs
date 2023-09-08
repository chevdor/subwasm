//! This crate is WIP and is not yet ready for v1.x.x
use semver::Version;

/// The runtime sp_version::RuntimeVersion contains the spec_version as u32
#[derive(Debug, PartialEq)]
pub struct SpecVersion {
	pub(crate) spec_version: u32,
}

impl From<u32> for SpecVersion {
	fn from(spec_version: u32) -> Self {
		Self { spec_version }
	}
}

impl From<Version> for SpecVersion {
	/// Convert from a semver Version to a SpecVersion.
	/// For instance 0.9.42 => 9420
	fn from(v: Version) -> Self {
		let s = if v.major < 1 {
			// 0.9.42 / 0.9.420
			format!("{}{:0>1}{:0<3}", v.major, v.minor, v.patch)
		} else {
			format!("{}{:0>2}{:0>3}", v.major, v.minor, v.patch)
		};
		let spec_version: u32 = s.parse().expect("A cleaned up semver version shoult parse to u32");
		Self { spec_version }
	}
}

impl Into<Version> for SpecVersion {
	/// Convert 9420 into 00_09_420 into 0.9.420
	fn into(self) -> Version {
		let patch = self.spec_version as u64 % 1000;
		let minor = self.spec_version as u64 / 1000 % 100;
		let major = self.spec_version as u64 / 100_000;
		Version::new(major, minor, patch)
	}
}

#[cfg(test)]
mod test_spec_version {
	use super::*;

	#[test]
	fn test_from_u64() {
		assert_eq!(9420_u32, SpecVersion::from(9420_u32).spec_version);
		assert_eq!(10420_u32, SpecVersion::from(10420_u32).spec_version);
	}

	#[test]
	fn test_from_version_0_9() {
		assert_eq!(0_9_420_u32, SpecVersion::from(Version::parse("0.9.42").unwrap()).spec_version);
	}

	#[test]
	fn test_from_version_1_x_polkadot() {
		assert_eq!(1_00_042_u32, SpecVersion::from(Version::parse("1.0.42").unwrap()).spec_version);
		assert_eq!(1_00_001_u32, SpecVersion::from(Version::parse("1.0.1").unwrap()).spec_version);
		assert_eq!(1_00_010_u32, SpecVersion::from(Version::parse("1.0.10").unwrap()).spec_version);
		assert_eq!(1_01_000_u32, SpecVersion::from(Version::parse("1.1.0").unwrap()).spec_version);
		assert_eq!(1_01_001_u32, SpecVersion::from(Version::parse("1.1.1").unwrap()).spec_version);
	}

	#[test]
	fn test_from_version_cumulus() {
		assert_eq!(0_09_420_u32, SpecVersion::from(Version::parse("0.9.420").unwrap()).spec_version);
		assert_eq!(1_00_420_u32, SpecVersion::from(Version::parse("1.0.420").unwrap()).spec_version);
		assert_eq!(1_00_042_u32, SpecVersion::from(Version::parse("1.0.42").unwrap()).spec_version);
		assert_eq!(1_01_000_u32, SpecVersion::from(Version::parse("1.1.0").unwrap()).spec_version);
		assert_eq!(1_01_001_u32, SpecVersion::from(Version::parse("1.1.1").unwrap()).spec_version);
		assert_eq!(1_01_010_u32, SpecVersion::from(Version::parse("1.1.10").unwrap()).spec_version);
		assert_eq!(1_01_100_u32, SpecVersion::from(Version::parse("1.1.100").unwrap()).spec_version);
	}

	#[test]
	fn test_to_version() {
		assert_eq!(
			Version::parse("0.9.420").expect("Simple version can be parsed"),
			SpecVersion::from(9420_u32).into()
		);
		assert_eq!(
			Version::parse("1.2.345").expect("Simple version can be parsed"),
			SpecVersion::from(102345_u32).into()
		);
	}
}

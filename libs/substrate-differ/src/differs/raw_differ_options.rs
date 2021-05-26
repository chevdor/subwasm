#[derive(Debug)]
pub struct RawDifferOptions {
	/// Array of Bytes will be collapsed into String to make the diff easier to process for us humans...
	pub collapse: bool,

	/// Show stats
	pub stats: bool,

	/// Skip documentation fields, those are not critical but the diff can be verbose
	pub skip_doc: bool,

	/// Ignore the difference for the Array of Bytes (`value` and `default` keys)
	pub skip_bytes: bool,

	/// Ignore version numer. That allows seeing diffs accross versions.
	/// If we donät do this between say V12 and V13, the diff will show that we deleleted V12 and added V13... which is not that useful.
	pub ignore_version: bool,
}

impl Default for RawDifferOptions {
	fn default() -> Self {
		Self { collapse: true, skip_doc: true, stats: true, skip_bytes: false, ignore_version: true }
	}
}

impl RawDifferOptions {
	pub fn full() -> Self {
		Self { collapse: false, skip_doc: false, stats: false, skip_bytes: false, ignore_version: false }
	}
}

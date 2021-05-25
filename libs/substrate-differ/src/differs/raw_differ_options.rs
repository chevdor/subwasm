#[derive(Debug)]
pub struct RawDifferOptions {
	pub collapse: bool,
	pub skip_doc: bool,
	pub stats: bool,
}

impl Default for RawDifferOptions {
	fn default() -> Self {
		Self { collapse: true, skip_doc: true, stats: true }
	}
}

impl RawDifferOptions {
	pub fn full() -> Self {
		Self { collapse: false, skip_doc: false, stats: false }
	}
}

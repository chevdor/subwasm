/// In order to reduce the output, the user may decide to
/// skip some checks. This struct keeps track of the
/// the diff we potentially skipped in order to inform the user
/// about them.
#[derive(Debug, Default)]
pub struct SkipCounter {
	pub documentation: u64,

	pub bytes: u64,
}

impl SkipCounter {
	pub fn print(&self) {
		println!("Skipped:");
		println!("- documentation\t= {:>5}", self.documentation);
		println!("- bytes\t\t= {:>5}", self.bytes);
	}
}

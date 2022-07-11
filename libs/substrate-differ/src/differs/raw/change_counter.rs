use super::change_type::ChangeType;

/// This struct keeps track of the various Changes detected by the `RawDiffer`.
#[derive(Debug, Default)]
pub struct ChangeCounter {
	pub removed: u64,
	pub added: u64,
	pub unchanged: u64,
	pub modified: u64,
}

impl ChangeCounter {
	pub fn inc(&mut self, ty: ChangeType) {
		match ty {
			ChangeType::Removed => self.removed += 1,
			ChangeType::Added => self.added += 1,
			ChangeType::Unchanged => self.unchanged += 1,
			ChangeType::Modified => self.modified += 1,
		}
	}

	pub fn percent(&self, ty: ChangeType) -> f64 {
		let sum = (self.unchanged + self.modified + self.removed + self.added) as f64;
		let relevant = match ty {
			ChangeType::Removed => self.removed as f64,
			ChangeType::Added => self.added as f64,
			ChangeType::Unchanged => self.unchanged as f64,
			ChangeType::Modified => self.modified as f64,
		};
		relevant / sum
	}

	pub fn print(&self) {
		println!("Changes:");
		println!("- unmodified\t= {:>5.02}% ({})", self.percent(ChangeType::Unchanged) * 100_f64, self.unchanged);
		println!("- added\t\t= {:>5.02}% ({})", self.percent(ChangeType::Added) * 100_f64, self.added);
		println!("- modified\t= {:>5.02}% ({})", self.percent(ChangeType::Modified) * 100_f64, self.modified);
		println!("- removed\t= {:>5.02}% ({})", self.percent(ChangeType::Removed) * 100_f64, self.removed);
	}
}

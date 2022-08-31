use super::change_type::Change;

/// This struct keeps track of the various Changes detected by the `RawDiffer`.
#[derive(Debug, Default)]
pub struct ChangeCounter {
	pub removed: u64,
	pub added: u64,
	pub unchanged: u64,
	pub modified: u64,
}

impl ChangeCounter {
	pub fn inc(&mut self, ty: Change) {
		match ty {
			Change::Removed => self.removed += 1,
			Change::Added => self.added += 1,
			Change::Unchanged => self.unchanged += 1,
			Change::Modified => self.modified += 1,
		}
	}

	pub fn percent(&self, ty: Change) -> f64 {
		let sum = (self.unchanged + self.modified + self.removed + self.added) as f64;
		let relevant = match ty {
			Change::Removed => self.removed as f64,
			Change::Added => self.added as f64,
			Change::Unchanged => self.unchanged as f64,
			Change::Modified => self.modified as f64,
		};
		relevant / sum
	}

	pub fn print(&self) {
		println!("Changes:");
		println!("- unmodified\t= {:>5.02}% ({})", self.percent(Change::Unchanged) * 100_f64, self.unchanged);
		println!("- added\t\t= {:>5.02}% ({})", self.percent(Change::Added) * 100_f64, self.added);
		println!("- modified\t= {:>5.02}% ({})", self.percent(Change::Modified) * 100_f64, self.modified);
		println!("- removed\t= {:>5.02}% ({})", self.percent(Change::Removed) * 100_f64, self.removed);
	}
}

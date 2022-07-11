pub mod diff_method;
pub mod raw;
pub mod reduced;
pub mod summary_differ;
pub mod utils;

#[derive(Debug)]
pub struct DiffOptionInclude {
	added: bool,
	removed: bool,
	changed: bool,
	doc: bool,
}

impl Default for DiffOptionInclude {
	fn default() -> Self {
		Self { added: true, removed: true, changed: true, doc: true }
	}
}

#[derive(Debug, Default)]
pub struct DiffOptions {
	include: DiffOptionInclude,
}

pub trait Differ {
	fn diff(&self, options: DiffOptions);
}

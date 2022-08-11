pub mod diff_method;
pub mod raw;
pub mod reduced;
pub mod summary;
pub mod utils;
use crate::differs::reduced::diff_result::DiffResult;
use crate::differs::reduced::PalletId;

#[cfg(test)]
pub mod test_constants;

#[derive(Debug)]
pub struct DiffOptionInclude {
	pub added: bool,
	pub removed: bool,
	pub changed: bool,
	pub doc: bool,
}

impl Default for DiffOptionInclude {
	fn default() -> Self {
		Self { added: true, removed: true, changed: true, doc: true }
	}
}

#[derive(Debug, Default)]
pub struct DiffOptions {
	pub include: DiffOptionInclude,
}

pub trait Differ<T>
where
	T: PartialEq,
{
	fn diff(&self, options: DiffOptions) -> Vec<(PalletId, DiffResult<'_, T>)>;
}

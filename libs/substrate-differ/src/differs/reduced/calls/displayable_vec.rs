use std::fmt::Display;

const LIMIT: usize = 32;

#[derive(Debug, Default)]
pub struct DisplayableVec<'a, T> {
	/// This is the referenced data we want to display
	reference: &'a [T],

	/// If the size the data remains under the `min_size`, it is displayed "as is"
	/// even if that means showing repetitions
	min_size: usize,

	/// Truncate above this length
	max_size: usize,

	/// Is the data made of ONE single repeating value ?
	all_same: Option<T>,
}

impl<'a, T: Default + PartialEq + Display + Clone + Copy + std::fmt::Debug> DisplayableVec<'a, T> {
	pub fn new(reference: &'a [T], max_size: Option<usize>) -> Self {
		Self { reference, min_size: 3, max_size: if let Some(m) = max_size { m } else { LIMIT }, ..Default::default() }
	}

	pub fn init(mut self) -> Self {
		let first = self.reference.iter().next().map(|x| *x).unwrap_or_default();

		self.all_same = if self.reference.iter().all(|&x| x == first) { Some(first) } else { None };
		// println!("self = {self:#?}");
		self
	}

	pub fn oversized(&self, limit: Option<usize>) -> bool {
		let limit = if let Some(value) = limit { value } else { LIMIT };
		self.reference.len() > limit
	}

	/// Lots of the default values are arrays of zeroes. This helper shows those long
	/// arrays in a compressed form more appropriate to display.
	/// Example: [0, 0, 0, 0] => [0: 4]
	fn repeating_vec_to_string(&self) -> String {
		if self.reference.len() > self.min_size {
			if let Some(same) = &self.all_same {
				return format!("[{}; {}]", same, self.reference.len());
			}
		}

		format!("{:?}", self.reference)
	}

	pub fn to_short_string(&self) -> String {
		self.to_short_string_with_max(self.max_size)
	}

	pub fn to_short_string_with_max(&self, max: usize) -> String {
		if self.all_same.is_some() {
			return self.repeating_vec_to_string();
		}

		if !self.oversized(Some(max)) {
			return format!("{:?}", self.reference);
		}

		let v = Vec::from(self.reference);
		let chunk = v.chunks(max).next().expect("Data is oversized so we must get at least one chunk");
		let strings: Vec<String> = chunk.iter().map(|x| x.to_string()).collect();
		format!("[ {}, ... ]", strings.join(", "))
	}
}

#[cfg(test)]
mod test_vec_display {
	use super::*;

	#[test]
	fn test_vec_display() {
		assert_eq!("[0]", DisplayableVec::new(&vec![0], None).init().to_short_string());
		assert_eq!("[0, 0, 0, 0]", DisplayableVec::new(&vec![0, 0, 0, 0], None).init().to_short_string());
		assert_eq!("[42; 4]", DisplayableVec::new(&vec![42, 42, 42, 42], Some(3)).init().to_short_string());
		assert_eq!("[99; 4]", DisplayableVec::new(&vec![99, 99, 99, 99], None).init().to_short_string_with_max(3));
		assert_eq!("[1, 2, 3, 4]", DisplayableVec::new(&vec![1, 2, 3, 4], None).init().to_short_string());
		assert_eq!(
			"[ 1, 2, 3, ... ]",
			DisplayableVec::new(&vec![1, 2, 3, 4, 5], None).init().to_short_string_with_max(3)
		);
	}
	#[test]
	fn test_vec_display_2() {
		let input =
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
		assert_eq!("[0; 32]", DisplayableVec::new(&input, None).init().to_short_string());
	}
}

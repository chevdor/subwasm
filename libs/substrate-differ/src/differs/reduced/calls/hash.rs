// #![deprecated(note = "ValueHash will likley not be used and replaced by the actual value")]
// use serde::Serialize;
// use std::fmt::Display;

// #[derive(Default, Debug, PartialEq, Eq, Hash, Serialize)]
// #[allow(deprecated)]
// pub struct ValueHash {
// 	inner: [u8; 32],
// }

// impl ValueHash {
// 	pub fn as_mut(&mut self) -> &mut [u8] {
// 		&mut self.inner
// 	}

// 	pub fn as_slice(&mut self) -> &[u8] {
// 		&self.inner
// 	}
// }

// impl Display for ValueHash {
// 	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// 		// TODO: do better
// 		f.write_fmt(format_args!("{:?}", self.inner))
// 	}
// }

// #[cfg(test)]
// mod test_super {
// 	use crate::differs::reduced::call::hash::ValueHash;

// 	#[test]
// 	fn test_hash_display() {
// 		let h = ValueHash::default();
// 		println!("h = {:?}", h);
// 		println!("h = {}", h);
// 	}
// }

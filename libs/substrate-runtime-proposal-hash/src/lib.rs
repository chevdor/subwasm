use blake2::digest::{Update, VariableOutput};
use blake2::VarBlake2b;
use codec::Encode;
use std::convert::TryInto;

/// Expected size of the hash
pub const SIZE: usize = 32;

/// Type for our Proposal hash
pub type ProposalHash = [u8; SIZE];

/// The PREFIX is prepended to the data before hashing
const PREFIX: [u8; 2] = [0x00, 0x03];

/// This struct is a container for whatever we calculated.
#[derive(Debug)]
pub struct SrhResult {
	/// This is the PropsalHash itself, not encoded
	pub hash: ProposalHash,

	/// Hex encoded proposal hash.
	pub encodedd_hash: String,
}

/// Concatenate 2 arrays.
pub fn concatenate_arrays<T: Clone>(x: &[T], y: &[T]) -> Vec<T> {
	let mut concat = x.to_vec();
	concat.extend_from_slice(y);
	concat
}

/// Generate our result object
pub fn get_result(buffer: &[u8]) -> SrhResult {
	buffer.using_encoded(|ref wasm_blob| {
		let hash = get_proposal_hash(wasm_blob);

		SrhResult { hash, encodedd_hash: hex::encode(hash) }
	})
}

/// Calculate the proposal hash
///
/// # Arguments
/// * `wasm_blob` - The WASM blob
/// # Returns
/// * `ProposalHash` - The hash of the proposal as calculated on chain
fn get_proposal_hash(wasm_blob: &[u8]) -> ProposalHash {
	let mut hasher = VarBlake2b::new(SIZE).unwrap();
	hasher.update(concatenate_arrays(&PREFIX, &wasm_blob));
	let mut result: ProposalHash = [0; SIZE];
	hasher.finalize_variable(|res| {
		result = res.try_into().expect("slice with incorrect length");
	});
	result
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_hash() {
		assert_eq!(
			get_proposal_hash(&[1, 2, 42]),
			[
				156, 244, 243, 93, 21, 8, 113, 238, 186, 17, 20, 52, 240, 236, 140, 15, 108, 26, 86, 5, 152, 148, 91,
				162, 108, 168, 3, 65, 254, 162, 114, 46
			]
		);
	}

	#[test]
	fn test_hash_length() {
		assert_eq!(32, get_proposal_hash(&[0]).len());
	}

	#[test]
	fn test_get_result() {
		let res = get_result(&[1, 2, 42]);
		assert!(res.encodedd_hash == "9388ba11b3f2a5db3ef9bf237f1c88ffb369d77ffa843fc67570c89c09fa9c0e");
	}

	#[test]
	fn test_long_input() {
		const SIZE_8MB: usize = 8 * 1024 * 1024;
		let res = get_result(&[0; SIZE_8MB]);
		assert!(res.encodedd_hash == "9348da94fcffe94318313f8ce237211a7fd6c1531ab21b61606a1f7eeb8b2409");
	}
}

use blake2::digest::{Update, VariableOutput};
use blake2::Blake2bVar;
use codec::Encode;
use hex::FromHex;
use sp_core::Hasher;
use sp_runtime::traits::BlakeTwo256;
use std::env;

/// Expected size of the hash
pub const SIZE: usize = 32;

/// Type for our Proposal hash
pub type CalllHash = [u8; SIZE];

type Prefix = (u8, u8);

/// The PREFIX is prepended to the data before hashing
pub const PREFIX_SYSTEM_SETCODE: Prefix = (0x00, 0x03);

const PARACHAIN_PALLET_ID_ENV: &str = "PARACHAIN_PALLET_ID";
const DEFAULT_PARACHAIN_PALLET_ID: &str = "0x01";

const AUTHORIZE_UPGRADE_PREFIX_ENV: &str = "AUTHORIZE_UPGRADE_PREFIX";
const DEFAULT_AUTHORIZE_UPGRADE_PREFIX: &str = "0x02";

/// This struct is a container for whatever we calculated.
#[derive(Debug)]
pub struct SrhResult {
	/// This is the PropsalHash itself, not encoded
	pub hash: CalllHash,

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
pub fn get_result(prefix: Prefix, buffer: &[u8]) -> SrhResult {
	buffer.using_encoded(|wasm_blob| {
		let hash = get_call_hash(prefix, wasm_blob);

		SrhResult { hash, encodedd_hash: hex::encode(hash) }
	})
}

/// Calculate the proposal hash
///
/// # Arguments
/// * `wasm_blob` - The WASM blob
/// # Returns
/// * `CalllHash` - The hash of the proposal as calculated on chain
/// @deprecated
pub fn get_proposal_hash(wasm_blob: &[u8]) -> CalllHash {
	get_call_hash(PREFIX_SYSTEM_SETCODE, wasm_blob)
}

/// This function replaces the deprecated `get_proposal_hash`
pub fn get_system_setcode(wasm_blob: &[u8]) -> CalllHash {
	get_call_hash(PREFIX_SYSTEM_SETCODE, wasm_blob)
}

pub fn get_parachainsystem_authorize_upgrade(wasm_blob: &[u8]) -> CalllHash {
	let s1 =
		env::var(PARACHAIN_PALLET_ID_ENV).unwrap_or_else(|_| DEFAULT_PARACHAIN_PALLET_ID.into()).replacen("0x", "", 1);
	let s2 = env::var(AUTHORIZE_UPGRADE_PREFIX_ENV)
		.unwrap_or_else(|_| DEFAULT_AUTHORIZE_UPGRADE_PREFIX.into())
		.replacen("0x", "", 1);
	let decoded1 = <[u8; 1]>::from_hex(&s1).map_err(|_| RuntimePropHashError::HexDecoding(s1))?;
	let decoded2 = <[u8; 1]>::from_hex(&s2).map_err(|_| RuntimePropHashError::HexDecoding(s2))?;

	let parachain_pallet_id = *decoded1.first().expect("Failure while fecthing the Parachain Pallet ID");
	let authorize_upgrade_prefix = *decoded2.first().expect("Failure while fecthing the Auhtorize upgrade ID");
	let prefix_parachainsystem_authorize_upgrade: Prefix = (parachain_pallet_id, authorize_upgrade_prefix);
	let code_hash = BlakeTwo256::hash(wasm_blob);
	get_call_hash(prefix_parachainsystem_authorize_upgrade, code_hash.as_bytes())
}

fn get_call_hash(prefix: Prefix, wasm_blob: &[u8]) -> CalllHash {
	let mut hasher = Blake2bVar::new(SIZE).unwrap();
	hasher.update(&concatenate_arrays(&[prefix.0, prefix.1], wasm_blob));
	let mut result: CalllHash = [0; SIZE];
	hasher.finalize_variable(&mut result).unwrap();
	result
}

#[cfg(test)]
mod prop_hash_tests {
	use super::*;
	use std::env;

	#[test]
	fn test_proposal_hash() {
		assert_eq!(
			get_proposal_hash(&[1, 2, 42]),
			[
				156, 244, 243, 93, 21, 8, 113, 238, 186, 17, 20, 52, 240, 236, 140, 15, 108, 26, 86, 5, 152, 148, 91,
				162, 108, 168, 3, 65, 254, 162, 114, 46
			]
		);
	}

	#[test]
	fn test_call_hash() {
		assert_eq!(
			get_call_hash(PREFIX_SYSTEM_SETCODE, &[1, 2, 42]),
			[
				156, 244, 243, 93, 21, 8, 113, 238, 186, 17, 20, 52, 240, 236, 140, 15, 108, 26, 86, 5, 152, 148, 91,
				162, 108, 168, 3, 65, 254, 162, 114, 46
			]
		);
	}

	#[test]
	fn test_parachain_upgrade() {
		env::set_var(PARACHAIN_PALLET_ID_ENV, DEFAULT_PARACHAIN_PALLET_ID);
		env::set_var(AUTHORIZE_UPGRADE_PREFIX_ENV, DEFAULT_AUTHORIZE_UPGRADE_PREFIX);
		assert_eq!(
			get_parachainsystem_authorize_upgrade(&[
				0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x97, 0x03, 0x39, 0x60, 0x03, 0x7f, 0x7f
			]),
			[
				231, 116, 0, 171, 31, 105, 209, 55, 219, 85, 107, 244, 188, 127, 92, 82, 111, 152, 5, 80, 44, 48, 66,
				9, 156, 175, 248, 163, 40, 92, 101, 11
			]
		);
	}

	// todo: sporadic errors
	#[test]
	fn test_custom_parachain_upgrade() {
		env::set_var(PARACHAIN_PALLET_ID_ENV, "0x32");
		env::set_var(AUTHORIZE_UPGRADE_PREFIX_ENV, DEFAULT_AUTHORIZE_UPGRADE_PREFIX);

		assert_eq!(
			get_parachainsystem_authorize_upgrade(&[
				0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x97, 0x03, 0x39, 0x60, 0x03, 0x7f, 0x7f
			]),
			[
				51, 203, 30, 131, 48, 13, 150, 26, 217, 87, 213, 55, 43, 10, 200, 193, 248, 254, 202, 83, 165, 231, 4,
				59, 213, 247, 98, 153, 119, 166, 175, 133
			]
		);
	}

	#[test]
	fn test_hash_length() {
		assert_eq!(32, get_proposal_hash(&[0]).len());
	}

	#[test]
	fn test_get_result() {
		let res = get_result(PREFIX_SYSTEM_SETCODE, &[1, 2, 42]);
		assert!(res.encodedd_hash == "9388ba11b3f2a5db3ef9bf237f1c88ffb369d77ffa843fc67570c89c09fa9c0e");
	}

	#[test]
	fn test_long_input() {
		const SIZE_8MB: usize = 8 * 1024 * 1024;
		let res = get_result(PREFIX_SYSTEM_SETCODE, &[0; SIZE_8MB]);
		assert!(res.encodedd_hash == "9348da94fcffe94318313f8ce237211a7fd6c1531ab21b61606a1f7eeb8b2409");
	}
}

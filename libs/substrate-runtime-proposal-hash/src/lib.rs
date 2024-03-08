pub mod error;

use blake2::digest::{Update, VariableOutput};
use blake2::Blake2bVar;
use codec::Encode;
use error::*;
use sp_core::Hasher;
use sp_runtime::traits::BlakeTwo256;

/// Expected size of the hash
pub const SIZE: usize = 32;

/// Type for our Proposal hash
pub type CalllHash = [u8; SIZE];

type Prefix = (u8, u8);

/// The PREFIX is prepended to the data before hashing
pub const PREFIX_SYSTEM_SETCODE: Prefix = (0x00, 0x02);

pub const PARACHAIN_PALLET_ID_ENV: &str = "PARACHAIN_PALLET_ID";
pub const DEFAULT_PARACHAIN_PALLET_ID: &str = "0x01";

pub const AUTHORIZE_UPGRADE_PREFIX_ENV: &str = "AUTHORIZE_UPGRADE_PREFIX";
pub const DEFAULT_AUTHORIZE_UPGRADE_PREFIX: &str = "0x02";

pub const AUTHORIZE_UPGRADE_CHECK_VERSION_ENV: &str = "AUTHORIZE_UPGRADE_CHECK_VERSION";

/// This struct is a container for whatever we calculated.
#[derive(Debug)]
pub struct SrhResult {
	/// This is the PropsalHash itself, not encoded
	pub hash: CalllHash,

	/// Hex encoded proposal hash.
	pub encoded_hash: String,
}

/// Concatenate 2 arrays.
pub fn concatenate_arrays<T: Clone>(x: &[T], y: &[T]) -> Vec<T> {
	let mut concat = x.to_vec();
	concat.extend_from_slice(y);
	concat
}

/// Generate our result object
pub fn get_result(prefix: Prefix, buffer: &[u8]) -> Result<SrhResult> {
	buffer.using_encoded(|wasm_blob: &[u8]| {
		let result = match get_call_hash(prefix, wasm_blob) {
			Ok(hash) => Ok(SrhResult { hash, encoded_hash: hex::encode(hash) }),
			Err(_e) => Err(RuntimePropHashError::HashComputing()),
		};
		result
	})
}

/// Calculate the proposal hash which is system.setCode
///
/// # Arguments
/// * `wasm_blob` - The WASM blob
/// # Returns
/// * `CalllHash` - The hash of the proposal as calculated on chain
/// This function replaces the deprecated `get_proposal_hash`
pub fn get_system_setcode(wasm_blob: &[u8]) -> Result<CalllHash> {
	get_call_hash(PREFIX_SYSTEM_SETCODE, wasm_blob)
}

pub fn get_parachainsystem_authorize_upgrade(
	prefix: Prefix,
	wasm_blob: &[u8],
	check_spec_version: Option<bool>,
) -> Result<CalllHash> {
	let code_hash = BlakeTwo256::hash(wasm_blob);
	let call_hash_preimage = if let Some(check_version) = check_spec_version {
		let encoded_check_version = [check_version as u8; 1];
		[code_hash.as_bytes(), encoded_check_version.as_slice()].concat()
	} else {
		code_hash.as_bytes().to_owned()
	};
	let call_hash = get_call_hash(prefix, call_hash_preimage.as_slice())?;
	Ok(call_hash)
}

fn get_call_hash(prefix: Prefix, wasm_blob: &[u8]) -> Result<CalllHash> {
	let mut hasher = Blake2bVar::new(SIZE)?;
	let prefix_array = vec![prefix.0, prefix.1];
	hasher.update(&concatenate_arrays(&prefix_array, wasm_blob));
	let mut result: CalllHash = [0; SIZE];
	hasher.finalize_variable(&mut result)?;
	Ok(result)
}

#[cfg(test)]
mod prop_hash_tests {
	use super::*;

	#[test]
	fn test_call_hash() {
		assert_eq!(
			get_call_hash(PREFIX_SYSTEM_SETCODE, &[1, 2, 42]).expect("Failed getting a hash"),
			[
				174, 123, 79, 240, 254, 106, 147, 26, 65, 61, 41, 84, 181, 102, 24, 182, 128, 135, 188, 31, 135, 187,
				99, 34, 143, 35, 120, 100, 246, 90, 186, 106
			]
		);
	}

	#[test]
	fn test_call_hash2() {
		assert_eq!(
			get_call_hash(PREFIX_SYSTEM_SETCODE, &[0u8; 10 * 1024 * 1024]).expect("Failed getting a hash"),
			[
				174, 179, 75, 47, 133, 33, 33, 117, 227, 27, 2, 242, 183, 69, 3, 244, 244, 219, 39, 24, 243, 38, 98,
				214, 76, 225, 132, 3, 25, 124, 58, 225
			]
		);
	}

	#[test]
	fn test_parachain_upgrade() {
		assert_eq!(
			get_parachainsystem_authorize_upgrade(
				(0x01, 0x02),
				&[0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x97, 0x03, 0x39, 0x60, 0x03, 0x7f, 0x7f],
				None
			)
			.expect("Failed getting a hash"),
			[
				231, 116, 0, 171, 31, 105, 209, 55, 219, 85, 107, 244, 188, 127, 92, 82, 111, 152, 5, 80, 44, 48, 66,
				9, 156, 175, 248, 163, 40, 92, 101, 11
			]
		);
	}

	#[test]
	fn test_parachain_upgrade_with_check_version_flag() {
		assert_eq!(
			get_parachainsystem_authorize_upgrade(
				(0x01, 0x02),
				&[0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x97, 0x03, 0x39, 0x60, 0x03, 0x7f, 0x7f],
				Some(true)
			)
			.expect("Failed getting a hash"),
			[
				3, 115, 197, 16, 201, 214, 199, 165, 224, 209, 253, 187, 187, 123, 235, 163, 30, 53, 236, 117, 235,
				174, 36, 14, 244, 4, 117, 95, 184, 249, 174, 176
			]
		);
		assert_eq!(
			get_parachainsystem_authorize_upgrade(
				(0x01, 0x02),
				&[0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x97, 0x03, 0x39, 0x60, 0x03, 0x7f, 0x7f],
				Some(false)
			)
			.expect("Failed getting a hash"),
			[
				119, 119, 214, 62, 223, 113, 109, 231, 134, 250, 52, 135, 213, 23, 52, 143, 125, 34, 235, 123, 167,
				177, 14, 206, 14, 88, 22, 165, 110, 233, 139, 157
			]
		);
	}

	#[test]
	fn test_custom_parachain_upgrade() {
		assert_eq!(
			get_parachainsystem_authorize_upgrade(
				(0x32, 0x02),
				&[0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x97, 0x03, 0x39, 0x60, 0x03, 0x7f, 0x7f],
				None
			)
			.expect("Failed getting a hash"),
			[
				51, 203, 30, 131, 48, 13, 150, 26, 217, 87, 213, 55, 43, 10, 200, 193, 248, 254, 202, 83, 165, 231, 4,
				59, 213, 247, 98, 153, 119, 166, 175, 133
			]
		);
	}

	#[test]
	fn test_custom_parachain_upgrade_with_check_version_flag() {
		assert_eq!(
			get_parachainsystem_authorize_upgrade(
				(0x32, 0x02),
				&[0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x97, 0x03, 0x39, 0x60, 0x03, 0x7f, 0x7f],
				Some(true)
			)
			.expect("Failed getting a hash"),
			[
				70, 248, 60, 132, 125, 149, 3, 80, 167, 12, 27, 56, 89, 181, 128, 158, 6, 205, 11, 42, 25, 255, 123,
				216, 86, 86, 127, 53, 193, 119, 224, 44
			]
		);
		assert_eq!(
			get_parachainsystem_authorize_upgrade(
				(0x32, 0x02),
				&[0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x97, 0x03, 0x39, 0x60, 0x03, 0x7f, 0x7f],
				Some(false)
			)
			.expect("Failed getting a hash"),
			[
				209, 146, 155, 255, 19, 158, 45, 191, 35, 129, 211, 182, 29, 74, 42, 196, 13, 139, 203, 105, 183, 102,
				35, 186, 218, 119, 33, 152, 116, 152, 199, 203
			]
		);
	}

	#[test]
	fn test_hash_length() {
		assert_eq!(32, get_call_hash((0, 0), &[0]).expect("Failed getting a hash").len());
	}

	#[test]
	fn test_get_result() {
		let res = get_result(PREFIX_SYSTEM_SETCODE, &[1, 2, 42]).expect("Failed getting a hash");
		assert_eq!(res.encoded_hash, "85233004e044626a6c08e469573369536d8de9b264b08eb6611b76b80148e429");
	}

	#[test]
	fn test_long_input() {
		const SIZE_8MB: usize = 8 * 1024 * 1024;
		let res = get_result(PREFIX_SYSTEM_SETCODE, &[0; SIZE_8MB]).expect("Failed getting a hash");
		assert_eq!(res.encoded_hash, "44de98eef7227a1f55c5d1cf2b437dc87e60177dc8607538a115773b567ed0d5");
	}
}

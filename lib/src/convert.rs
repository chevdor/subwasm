use frame_metadata::decode_different::DecodeDifferent;

/// Helper to make fetching the decoded value easier
pub fn convert<B: 'static, O: 'static>(dd: &DecodeDifferent<B, O>) -> &O {
	match dd {
		DecodeDifferent::Decoded(value) => value,
		_ => panic!("Failed decoding, that's unfortunate and rather expected"),
	}
}

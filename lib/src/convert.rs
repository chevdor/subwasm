use frame_metadata::decode_different::DecodeDifferent;

/// Helper to make fetching the decoded value easier for V12 and V13.
pub fn convert<B: 'static, O: 'static>(dd: &DecodeDifferent<B, O>) -> &O {
	match dd {
		DecodeDifferent::Decoded(value) => value,
		_ => panic!("Failed decoding, that's unfortunate and rather expected"),
	}
}

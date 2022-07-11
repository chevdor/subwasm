use serde::Serialize;

use crate::{WasmBytes, CODE_BLOB_BOMB_LIMIT};

/// Stores compression information
#[derive(Debug, Clone, Copy, Serialize)]
pub struct Compression {
	/// Size in bytes of the compressed runtime. This is what you get from disk or from the chain.
	size_compressed: usize,

	/// Size in bytes after decompression.
	size_decompressed: usize,

	/// Whether we detected compression or not.
	compressed: bool,
}

impl Compression {
	pub fn new(compressed: &[u8], decompressed: &[u8]) -> Self {
		Self {
			size_compressed: compressed.len(),
			size_decompressed: decompressed.len(),
			compressed: compressed != decompressed,
		}
	}

	pub fn compressed(&self) -> bool {
		self.compressed
	}

	pub fn compression_ratio(&self) -> f32 {
		self.size_compressed as f32 / self.size_decompressed as f32
	}

	/// Compress a runtime
	pub fn compress(bytes: &[u8]) -> Result<WasmBytes, String> {
		match sp_maybe_compressed_blob::compress(bytes, CODE_BLOB_BOMB_LIMIT) {
			Some(bytes) => Ok(bytes.to_vec()),
			None => Err(String::from("Compression failed and returned nothing")),
		}
	}

	/// Decompress a runtime
	pub fn decompress(bytes: &[u8]) -> Result<WasmBytes, String> {
		sp_maybe_compressed_blob::decompress(bytes, CODE_BLOB_BOMB_LIMIT)
			.map(|res| res.to_vec())
			.map_err(|err| err.to_string())
	}
}

#[cfg(test)]
mod tests_compression {
	use super::*;

	#[test]
	fn test_ratio() {
		let v1 = vec![1, 2];
		let v2 = vec![1, 2, 3, 4];
		let c = Compression::new(&v1, &v2);
		assert!((c.compression_ratio() - 0.50).abs() < 0.01);
	}

	#[test]
	fn test_compression() {
		let bytes = vec![0, 42, 7, 27, 0, 0, 0, 27, 26, 27];
		let compressed = Compression::compress(&bytes);
		assert!(compressed.is_ok());
		assert_eq!(
			Ok(vec![
				82, 188, 83, 118, 70, 219, 142, 5, 40, 181, 47, 253, 0, 88, 81, 0, 0, 0, 42, 7, 27, 0, 0, 0, 27, 26, 27
			]),
			compressed
		);

		let decompressed = Compression::decompress(&compressed.unwrap());
		assert_eq!(bytes, decompressed.unwrap());
	}
}

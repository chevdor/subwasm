use serde::Serialize;

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
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_ratio() {
		let v1 = vec![1, 2];
		let v2 = vec![1, 2, 3, 4];
		let c = Compression::new(&v1, &v2);
		assert!((c.compression_ratio() - 0.50).abs() < 0.01);
	}
}

pub mod error;

use ipfs_unixfs::file::adder::Chunker;
use ipfs_unixfs::file::adder::FileAdder;

use error::*;

/// Provide the bytes of your content and IpfsHasher
/// will return the IPFS hash.
///
/// Calculation the IPFS hash of a file is not as trivial as
/// passing all its bytes into a Hasher. It first requires to
/// split the bytes in a chunks of a given size, turning those
/// into blocks, giving 1..n CIDs then return the root CID.
/// This is what this crate does.
/// If you need more control, check out the `cid` and the `ipfs-unixfs``
/// crates.
/// This crate procudes CidV0.
#[derive(Debug, Default)]
pub struct IpfsHasher {
	chunk_size: Option<usize>,
}

impl IpfsHasher {
	#[cfg(test)]
	pub fn new(chunk_size: usize) -> Self {
		Self { chunk_size: Some(chunk_size) }
	}

	/// Compute and return the IPFS Hash (cid) as String
	pub fn compute(&self, content: &[u8]) -> Result<String> {
		let mut adder = match self.chunk_size {
			None => FileAdder::default(),
			Some(size) => FileAdder::builder().with_chunker(Chunker::Size(size)).build(),
		};

		let mut written = 0;

		while written < content.len() {
			let slice = &content[written..];
			let (_blocks, pushed) = adder.push(slice);
			written += pushed;
		}

		let (cid, _data) = adder.finish().last().ok_or_else(|| IpfsHasherError::HashError())?;

		Ok(cid.to_string())
	}
}

#[cfg(test)]
mod tests {
	use super::IpfsHasher;
	use wasm_loader::{OnchainBlock, Source, WasmLoader};

	#[test]
	fn it_works_with_single_block() {
		let hasher = IpfsHasher::default();
		let ipfs = hasher.compute(b"foobar\n").unwrap();
		assert!(ipfs == "QmRgutAxd8t7oGkSm4wmeuByG6M51wcTso6cubDdQtuEfL");
	}

	#[test]
	fn it_works_with_multiple_blocks() {
		let hasher = IpfsHasher::new(2);
		let ipfs = hasher.compute(b"foobar\n").unwrap();
		assert!(ipfs == "QmRJHYTNvC3hmd9gJQARxLR1QMEincccBV53bBw524yyq6");
	}

	#[test]
	#[ignore = "Onchain data..."]
	fn it_computes_a_runtime_ipfs_hash() {
		const POLKADOT_BLOCK20: &str = "0x4d6a0bca208b85d41833a7f35cf73d1ae6974f4bad8ab576e2c3f751d691fe6c"; // Polkadot Block #20

		let ocb = OnchainBlock::new("wss://rpc.polkadot.io:443", Some(POLKADOT_BLOCK20.to_string())).unwrap();
		let loader = WasmLoader::load_from_source(&Source::Chain(ocb)).unwrap();
		let hasher = IpfsHasher::default();
		let cid = hasher.compute(loader.uncompressed_bytes()).unwrap();
		assert!(cid == "QmevKMGkRViXfQMSZ38DBdcJ1cXcXf9sXdfXie8Jkc7ZGs");
	}
}

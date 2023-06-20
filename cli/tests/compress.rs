mod test_utils;

#[cfg(test)]
mod cli_tests {
	#[cfg(test)]
	mod cli_compress {
		use assert_cmd::Command;

		// todo: sporadic errors
		#[test]
		fn it_does_basic_compress_decompress() {
			let test_wasm: &str = &crate::test_utils::ensure_local_wasm();

			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Failed getting test bin");
			cmd.args(["decompress", test_wasm, "/tmp/decompressed.wasm"]).assert().success().code(0);

			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Failed getting test bin");
			cmd.args(["compress", "/tmp/decompressed.wasm", "/tmp/new_compressed.wasm"]).assert().success().code(0);
		}

		#[test]
		fn it_does_decompress_on_already() {
			let test_wasm: &str = &crate::test_utils::ensure_local_wasm();

			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Failed getting test bin");
			cmd.args(["decompress", test_wasm, "/tmp/decompressed.wasm"]).assert().success().code(0);

			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Failed getting test bin");
			cmd.args(["decompress", "/tmp/decompressed.wasm", "/tmp/new_decompressed.wasm"]).assert().success().code(0);
		}
	}
}

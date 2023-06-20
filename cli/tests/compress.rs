mod test_utils;

#[cfg(test)]
mod cli_tests {
	#[cfg(test)]
	mod cli_compress {
		use crate::test_utils::temp_file;
		use assert_cmd::Command;

		#[test]
		fn it_does_basic_compress_decompress() {
			let test_wasm: &str = &crate::test_utils::ensure_local_wasm();
			let tmp_decompressed = temp_file();
			let tmp_recompressed = temp_file();

			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Failed getting test bin");
			cmd.args(["decompress", test_wasm, tmp_decompressed.as_str()]).assert().success().code(0);

			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Failed getting test bin");
			cmd.args(["compress", tmp_decompressed.as_str(), tmp_recompressed.as_str()]).assert().success().code(0);
		}

		#[test]
		fn it_does_decompress_on_already() {
			let test_wasm: &str = &crate::test_utils::ensure_local_wasm();
			let tmp_decompressed = temp_file();
			let tmp_decompressed_again = temp_file();

			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Failed getting test bin");
			cmd.args(["decompress", test_wasm, tmp_decompressed.as_str()]).assert().success().code(0);

			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Failed getting test bin");
			cmd.args(["decompress", tmp_decompressed.as_str(), tmp_decompressed_again.as_str()])
				.assert()
				.success()
				.code(0);
		}
	}
}

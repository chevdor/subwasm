#[cfg(test)]
mod cli_tests {
	#[cfg(test)]
	mod cli_compress {
		use assert_cmd::Command;

		#[test]
		fn it_does_basic_compress_decompress() {
			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
			let assert = cmd.args(["get", "wss://rpc.polkadot.io:443", "--output", "compressed.wasm"]).assert();
			assert.success().code(0);

			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
			cmd.args(["decompress", "compressed.wasm", "decompressed.wasm"]).assert().success().code(0);

			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
			cmd.args(["compress", "decompressed.wasm", "new_compressed.wasm"]).assert().success().code(0);
		}

		#[test]
		fn it_does_decompress_on_already() {
			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
			let assert = cmd.args(["get", "wss://rpc.polkadot.io:443", "--output", "compressed.wasm"]).assert();
			assert.success().code(0);

			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
			cmd.args(["decompress", "compressed.wasm", "decompressed.wasm"]).assert().success().code(0);

			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
			cmd.args(["decompress", "decompressed.wasm", "new_decompressed.wasm"]).assert().success().code(0);
		}
	}
}

#[cfg(test)]
mod cli_tests {

	#[cfg(test)]
	mod help {
		use assert_cmd::Command;

		#[test]
		fn it_shows_help() {
			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
			let assert = cmd.arg("--help").assert();
			assert.success().code(0);
		}
	}

	#[cfg(test)]
	mod info {
		use assert_cmd::Command;
		#[test]
		fn it_fails_without_source() {
			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
			let assert = cmd.arg("info tcp://foo.bar").assert();
			assert.failure().code(2);
		}
	}

	#[cfg(test)]
	mod get {
		use assert_cmd::Command;
		use std::path::Path;
		#[test]
		fn it_gets_a_runtime() {
			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

			let assert = cmd.args(["get", "--output", "runtime.wasm", "wss://rpc.polkadot.io:443"]).assert();
			assert.success().code(0);
			assert!(Path::new("runtime.wasm").exists());
		}

		#[test]
		fn it_fails_on_bad_chain() {
			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

			let assert = cmd.args(["get", "--chain", "foobar"]).assert();
			assert.failure().code(101);
		}
	}

	#[cfg(test)]
	mod meta {
		use assert_cmd::Command;

		#[test]
		fn it_shows_metadata() {
			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
			let assert = cmd.args(["get", "wss://rpc.polkadot.io:443", "--output", "runtime.wasm"]).assert();
			assert.success().code(0);

			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
			let assert = cmd.args(["meta", "runtime.wasm"]).assert();
			assert.success().code(0);
		}
	}

	#[cfg(test)]
	mod compress {
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

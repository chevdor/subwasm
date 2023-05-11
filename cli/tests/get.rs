mod test_utils;

#[cfg(test)]
mod cli_tests {
	#[cfg(test)]
	mod get {
		use assert_cmd::Command;
		use std::path::Path;
		#[test]
		fn it_gets_a_runtime() {
			const TARGET: &str = "/tmp/polkadot_runtime.wasm";
			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

			let assert = cmd.args(["get", "--output", TARGET, "wss://rpc.polkadot.io:443"]).assert();
			assert.success().code(0);
			assert!(Path::new(TARGET).exists());
		}

		#[test]
		fn it_fails_on_bad_chain() {
			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

			let assert = cmd.args(["get", "--chain", "foobar"]).assert();
			assert.failure().code(101);
		}
	}
}

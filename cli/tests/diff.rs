#[cfg(test)]
mod cli_tests {
	#[cfg(test)]
	mod diff {
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
}

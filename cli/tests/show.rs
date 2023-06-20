mod test_utils;
#[cfg(test)]
mod cli_tests {
	#[cfg(test)]
	mod show {
		use assert_cmd::Command as AssertCommand;

		#[test]
		fn it_shows_metadata() {
			let test_wasm: &str = &crate::test_utils::ensure_local_wasm();

			let mut cmd = AssertCommand::cargo_bin(env!("CARGO_PKG_NAME")).expect("Failed getting test bin");
			let assert = cmd.args(["show", "--json"]).arg(test_wasm).assert();
			assert.success().code(0);
		}
	}
}

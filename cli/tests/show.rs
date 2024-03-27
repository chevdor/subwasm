mod test_utils;
#[cfg(test)]
mod cli_tests {

	#[cfg(test)]
	mod show {
		use crate::test_utils::CLI;
		use assert_cmd::Command as AssertCommand;

		#[test]
		fn it_shows_runtime() {
			let test_wasm: &str = &crate::test_utils::ensure_local_wasm();

			let mut cmd = AssertCommand::cargo_bin(CLI).expect("Failed getting test bin");
			let assert = cmd.args(["show", test_wasm]).assert();
			assert.success().code(0);
		}
	}
}

mod test_utils;

#[cfg(test)]
mod cli_tests {

	#[cfg(test)]
	mod diff {
		use assert_cmd::Command;

		use crate::test_utils::CLI;

		#[test]
		fn it_shows_metadata() {
			let test_wasm: &str = &crate::test_utils::ensure_local_wasm();

			let mut cmd = Command::cargo_bin(CLI).expect("Failed getting test bin");
			let assert = cmd.args(["meta", test_wasm]).assert();
			assert.success().code(0);
		}
	}
}

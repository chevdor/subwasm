mod test_utils;

#[cfg(test)]
mod cli_tests {
	#[cfg(test)]
	mod info {
		use assert_cmd::Command;

		use crate::test_utils::CLI;

		#[test]
		fn it_fails_without_source() {
			let mut cmd = Command::cargo_bin(CLI).expect("Failed getting test bin");
			let assert = cmd.arg("info tcp://foo.bar").assert();
			assert.failure().code(2);
		}

		#[test]
		fn it_returns_infos() {
			let test_wasm: &str = &crate::test_utils::ensure_local_wasm();

			let mut cmd = Command::cargo_bin(CLI).expect("Failed getting test bin");
			let assert = cmd.args(["info", test_wasm]).assert();
			assert.success().code(0);
		}
	}
}

#[cfg(test)]
mod cli_tests {
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
}

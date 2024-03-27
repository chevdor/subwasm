#[cfg(test)]
mod cli_tests {
	#[cfg(test)]
	mod help {
		use assert_cmd::Command;
		use subwasmlib::CLI;

		#[test]
		fn it_shows_help() {
			println!("CARGO_PKG_NAME = {:?}", CLI);
			let mut cmd = Command::cargo_bin(CLI).expect("Failed getting test bin");
			println!("cmd = {:?}", cmd);
			let assert = cmd.arg("--help").assert();
			assert.success().code(0);
		}
	}
}

mod test_utils;
#[cfg(test)]
mod cli_tests {
	#[cfg(test)]
	mod meta {
		use assert_cmd::Command as AssertCommand;

		#[test]
		fn it_shows_metadata() {
			let test_wasm: &str = &crate::test_utils::ensure_local_wasm();

			let mut cmd = AssertCommand::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
			let assert = cmd.args(["meta", test_wasm]).assert();
			assert.success().code(0);
		}

		// #[test]
		// fn it_shows_when_piped() {
		// 	// let mut cmd = AssertCommand::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
		// 	// let assert = cmd.args(["get", "wss://rpc.polkadot.io:443", "--output", "runtime.wasm"]).assert();
		// 	// assert.success().code(0);

		// 	let subwasm_process = Command::new(env!("CARGO_PKG_NAME"))
		// 		.args(["meta", "wss://rpc.polkadot.io:443", "-f", "json"])
		// 		.stdout(Stdio::piped())
		// 		.spawn()
		// 		.unwrap();
		// 	let head_process = Command::new("head")
		// 		.stdin(Stdio::from(subwasm_process.stdout.unwrap())) // Pipe through.
		// 		.stdout(Stdio::piped())
		// 		.spawn()
		// 		.unwrap();
		// 	let output = head_process.wait_with_output().unwrap();

		// 	let s = String::from_utf8_lossy(&output.stdout);
		// 	println!("s = {:?}", s);
		// 	println!("s = {:?}", s.len());
		// 	assert!(s.len() > 100)
		// }
	}
}

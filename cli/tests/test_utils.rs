use assert_cmd::Command;
use core::panic;
use std::path::PathBuf;

#[allow(dead_code)]
#[cfg(test)]
pub fn ensure_local_wasm() -> String {
	const MAX_RETRIES: u8 = 10;
	const WASM_FILE: &str = "/tmp/runtime.wasm";
	let mut retry = 0;

	let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

	if PathBuf::from(WASM_FILE).exists() {
		println!("Found an existing runtime, reusing...");
		return WASM_FILE.to_string();
	} else {
		while retry < MAX_RETRIES {
			let assert = cmd.args(["get", "--chain", "wss://rpc.polkadot.io:443", "--output", WASM_FILE]).assert();

			if assert.try_success().is_ok() {
				println!("Successfully retrived a runtime");
				return String::from(WASM_FILE);
			}
			eprintln!("Fetching Runtime failed, trying again...");
			retry += 1
		}
	}

	panic!("Failed fetching a runtime")
}

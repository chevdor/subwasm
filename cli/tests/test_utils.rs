use assert_cmd::Command;
use core::panic;
use std::{env::temp_dir, path::PathBuf};

#[cfg(test)]
pub const CLI: &str = "subwasm";

#[allow(dead_code)]
#[cfg(test)]
/// Ensure we have a local wasm available at /tpm/runtime.wasm
pub fn ensure_local_wasm() -> String {
	const MAX_RETRIES: u8 = 10;
	const WASM_FILE: &str = "/tmp/runtime.wasm";
	let mut retry = 0;

	let mut cmd = Command::cargo_bin(CLI).expect("Failed getting test bin");

	if PathBuf::from(WASM_FILE).exists() {
		println!("Found an existing runtime, reusing...");
		return WASM_FILE.to_string();
	} else {
		while retry < MAX_RETRIES {
			let assert = cmd.args(["get", "wss://rpc.polkadot.io:443", "--output", WASM_FILE]).assert();

			if assert.try_success().is_ok() {
				println!("Successfully retrieved a runtime");
				return String::from(WASM_FILE);
			}
			eprintln!("Fetching Runtime failed, trying again...");
			retry += 1
		}
	}

	panic!("Failed fetching a runtime")
}

/// Generate the path of a temp file
#[allow(dead_code)]
#[cfg(test)]
pub fn temp_file() -> String {
	let mut res = PathBuf::from(temp_dir());
	res.push(PathBuf::from(uuid::Uuid::new_v4().to_string()));
	String::from(res.to_str().expect("Failed generating temp file path"))
}

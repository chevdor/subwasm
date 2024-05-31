use chrono::{DateTime, Utc};

use std::{borrow::Cow, process::Command};

fn main() {
	generate_cargo_keys();
}

/// Generate the cargo keys
pub fn generate_cargo_keys() {
	generate_cargo_key_git();
	generate_cargo_key_build_date();
}

pub fn generate_cargo_key_build_date() {
	let build_date = std::env::var("SOURCE_DATE_EPOCH")
		.ok()
		.and_then(|ts| ts.parse::<i64>().ok())
		.and_then(|ts| DateTime::from_timestamp(ts, 0))
		.unwrap_or_else(|| Utc::now());

	let formatted_build_date = build_date.format("%Y-%m-%dT%H:%M:%SZ").to_string();

	println!("cargo:rustc-env=SUBWASM_CLI_BUILD_DATE={formatted_build_date}");
}

pub fn generate_cargo_key_git() {
	let commit = if let Ok(hash) = std::env::var("SUBWASM_CLI_GIT_COMMIT_HASH") {
		Cow::from(hash.trim().to_owned())
	} else {
		match Command::new("git").args(["rev-parse", "--short=11", "HEAD"]).output() {
			Ok(o) if o.status.success() => {
				let tmsp = String::from_utf8_lossy(&o.stdout).trim().to_owned();
				Cow::from(tmsp)
			}
			Ok(o) => {
				let status = o.status;
				println!("cargo:warning=Git command failed with status: {status}");
				Cow::from("unknown")
			}
			Err(err) => {
				println!("cargo:warning=Failed to execute git command: {err}");
				Cow::from("unknown")
			}
		}
	};

	println!("cargo:rustc-env=SUBWASM_CLI_GIT_COMMIT_HASH={commit}");
}

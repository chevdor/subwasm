use anyhow::bail;

// TODO: Once the lined issue is fixed, we can remove the dependency on calm_io
/// There is a bug caused by printing big output to stdout.
///
/// We need to take extra precautions due to the following bug:
/// https://github.com/rust-lang/rust/issues/46016
pub fn print_big_output_safe(s: &str) -> anyhow::Result<()> {
	// The following fails if piped to another command that truncates the output.
	// println!("{}", s);
	// Typical use cases here are:
	// 	- subwasm meta --chain westend --json | head
	// 	- subwasm meta --chain westend --json | less
	match calm_io::stdoutln!("{}", s) {
		Ok(_) => Ok(()),
		Err(e) => match e.kind() {
			std::io::ErrorKind::BrokenPipe => Ok(()),
			_ => bail!(e),
		},
	}
}

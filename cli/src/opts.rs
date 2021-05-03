use clap::{crate_authors, crate_version, AppSettings, Clap};
use std::path::PathBuf;

/// `subwasm` allows fetching, parsing and calling some methods on WASM runtimes of Substrate based chains.
#[derive(Clap)]
#[clap(version = crate_version!(), author = crate_authors!())]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
	/// A level of verbosity, and can be used multiple times
	#[clap(short, long, parse(from_occurrences))]
	pub _verbose: i32,

	#[clap(subcommand)]
	pub subcmd: SubCommand,
}

/// You can find all available commands below.
#[derive(Clap)]
pub enum SubCommand {
	#[clap(version = crate_version!(), author = crate_authors!())]
	Get(GetOpts),

	// #[clap(version = crate_version!(), author = crate_authors!())]
	// Diff(DiffOpts),
	#[clap(version = crate_version!(), author = crate_authors!())]
	Info(InfoOpts),

	#[clap(version = crate_version!(), author = crate_authors!(), alias("meta"))]
	Metadata(MetaOpts),
}

/// The `info` command returns summarized information about a runtime.
#[derive(Clap)]
pub struct InfoOpts {
	/// The wasm file to load
	#[clap(short, long, default_value = "runtime.wasm")]
	pub input: PathBuf,

	/// The more `-d` you add and the more you see... Try `-d` or `-dd` or `-ddd` or ... `-dddd`
	#[clap(short, long("details-level"), parse(from_occurrences))]
	pub details_level: i32,
}

/// Returns the metadata as a json object. You may also use the "meta" alias.
#[derive(Clap)]
pub struct MetaOpts {
	/// The wasm file to load
	#[clap(short, long, default_value = "runtime.wasm")]
	pub input: PathBuf,
}

/// Get/Download the runtime wasm from a running node through rpc
#[derive(Clap)]
pub struct GetOpts {
	/// The node url. Example: ws://localhost:9944 or http://localhost:9933.
	#[clap(short, long, default_value = "http://localhost:9933")]
	pub url: String,

	/// The optional block where to fetch the runtime. That allows fetching older runtimes but you will need to connect to archive nodes.
	/// Currently, you must pass a block hash. Passing the block numbers is not supported.
	#[clap(short, long)]
	pub block: Option<String>, // TODO: can do better...

	/// You may specifiy the output filename where the runtime will be saved. If not provided, we will figure out an appropriate default name
	/// based on a counter: runtime_NNN.wasm where NNN is incrementing to make sure you do not override previous runtime. If you specify an
	/// existing file as output, it will be overwritten.
	#[clap(short, long, parse(from_os_str))]
	pub output: Option<PathBuf>,
}

// /// Compare 2 runtimes. NOT SUPPORTED YET.
// #[derive(Clap)]
// struct DiffOpts {}

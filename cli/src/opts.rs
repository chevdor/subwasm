use clap::{crate_authors, crate_version, AppSettings, Clap};
use std::path::PathBuf;
use subwasmlib::ChainInfo;
use wasm_loader::{OnchainBlock, Source};

/// `subwasm` allows fetching, parsing and calling some methods on WASM runtimes of Substrate based chains.
#[derive(Clap)]
#[clap(version = crate_version!(), author = crate_authors!())]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
	/// Output as json
	#[clap(short, long)]
	pub json: bool,

	/// Less output
	#[clap(short, long)]
	pub quiet: bool,

	#[clap(subcommand)]
	pub subcmd: SubCommand,
}

/// You can find all available commands below.
#[derive(Clap)]
pub enum SubCommand {
	#[clap(version = crate_version!(), author = crate_authors!())]
	Get(GetOpts),

	#[clap(version = crate_version!(), author = crate_authors!())]
	Info(InfoOpts),

	#[clap(version = crate_version!(), author = crate_authors!(), alias("meta"))]
	Metadata(MetaOpts),

	#[clap(version = crate_version!(), author = crate_authors!())]
	Diff(DiffOpts),
}

/// Get/Download the runtime wasm from a running node through rpc
#[derive(Clap)]
pub struct GetOpts {
	/// The node url including (mandatory) the port number. Example: ws://localhost:9944 or http://localhost:9933
	#[clap(default_value = "http://localhost:9933", required_unless_present = "chain", index = 1)]
	pub url: OnchainBlock,

	/// Provide the name of a chain and a random url amongst a list of known nodes will be used.
	/// If you pass a valid --chain, --url will be ignored
	/// --chain local = http://localhost:9933
	#[clap(long, parse(from_str), conflicts_with = "url")]
	pub chain: Option<ChainInfo>,

	/// The optional block where to fetch the runtime. That allows fetching older runtimes but you will need to connect to archive nodes.
	/// Currently, you must pass a block hash. Passing the block numbers is not supported.
	#[clap(short, long)]
	pub block: Option<String>, // TODO: can do better...

	/// You may specifiy the output filename where the runtime will be saved. If not provided, we will figure out an appropriate default name
	/// based on a counter: runtime_NNN.wasm where NNN is incrementing to make sure you do not override previous runtime. If you specify an
	/// existing file as output, it will be overwritten.
	#[clap(short, long, alias("out"), parse(from_os_str))]
	pub output: Option<PathBuf>,
}

/// The `info` command returns summarized information about a runtime.
#[derive(Clap)]
pub struct InfoOpts {
	/// The wasm file to load. It can be a path on your local filesystem such as
	/// /tmp/runtime.wasm or a node url such as http://localhost:9933 or ws://localhost:9944
	#[clap(alias("src"), default_value = "runtime_000.wasm", required_unless_present = "chain", index = 1)]
	pub source: Source,

	/// Provide the name of a chain and a random url amongst a list of known nodes will be used.
	/// If you pass a valid --chain, --url will be ignored
	/// --chain local = http://localhost:9933
	#[clap(long, parse(from_str), conflicts_with = "source")]
	pub chain: Option<ChainInfo>,

	#[clap(short, long)]
	pub block: Option<String>, // TODO: can do better...
}

/// Returns the metadata as a json object. You may also use the "meta" alias.
#[derive(Clap)]
pub struct MetaOpts {
	/// The wasm file to load. It can be a path on your local filesystem such as
	/// /tmp/runtime.wasm or a node url such as http://localhost:9933 or ws://localhost:9944
	#[clap(alias("src"), default_value = "runtime_000.wasm", required_unless_present = "chain", index = 1)]
	pub source: Source,

	/// Provide the name of a chain and a random url amongst a list of known nodes will be used.
	/// If you pass a valid --chain, --url will be ignored
	/// --chain local = http://localhost:9933
	#[clap(long, parse(from_str), conflicts_with = "source")]
	pub chain: Option<ChainInfo>,

	/// Without this flag, the metadata command display the list of all modules.
	/// Using this flag, you will only see the module of your choice and a few details about it.
	#[clap(long, short)]
	pub module: Option<String>,

	#[clap(short, long)]
	pub block: Option<String>, // TODO: can do better...

	/// Output as json
	#[clap(short, long)]
	pub json: bool,
}

/// Compare 2 runtimes
#[derive(Clap)]
pub struct DiffOpts {
	/// The first source
	#[clap(index = 1, alias = "src-a", default_value = "runtime_000.wasm", required_unless_present = "chain-a")]
	pub src_a: Source,

	/// Provide the name of a chain and a random url amongst a list of known nodes will be used.
	/// If you pass a valid --chain, --url will be ignored
	/// --chain local = http://localhost:9933
	#[clap(long, parse(from_str), conflicts_with = "src")]
	pub chain: Option<ChainInfo>,


	/// Provide the name of a chain and a random url amongst a list of known nodes will be used.
	/// If you pass a valid --chain, --url will be ignored
	/// --chain local = http://localhost:9933
	#[clap(long, short('a'), parse(from_str), conflicts_with = "src-a")]
	pub chain_a: Option<ChainInfo>,

	/// The second source
	#[clap(index = 2, alias = "src-b", default_value = "runtime_001.wasm", required_unless_present = "chain-b")]
	pub src_b: Source,

	/// Provide the name of a chain and a random url amongst a list of known nodes will be used.
	/// If you pass a valid --chain, --url will be ignored
	/// --chain local = http://localhost:9933
	#[clap(long, short('b'), parse(from_str), conflicts_with = "src-b")]
	pub chain_b: Option<ChainInfo>,
}

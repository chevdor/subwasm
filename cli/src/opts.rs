use clap::{crate_authors, crate_version, ColorChoice, Parser, Subcommand};
use std::path::PathBuf;
use subwasmlib::{source::Source, *};
use url::Url;
use wasm_loader::{BlockRef, OnchainBlock};

use crate::error::{self, *};

/// `subwasm` allows fetching, parsing and calling some methods on WASM runtimes of Substrate based chains.
#[derive(Parser)]
#[clap(color=ColorChoice::Auto, disable_version_flag = true, arg_required_else_help = true )]
pub struct Opts {
	/// Output as json
	#[clap(short, long, global = true)]
	pub json: bool,

	/// Less output
	#[clap(short, long, global = true)]
	pub quiet: bool,

	#[clap(short, long, global = true, env = "NO_COLOR")]
	pub no_color: bool,

	#[clap(subcommand)]
	pub subcmd: Option<SubCommand>,

	/// Show the version
	#[clap(short, long, alias = "V")]
	pub version: bool,
}

/// You can find all available commands below.
#[derive(Subcommand)]
pub enum SubCommand {
	#[clap(version = crate_version!(), author = crate_authors!())]
	Get(GetOpts),

	/// The `info` command returns summarized information about a runtime.
	#[clap(version = crate_version!(), author = crate_authors!())]
	Info(InfoOpts),

	/// The `version` command returns summarized information about the versions of a runtime.
	#[clap(version = crate_version!(), author = crate_authors!())]
	Version(InfoOpts),

	#[clap(version = crate_version!(), author = crate_authors!(), alias("meta"))]
	Metadata(MetaOpts),

	#[clap(version = crate_version!(), author = crate_authors!())]
	Show(ShowOpts),

	#[clap(version = crate_version!(), author = crate_authors!())]
	Diff(DiffOpts),

	#[clap(version = crate_version!(), author = crate_authors!())]
	Compress(CompressOpts),

	#[clap(version = crate_version!(), author = crate_authors!())]
	Decompress(DecompressOpts),
}

/// Get/Download the runtime wasm from a running node through rpc
#[derive(Parser)]
pub struct GetOpts {
	/// The node url including (mandatory) the port number. Example: ws://localhost:9944 or http://localhost:9933
	#[clap(default_value = "http://localhost:9933", required_unless_present = "chain", index = 1)]
	pub url: OnchainBlock,

	/// Provide the name of a chain and a random url amongst a list of known nodes will be used.
	///
	/// If you pass a valid --chain, --url will be ignored
	/// --chain local = http://localhost:9933
	#[clap(long, conflicts_with = "url")]
	pub chain: Option<ChainInfo>,

	/// The optional block where to fetch the runtime.
	///
	/// That allows fetching older runtimes but you will need to connect to archive nodes.
	/// Currently, you must pass a block hash. Passing the block numbers is not supported.
	#[clap(short, long)]
	pub block: Option<BlockRef>,

	/// You may specifiy the output filename where the runtime will be saved.
	///
	/// If not provided, we will figure out an appropriate default name
	/// based on a counter: runtime_NNN.wasm where NNN is incrementing to make sure you do not override previous runtime. If you specify an
	/// existing file as output, it will be overwritten.
	#[clap(short, long, alias("out"), value_parser)]
	pub output: Option<PathBuf>,
}

#[derive(Parser)]
pub struct InfoOpts {
	/// The wasm file to load. It can be a path on your local filesystem such /tmp/runtime.wasm
	///
	/// You may also fetch the runtime remotely, see `chain` and `url` flags.
	#[clap(required_unless_present_any = ["chain", "url", "github"], required_unless_present = "url", index = 1)]
	pub file: Option<PathBuf>,

	/// Load the wasm from an RPC node url such as http://localhost:9933 or ws://localhost:9944,
	/// a node alias such as "polkadot" or "dot",
	///
	/// NOTE: --chain local = http://localhost:9933
	#[clap(long, short, conflicts_with = "file")]
	pub chain: Option<ChainInfo>,

	/// The optional block where to fetch the runtime. That allows fetching older runtimes but you will need to connect to archive nodes.
	/// Currently, you must pass a block hash. Passing the block numbers is not supported.
	#[clap(short, long, requires = "chain")]
	pub block: Option<BlockRef>,

	/// Load the wasm from a URL (no node) such as https://github.com/paritytech/polkadot/releases/download/v0.9.42/polkadot_runtime-v9420.compact.compressed.wasm
	#[clap(long, short, conflicts_with = "file")]
	pub url: Option<Url>,

	/// Load the wasm from Github passing a string in the format <runtime>@<version>
	/// such as `kusama@0.9.42`
	#[clap(long, short, conflicts_with = "file")]
	pub github: Option<String>,
	// /// Load the wasm from IPFS
	// #[clap(long, short, conflicts_with = "file")]
	// pub ipfs: Option<String>,
}

/// Returns the metadata as a json object. You may also use the "meta" alias.
/// See also the 'show' sub-command.
#[derive(Parser)]
pub struct MetaOpts {
	/// The wasm file to load. It can be a path on your local filesystem such as
	/// /tmp/runtime.wasm or a node url such as http://localhost:9933 or ws://localhost:9944
	#[clap(alias("src"), default_value = "runtime_000.wasm", required_unless_present = "chain", index = 1, value_parser = parse_source)]
	pub source: Source,

	/// Provide the name of a chain and a random url amongst a list of known nodes will be used.
	/// If you pass a valid --chain, --url will be ignored
	/// --chain local = http://localhost:9933
	#[clap(long, conflicts_with = "source")]
	pub chain: Option<ChainInfo>,

	/// Without this flag, the metadata command display the list of all modules.
	/// Using this flag, you will only see the module of your choice and a few details about it.
	#[clap(long, short)]
	pub module: Option<String>,

	/// The optional block where to fetch the runtime. That allows fetching older runtimes but you will need to connect to archive nodes.
	/// Currently, you must pass a block hash. Passing the block numbers is not supported.
	#[clap(short, long)]
	pub block: Option<BlockRef>,

	/// You may specifiy the output format. One of "human", "scale", "json", "json+scale", "hex+scale".
	/// If you use the default: human, you may want to check out the "show_reduced" command instead.
	#[clap(long, short, default_value = "human")]
	pub format: Option<String>,

	/// You may specifiy the output filename where the metadata will be saved.
	/// Alternatively, you may use `auto` and an appropriate name will be generated according to the `format` your chose.
	#[clap(short, long)]
	pub output: Option<String>,
}

/// Compare 2 runtimes after converting them to ReducedRuntime.
#[derive(Parser)]
pub struct DiffOpts {
	/// The first source
	#[clap(index = 1, alias = "src-a", value_parser = parse_source)]
	pub src_a: Source,

	// /// Provide the name of a chain and a random url amongst a list of known nodes will be used.
	// /// If you pass a valid --chain, --url will be ignored
	// /// --chain local = http://localhost:9933
	// #[clap(long, short('a'), conflicts_with = "src_a")]
	// pub chain_a: Option<ChainInfo>,
	/// The second source
	#[clap(index = 2, alias = "src-b", value_parser = parse_source)]
	pub src_b: Source,

	// /// Provide the name of a chain and a random url amongst a list of known nodes will be used.
	// /// If you pass a valid --chain, --url will be ignored
	// /// --chain local = http://localhost:9933
	// #[clap(long, short('b'), conflicts_with = "src_b")]
	// pub chain_b: Option<ChainInfo>,
	/// You probably want to use `Reduced`.
	#[clap(long, short, default_value = "reduced")]
	pub method: DiffMethod,
}

/// Shows the a reduced view of the runtime where the types have been resolved.
#[derive(Parser)]
pub struct ShowOpts {
	/// The first source
	#[clap(index = 1, alias = "src", default_value = "runtime_000.wasm", required_unless_present = "chain", value_parser = parse_source)]
	pub src: Source,

	/// Provide the name of a chain and a random url amongst a list of known nodes will be used.
	/// If you pass a valid --chain, --url will be ignored
	/// --chain local = http://localhost:9933
	#[clap(long, conflicts_with = "src")]
	pub chain: Option<ChainInfo>,

	/// The optional block where to fetch the runtime. That allows fetching older runtimes but you will need to connect to archive nodes.
	/// Currently, you must pass a block hash. Passing the block numbers is not supported.
	#[clap(short, long)]
	pub block: Option<BlockRef>,

	/// Show only information related to the provided pallet
	#[clap(long, short)]
	pub pallet: Option<String>,

	#[clap(long, short)]
	pub summary: bool,
}

/// Compress a given runtime wasm file.
/// You will get an error if you try compressing a runtime that is already compressed.
#[derive(Parser)]
pub struct CompressOpts {
	/// The path of uncompressed wasm file to load.
	#[clap(alias("in"), index = 1)]
	pub input: PathBuf,

	/// The path of the file where the compressed runtime will be stored.
	#[clap(alias("out"), index = 2)]
	pub output: PathBuf,
}

/// Decompress a given runtime wasm file. You may pass a runtime that is already uncompressed.
///
/// In that case, you will get the same content as output. This is useful
/// if you want to decompress "no matter what" and don't really know whether the input
/// will be compressed or not.
#[derive(Parser)]
pub struct DecompressOpts {
	/// The path of the compressed or uncompressed wasm file to load.
	#[clap(alias("in"), index = 1)]
	pub input: PathBuf,

	/// The path of the file where the uncompressed runtime will be stored.
	#[clap(alias("out"), index = 2)]
	pub output: PathBuf,
}

fn parse_source(s: &str) -> error::Result<Source> {
	Source::try_from(s).map_err(|_e| SubwasmError::SourceParseError(s.into()))
}

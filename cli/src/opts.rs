use clap::{crate_authors, crate_version, ColorChoice, Parser, Subcommand};
use std::path::PathBuf;
use subwasmlib::{source::Source, *};
use url::Url;
use wasm_loader::{BlockRef, OnchainBlock};

use crate::error;

/// `subwasm` allows fetching, parsing and calling some methods on WASM runtimes of Substrate based chains.
#[derive(Parser, Debug)]
#[clap(color=ColorChoice::Auto, disable_version_flag = true, arg_required_else_help = true )]
pub struct Opts {
	/// Output as json
	#[clap(short, long, global = true, display_order = 99)]
	pub json: bool,

	/// Less output
	#[clap(short, long, global = true, display_order = 99)]
	pub quiet: bool,

	#[clap(short, long, global = true, env = "NO_COLOR", display_order = 99)]
	pub no_color: bool,

	#[clap(subcommand)]
	pub subcmd: Option<SubCommand>,

	/// Show the version
	#[clap(short, long, alias = "V")]
	pub version: bool,
}

/// You can find all available commands below.
#[derive(Subcommand, Debug)]
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
#[derive(Parser, Debug)]
pub struct GetOpts {
	/// The node url including (mandatory) the port number. Example: ws://localhost:9944 or http://localhost:9933
	#[clap(required_unless_present_any = ["chain", "url", "github"], index = 1)]
	pub rpc_url: Option<OnchainBlock>,

	/// Provide the name of a chain or an alias.
	///
	/// If you pass a valid --chain, --rpc_url will be ignored
	/// --chain local = http://localhost:9933
	#[clap(short, long, conflicts_with = "rpc_url")]
	pub chain: Option<ChainInfo>,

	/// The optional block where to fetch the runtime.
	///
	/// That allows fetching older runtimes but you will need to connect to archive nodes.
	/// Currently, you must pass a block hash. Passing the block numbers is not supported.
	#[clap(short, long, requires = "chain")]
	pub block: Option<BlockRef>,

	/// Load the wasm from a URL (no node) such as https://github.com/paritytech/polkadot/releases/download/v0.9.42/polkadot_runtime-v9420.compact.compressed.wasm
	#[clap(long, short, conflicts_with = "rpc_url")]
	pub url: Option<Url>,

	/// Load the wasm from Github passing a string in the format <runtime>@<version>
	/// such as `kusama@0.9.42`
	#[clap(long, short, alias = "gh", conflicts_with = "rpc_url")]
	pub github: Option<String>,
	// /// Load the wasm from IPFS
	// #[clap(long, short, conflicts_with = "file")]
	// pub ipfs: Option<String>,
	/// You may specifiy the output filename where the runtime will be saved.
	///
	/// If not provided, we will figure out an appropriate default name
	/// based on a counter: runtime_NNN.wasm where NNN is incrementing to make sure you do not override previous runtime. If you specify an
	/// existing file as output, it will be overwritten.
	#[clap(short, long, alias("out"), value_parser)]
	pub output: Option<PathBuf>,
}

#[derive(Parser, Debug)]
pub struct InfoOpts {
	/// The wasm file to load. It can be a path on your local filesystem such /tmp/runtime.wasm
	///
	/// You may also fetch the runtime remotely, see `chain` and `url` flags.
	#[clap(required_unless_present_any = ["chain", "url", "github"], index = 1)]
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
	#[clap(long, short, alias = "gh", conflicts_with = "file")]
	pub github: Option<String>,
	// /// Load the wasm from IPFS
	// #[clap(long, short, conflicts_with = "file")]
	// pub ipfs: Option<String>,
}

/// Returns the metadata of the given runtime in several format. You may also use the "meta" alias.
///
/// If you want to see the content of a runtime, see the `show` sub-command.
#[derive(Parser, Debug)]
pub struct MetaOpts {
	/// The wasm file to load. It can be a path on your local filesystem such as
	/// /tmp/runtime.wasm or a node url such as http://localhost:9933 or ws://localhost:9944
	#[clap(required_unless_present_any = ["chain", "url", "github"], index = 1)]
	pub file: Option<PathBuf>,

	/// Provide the name of a chain and a random url amongst a list of known nodes will be used.
	/// If you pass a valid --chain, --url will be ignored
	/// --chain local = http://localhost:9933
	#[clap(long, short, conflicts_with = "file")]
	pub chain: Option<ChainInfo>,

	/// Load the wasm from a URL (no node) such as https://github.com/paritytech/polkadot/releases/download/v0.9.42/polkadot_runtime-v9420.compact.compressed.wasm
	#[clap(long, short, conflicts_with = "file")]
	pub url: Option<Url>,

	/// Load the wasm from Github passing a string in the format <runtime>@<version>
	/// such as `kusama@0.9.42`
	#[clap(long, short, alias = "gh", conflicts_with = "file")]
	pub github: Option<String>,

	/// The optional block where to fetch the runtime. That allows fetching older runtimes but you will need to connect to archive nodes.
	/// Currently, you must pass a block hash. Passing the block numbers is not supported.
	#[clap(short, long, requires = "chain")]
	pub block: Option<BlockRef>,

	/// Without this flag, the metadata command display the list of all modules.
	/// Using this flag, you will only see the module of your choice and a few details about it.
	#[clap(long, short)]
	pub module: Option<String>,

	/// You may specify the output format. One of "human", "scale", "json", "json+scale", "hex+scale".
	/// If you use the default: human, you may want to check out the "show_reduced" command instead.
	#[clap(long, short, default_value = "human")]
	pub format: Option<String>,

	/// You may specifiy the output filename where the metadata will be saved.
	/// Alternatively, you may use `auto` and an appropriate name will be generated according to the `format` your chose.
	#[clap(short, long)]
	pub output: Option<String>,
}

/// Compare 2 runtimes after converting them to `ReducedRuntime`s.
///
/// You must pass exactly 2 runtimes.
#[derive(Parser, Debug)]
pub struct DiffOpts {
	// NOTE: Here I would prefer to remain specific and support the args shown commented out
	// below but clap+derive does not seem to be able to provide the position/index_of the passed
	// args to differentiate between `<cli> diff --file file --url url` and `<cli> diff --url url --file file`

	// /// File(s) to compare
	// #[clap(short, long)]
	// pub file: Vec<PathBuf>,

	// /// Chain(s) to compare
	// #[clap(short, long)]
	// pub chain: Vec<ChainInfo>,

	// /// Url(s) to compare
	// #[clap(short, long)]
	// pub url: Vec<Url>,

	// /// Github reference(s) to compare, in the format <runtime>@<version>. For instance 'kusama@0.9.42'
	// #[clap(short, long, alias = "gh")]
	// pub github: Vec<String>,
	/// Reference runtime
	#[clap(index=1, value_parser = parse_source)]
	pub runtime_1: Source,

	/// Second runtime
	#[clap(index=2, value_parser = parse_source)]
	pub runtime_2: Source,
}

/// Shows the a reduced view of the runtime.
///
/// A reduced view makes it much easier to understand the inner workings of a given runtime.
#[derive(Parser, Debug)]
pub struct ShowOpts {
	/// The runtimwe to analyze
	#[clap(required_unless_present_any = ["chain", "url", "github"], index = 1)]
	pub file: Option<PathBuf>,

	/// Provide the name of a chain and a random url amongst a list of known nodes will be used.
	/// If you pass a valid --chain, --url will be ignored
	/// --chain local = http://localhost:9933
	#[clap(long, conflicts_with = "file")]
	pub chain: Option<ChainInfo>,

	/// The optional block where to fetch the runtime. That allows fetching older runtimes but you will need to connect to archive nodes.
	/// Currently, you must pass a block hash. Passing the block numbers is not supported.
	#[clap(short, long)]
	pub block: Option<BlockRef>,

	/// Load the wasm from a URL (no node) such as https://github.com/paritytech/polkadot/releases/download/v0.9.42/polkadot_runtime-v9420.compact.compressed.wasm
	#[clap(long, short, conflicts_with = "file")]
	pub url: Option<Url>,

	/// Load the wasm from Github passing a string in the format <runtime>@<version>
	/// such as `kusama@0.9.42`
	#[clap(long, short, alias = "gh", conflicts_with = "file")]
	pub github: Option<String>,

	/// Show only information related to the provided pallet
	#[clap(long, short)]
	pub pallet: Option<String>,

	/// The runtime is shown as a table, listing all pallets with their IDs, the count of calls, events, errors, constants and storage items.
	#[clap(long, short)]
	pub summary: bool,
}

/// Compress a given runtime wasm file.
/// You will get an error if you try compressing a runtime that is already compressed.
#[derive(Parser, Debug)]
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
#[derive(Parser, Debug)]
pub struct DecompressOpts {
	/// The path of the compressed or uncompressed wasm file to load.
	#[clap(alias("in"), index = 1)]
	pub input: PathBuf,

	/// The path of the file where the uncompressed runtime will be stored.
	#[clap(alias("out"), index = 2)]
	pub output: PathBuf,
}

// // TODO: Remove that
fn parse_source(s: &str) -> error::Result<Source> {
	Source::try_from(s).map_err(|_e| error::SubwasmError::SourceParseError(s.to_string()))
}

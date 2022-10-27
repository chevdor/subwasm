# subwasm

![badge](https://github.com/chevdor/subwasm/actions/workflows/quick-check.yml/badge.svg?branch=master)

![Logo 1024](resources/logos/Logo_1024.png)

Unfortunately, the crate of the project and its dependencies are squatted on crates.io. The squatter remains unresponsive and the crates.io team does not care. I strongly recommend NOT to install the old version from crates.io. Check the [Install](#_install) chapter below to see how to install from this repo or use the official binary packages.

## Introduction

The metadata of a given runtime is a critical piece of information as it describes how one can interact with the runtime, what operations are possible and what are the signatures of the calls. It contains the exhaustive list of all the features publicly exposed by the runtime.

This tool only works with Substrate-based runtimes which are `>=V11`. For earlier versions, you’ll need to connect to an archive node.

Any node can be queried to provide its current metadata. This can be displayed in json format for instance. This is a great way to have a peek at what the runtime can do.

## Sample runs

![subwasm get](scripts/demos/gif/demo-get.gif)

![subwasm info](scripts/demos/gif/demo-info.gif)

![subwasm meta](scripts/demos/gif/demo-meta.gif)

![subwasm diff](scripts/demos/gif/demo-diff.gif)

## Capabilities

`subwasm` allows:

-   getting the latest metadata from a running node

-   getting the latest runtime (wasm) from a running node

-   getting runtime and metadata at any point of time using a Block hash as reference

-   getting the metadata from a a wasm file without any node

-   extracting information (those not requiring storage) from a runtime as wasm file

-   (de)compress a given runtime WASM

## Install

### Using Cargo

    cargo install --locked --git https://github.com/chevdor/subwasm --tag v0.16.1

### Homebrew

MacOS Homebrew users can use:

    brew tap chevdor/subwasm https://github.com/chevdor/subwasm
    brew install subwasm

### Linux

    wget https://github.com/chevdor/subwasm/releases/download/v0.16.1/subwasm_linux_amd64_v0.16.1 -O subwasm.deb
    sudo dpkg -i subwasm.deb
    subwasm --help

## Usage

### Command: --help

    [0m`subwasm` allows fetching, parsing and calling some methods on WASM runtimes of Substrate based chains[0m[0m
    [0m[0m
    [0m[0m[1m[4mUsage:[0m[0m [0m[0m[1msubwasm[0m[0m [OPTIONS][0m[0m <[0m[0mCOMMAND[0m[0m>[0m[0m

    [0m[0m[1m[4mCommands[0m[0m[1m[4m:
    [0m[0m  [0m[0m[1mget[0m[0m           [0m[0mGet/Download the runtime wasm from a running node through rpc[0m[0m
    [0m[0m  [0m[0m[1minfo[0m[0m          [0m[0mThe `info` command returns summarized information about a runtime[0m[0m
    [0m[0m  [0m[0m[1mversion[0m[0m       [0m[0mThe `version` command returns summarized information about the versions of a runtime[0m[0m
    [0m[0m  [0m[0m[1mmetadata[0m[0m      [0m[0mReturns the metadata as a json object. You may also use the "meta" alias. It is no longer possible to have a "printable" output, for that, please use the 'show-reduced' sub-command[0m[0m
    [0m[0m  [0m[0m[1mshow-reduced[0m[0m  [0m[0mShows the ReducedRuntime[0m[0m
    [0m[0m  [0m[0m[1mdiff[0m[0m          [0m[0mCompare 2 runtimes after converting them to ReducedRuntime[0m[0m
    [0m[0m  [0m[0m[1mcompress[0m[0m      [0m[0mCompress a given runtime wasm file. You will get an error if you try compressing a runtime that is already compressed[0m[0m
    [0m[0m  [0m[0m[1mdecompress[0m[0m    [0m[0mDecompress a given runtime wasm file. You may pass a runtime that is uncompressed already. In that case, you will get the same content as output. This is useful if you want to decompress "no matter what" and don't really know whether the input will be compressed or not[0m[0m
    [0m[0m  [0m[0m[1mhelp[0m[0m          [0m[0mPrint this message or the help of the given subcommand(s)[0m[0m

    [0m[0m[1m[4mOptions:
    [0m[0m  [0m[0m[1m-j[0m[0m, [0m[0m[1m--json[0m[0m      [0m[0mOutput as json[0m[0m
    [0m[0m  [0m[0m[1m-q[0m[0m, [0m[0m[1m--quiet[0m[0m     [0m[0mLess output[0m[0m
    [0m[0m  [0m[0m[1m-n[0m[0m, [0m[0m[1m--no-color[0m[0m  [0m[0m[env: NO_COLOR=][0m[0m
    [0m[0m  [0m[0m[1m-h[0m[0m, [0m[0m[1m--help[0m[0m      [0m[0mPrint help information[0m[0m
    [0m[0m  [0m[0m[1m-V[0m[0m, [0m[0m[1m--version[0m[0m   [0m[0mPrint version information[0m[0m
    [0m

### Command: get

    [0mGet/Download the runtime wasm from a running node through rpc[0m[0m
    [0m[0m
    [0m[0m[1m[4mUsage:[0m[0m [0m[0m[1msubwasm get[0m[0m [OPTIONS][0m[0m [0m[0m[URL][0m[0m

    [0m[0m[1m[4mArguments:
    [0m[0m  [0m[0m[URL][0m[0m  [0m[0mThe node url including (mandatory) the port number. Example: ws://localhost:9944 or http://localhost:9933[0m[0m [0m[0m[default: http://localhost:9933][0m[0m

    [0m[0m[1m[4mOptions:
    [0m[0m  [0m[0m    [0m[0m[1m--chain[0m[0m [0m[0m<CHAIN>[0m[0m    [0m[0mProvide the name of a chain and a random url amongst a list of known nodes will be used. If you pass a valid --chain, --url will be ignored --chain local = http://localhost:9933[0m[0m
    [0m[0m  [0m[0m[1m-j[0m[0m, [0m[0m[1m--json[0m[0m             [0m[0mOutput as json[0m[0m
    [0m[0m  [0m[0m[1m-b[0m[0m, [0m[0m[1m--block[0m[0m [0m[0m<BLOCK>[0m[0m    [0m[0mThe optional block where to fetch the runtime. That allows fetching older runtimes but you will need to connect to archive nodes. Currently, you must pass a block hash. Passing the block numbers is not supported[0m[0m
    [0m[0m  [0m[0m[1m-q[0m[0m, [0m[0m[1m--quiet[0m[0m            [0m[0mLess output[0m[0m
    [0m[0m  [0m[0m[1m-n[0m[0m, [0m[0m[1m--no-color[0m[0m         [0m[0m[env: NO_COLOR=][0m[0m
    [0m[0m  [0m[0m[1m-o[0m[0m, [0m[0m[1m--output[0m[0m [0m[0m<OUTPUT>[0m[0m  [0m[0mYou may specifiy the output filename where the runtime will be saved. If not provided, we will figure out an appropriate default name based on a counter: runtime_NNN.wasm where NNN is incrementing to make sure you do not override previous runtime. If you specify an existing file as output, it will be overwritten[0m[0m
    [0m[0m  [0m[0m[1m-h[0m[0m, [0m[0m[1m--help[0m[0m             [0m[0mPrint help information[0m[0m
    [0m[0m  [0m[0m[1m-V[0m[0m, [0m[0m[1m--version[0m[0m          [0m[0mPrint version information[0m[0m
    [0m

### Command: info

    [0mThe `info` command returns summarized information about a runtime[0m[0m
    [0m[0m
    [0m[0m[1m[4mUsage:[0m[0m [0m[0m[1msubwasm info[0m[0m [OPTIONS][0m[0m [0m[0m[SOURCE][0m[0m

    [0m[0m[1m[4mArguments:
    [0m[0m  [0m[0m[SOURCE][0m[0m  [0m[0mThe wasm file to load. It can be a path on your local filesystem such as /tmp/runtime.wasm or a node url such as http://localhost:9933 or ws://localhost:9944[0m[0m [0m[0m[default: runtime_000.wasm][0m[0m

    [0m[0m[1m[4mOptions:
    [0m[0m  [0m[0m    [0m[0m[1m--chain[0m[0m [0m[0m<CHAIN>[0m[0m  [0m[0mProvide the name of a chain and a random url amongst a list of known nodes will be used. If you pass a valid --chain, --url will be ignored --chain local = http://localhost:9933[0m[0m
    [0m[0m  [0m[0m[1m-j[0m[0m, [0m[0m[1m--json[0m[0m           [0m[0mOutput as json[0m[0m
    [0m[0m  [0m[0m[1m-b[0m[0m, [0m[0m[1m--block[0m[0m [0m[0m<BLOCK>[0m[0m  [0m[0mThe optional block where to fetch the runtime. That allows fetching older runtimes but you will need to connect to archive nodes. Currently, you must pass a block hash. Passing the block numbers is not supported[0m[0m
    [0m[0m  [0m[0m[1m-q[0m[0m, [0m[0m[1m--quiet[0m[0m          [0m[0mLess output[0m[0m
    [0m[0m  [0m[0m[1m-n[0m[0m, [0m[0m[1m--no-color[0m[0m       [0m[0m[env: NO_COLOR=][0m[0m
    [0m[0m  [0m[0m[1m-h[0m[0m, [0m[0m[1m--help[0m[0m           [0m[0mPrint help information[0m[0m
    [0m[0m  [0m[0m[1m-V[0m[0m, [0m[0m[1m--version[0m[0m        [0m[0mPrint version information[0m[0m
    [0m

By default, the ID for the Parachain pallet is expected to be `0x01` and the call ID for `authorize_upgrade` is expected to be `0x03`.
This default behavior can be overriden by setting the `PARACHAIN_PALLET_ID` to the ID of your parachain pallet and the `AUTHORIZE_UPGRADE_PREFIX` to the ID of your choice.

### Command: version

    [0mThe `version` command returns summarized information about the versions of a runtime[0m[0m
    [0m[0m
    [0m[0m[1m[4mUsage:[0m[0m [0m[0m[1msubwasm version[0m[0m [OPTIONS][0m[0m [0m[0m[SOURCE][0m[0m

    [0m[0m[1m[4mArguments:
    [0m[0m  [0m[0m[SOURCE][0m[0m  [0m[0mThe wasm file to load. It can be a path on your local filesystem such as /tmp/runtime.wasm or a node url such as http://localhost:9933 or ws://localhost:9944[0m[0m [0m[0m[default: runtime_000.wasm][0m[0m

    [0m[0m[1m[4mOptions:
    [0m[0m  [0m[0m    [0m[0m[1m--chain[0m[0m [0m[0m<CHAIN>[0m[0m  [0m[0mProvide the name of a chain and a random url amongst a list of known nodes will be used. If you pass a valid --chain, --url will be ignored --chain local = http://localhost:9933[0m[0m
    [0m[0m  [0m[0m[1m-j[0m[0m, [0m[0m[1m--json[0m[0m           [0m[0mOutput as json[0m[0m
    [0m[0m  [0m[0m[1m-b[0m[0m, [0m[0m[1m--block[0m[0m [0m[0m<BLOCK>[0m[0m  [0m[0mThe optional block where to fetch the runtime. That allows fetching older runtimes but you will need to connect to archive nodes. Currently, you must pass a block hash. Passing the block numbers is not supported[0m[0m
    [0m[0m  [0m[0m[1m-q[0m[0m, [0m[0m[1m--quiet[0m[0m          [0m[0mLess output[0m[0m
    [0m[0m  [0m[0m[1m-n[0m[0m, [0m[0m[1m--no-color[0m[0m       [0m[0m[env: NO_COLOR=][0m[0m
    [0m[0m  [0m[0m[1m-h[0m[0m, [0m[0m[1m--help[0m[0m           [0m[0mPrint help information[0m[0m
    [0m[0m  [0m[0m[1m-V[0m[0m, [0m[0m[1m--version[0m[0m        [0m[0mPrint version information[0m[0m
    [0m

### Command: meta

    [0mReturns the metadata as a json object. You may also use the "meta" alias. It is no longer possible to have a "printable" output, for that, please use the 'show-reduced' sub-command[0m[0m
    [0m[0m
    [0m[0m[1m[4mUsage:[0m[0m [0m[0m[1msubwasm metadata[0m[0m [OPTIONS][0m[0m [0m[0m[SOURCE][0m[0m

    [0m[0m[1m[4mArguments:
    [0m[0m  [0m[0m[SOURCE][0m[0m  [0m[0mThe wasm file to load. It can be a path on your local filesystem such as /tmp/runtime.wasm or a node url such as http://localhost:9933 or ws://localhost:9944[0m[0m [0m[0m[default: runtime_000.wasm][0m[0m

    [0m[0m[1m[4mOptions:
    [0m[0m  [0m[0m    [0m[0m[1m--chain[0m[0m [0m[0m<CHAIN>[0m[0m    [0m[0mProvide the name of a chain and a random url amongst a list of known nodes will be used. If you pass a valid --chain, --url will be ignored --chain local = http://localhost:9933[0m[0m
    [0m[0m  [0m[0m[1m-j[0m[0m, [0m[0m[1m--json[0m[0m             [0m[0mOutput as json[0m[0m
    [0m[0m  [0m[0m[1m-m[0m[0m, [0m[0m[1m--module[0m[0m [0m[0m<MODULE>[0m[0m  [0m[0mWithout this flag, the metadata command display the list of all modules. Using this flag, you will only see the module of your choice and a few details about it[0m[0m
    [0m[0m  [0m[0m[1m-q[0m[0m, [0m[0m[1m--quiet[0m[0m            [0m[0mLess output[0m[0m
    [0m[0m  [0m[0m[1m-b[0m[0m, [0m[0m[1m--block[0m[0m [0m[0m<BLOCK>[0m[0m    [0m[0mThe optional block where to fetch the runtime. That allows fetching older runtimes but you will need to connect to archive nodes. Currently, you must pass a block hash. Passing the block numbers is not supported[0m[0m
    [0m[0m  [0m[0m[1m-n[0m[0m, [0m[0m[1m--no-color[0m[0m         [0m[0m[env: NO_COLOR=][0m[0m
    [0m[0m  [0m[0m[1m-h[0m[0m, [0m[0m[1m--help[0m[0m             [0m[0mPrint help information[0m[0m
    [0m[0m  [0m[0m[1m-V[0m[0m, [0m[0m[1m--version[0m[0m          [0m[0mPrint version information[0m[0m
    [0m

### Command: show-reduced

    [0mShows the ReducedRuntime[0m[0m
    [0m[0m
    [0m[0m[1m[4mUsage:[0m[0m [0m[0m[1msubwasm show-reduced[0m[0m [OPTIONS][0m[0m [0m[0m[SRC][0m[0m

    [0m[0m[1m[4mArguments:
    [0m[0m  [0m[0m[SRC][0m[0m  [0m[0mThe first source[0m[0m [0m[0m[default: runtime_000.wasm][0m[0m

    [0m[0m[1m[4mOptions:
    [0m[0m  [0m[0m    [0m[0m[1m--chain[0m[0m [0m[0m<CHAIN>[0m[0m    [0m[0mProvide the name of a chain and a random url amongst a list of known nodes will be used. If you pass a valid --chain, --url will be ignored --chain local = http://localhost:9933[0m[0m
    [0m[0m  [0m[0m[1m-j[0m[0m, [0m[0m[1m--json[0m[0m             [0m[0mOutput as json[0m[0m
    [0m[0m  [0m[0m[1m-b[0m[0m, [0m[0m[1m--block[0m[0m [0m[0m<BLOCK>[0m[0m    [0m[0mThe optional block where to fetch the runtime. That allows fetching older runtimes but you will need to connect to archive nodes. Currently, you must pass a block hash. Passing the block numbers is not supported[0m[0m
    [0m[0m  [0m[0m[1m-q[0m[0m, [0m[0m[1m--quiet[0m[0m            [0m[0mLess output[0m[0m
    [0m[0m  [0m[0m[1m-n[0m[0m, [0m[0m[1m--no-color[0m[0m         [0m[0m[env: NO_COLOR=][0m[0m
    [0m[0m  [0m[0m[1m-p[0m[0m, [0m[0m[1m--pallet[0m[0m [0m[0m<PALLET>[0m[0m  [0m[0mYou probably want to use `Reduced`[0m[0m
    [0m[0m  [0m[0m[1m-h[0m[0m, [0m[0m[1m--help[0m[0m             [0m[0mPrint help information[0m[0m
    [0m[0m  [0m[0m[1m-V[0m[0m, [0m[0m[1m--version[0m[0m          [0m[0mPrint version information[0m[0m
    [0m

### Command: diff

    [0mCompare 2 runtimes after converting them to ReducedRuntime[0m[0m
    [0m[0m
    [0m[0m[1m[4mUsage:[0m[0m [0m[0m[1msubwasm diff[0m[0m [OPTIONS][0m[0m [0m[0m[SRC_A][0m[0m [0m[0m[SRC_B][0m[0m

    [0m[0m[1m[4mArguments:
    [0m[0m  [0m[0m[SRC_A][0m[0m  [0m[0mThe first source[0m[0m [0m[0m[default: runtime_000.wasm][0m[0m
    [0m[0m  [0m[0m[SRC_B][0m[0m  [0m[0mThe second source[0m[0m [0m[0m[default: runtime_001.wasm][0m[0m

    [0m[0m[1m[4mOptions:
    [0m[0m  [0m[0m[1m-a[0m[0m, [0m[0m[1m--chain-a[0m[0m [0m[0m<CHAIN_A>[0m[0m  [0m[0mProvide the name of a chain and a random url amongst a list of known nodes will be used. If you pass a valid --chain, --url will be ignored --chain local = http://localhost:9933[0m[0m
    [0m[0m  [0m[0m[1m-j[0m[0m, [0m[0m[1m--json[0m[0m               [0m[0mOutput as json[0m[0m
    [0m[0m  [0m[0m[1m-b[0m[0m, [0m[0m[1m--chain-b[0m[0m [0m[0m<CHAIN_B>[0m[0m  [0m[0mProvide the name of a chain and a random url amongst a list of known nodes will be used. If you pass a valid --chain, --url will be ignored --chain local = http://localhost:9933[0m[0m
    [0m[0m  [0m[0m[1m-q[0m[0m, [0m[0m[1m--quiet[0m[0m              [0m[0mLess output[0m[0m
    [0m[0m  [0m[0m[1m-m[0m[0m, [0m[0m[1m--method[0m[0m [0m[0m<METHOD>[0m[0m    [0m[0mYou probably want to use `Reduced`[0m[0m [0m[0m[default: reduced][0m[0m
    [0m[0m  [0m[0m[1m-n[0m[0m, [0m[0m[1m--no-color[0m[0m           [0m[0m[env: NO_COLOR=][0m[0m
    [0m[0m  [0m[0m[1m-h[0m[0m, [0m[0m[1m--help[0m[0m               [0m[0mPrint help information[0m[0m
    [0m[0m  [0m[0m[1m-V[0m[0m, [0m[0m[1m--version[0m[0m            [0m[0mPrint version information[0m[0m
    [0m

### Command: compress

    [0mCompress a given runtime wasm file. You will get an error if you try compressing a runtime that is already compressed[0m[0m
    [0m[0m
    [0m[0m[1m[4mUsage:[0m[0m [0m[0m[1msubwasm compress[0m[0m [OPTIONS][0m[0m [0m[0m<INPUT>[0m[0m [0m[0m<OUTPUT>[0m[0m

    [0m[0m[1m[4mArguments:
    [0m[0m  [0m[0m<INPUT>[0m[0m   [0m[0mThe path of uncompressed wasm file to load[0m[0m
    [0m[0m  [0m[0m<OUTPUT>[0m[0m  [0m[0mThe path of the file where the compressed runtime will be stored[0m[0m

    [0m[0m[1m[4mOptions:
    [0m[0m  [0m[0m[1m-j[0m[0m, [0m[0m[1m--json[0m[0m      [0m[0mOutput as json[0m[0m
    [0m[0m  [0m[0m[1m-q[0m[0m, [0m[0m[1m--quiet[0m[0m     [0m[0mLess output[0m[0m
    [0m[0m  [0m[0m[1m-n[0m[0m, [0m[0m[1m--no-color[0m[0m  [0m[0m[env: NO_COLOR=][0m[0m
    [0m[0m  [0m[0m[1m-h[0m[0m, [0m[0m[1m--help[0m[0m      [0m[0mPrint help information[0m[0m
    [0m[0m  [0m[0m[1m-V[0m[0m, [0m[0m[1m--version[0m[0m   [0m[0mPrint version information[0m[0m
    [0m

### Command: decompress

    [0mDecompress a given runtime wasm file. You may pass a runtime that is uncompressed already. In that case, you will get the same content as output. This is useful if you want to decompress "no matter what" and don't really know whether the input will be compressed or not[0m[0m
    [0m[0m
    [0m[0m[1m[4mUsage:[0m[0m [0m[0m[1msubwasm decompress[0m[0m [OPTIONS][0m[0m [0m[0m<INPUT>[0m[0m [0m[0m<OUTPUT>[0m[0m

    [0m[0m[1m[4mArguments:
    [0m[0m  [0m[0m<INPUT>[0m[0m   [0m[0mThe path of the compressed or uncompressed wasm file to load[0m[0m
    [0m[0m  [0m[0m<OUTPUT>[0m[0m  [0m[0mThe path of the file where the uncompressed runtime will be stored[0m[0m

    [0m[0m[1m[4mOptions:
    [0m[0m  [0m[0m[1m-j[0m[0m, [0m[0m[1m--json[0m[0m      [0m[0mOutput as json[0m[0m
    [0m[0m  [0m[0m[1m-q[0m[0m, [0m[0m[1m--quiet[0m[0m     [0m[0mLess output[0m[0m
    [0m[0m  [0m[0m[1m-n[0m[0m, [0m[0m[1m--no-color[0m[0m  [0m[0m[env: NO_COLOR=][0m[0m
    [0m[0m  [0m[0m[1m-h[0m[0m, [0m[0m[1m--help[0m[0m      [0m[0mPrint help information[0m[0m
    [0m[0m  [0m[0m[1m-V[0m[0m, [0m[0m[1m--version[0m[0m   [0m[0mPrint version information[0m[0m
    [0m

### Environment variables

In addition to the command line flags, you can also pass one of the following ENV variables:

    # This is a sample .env file. It is not needed if you
    # are using defaults if you want to use the default defined
    # below.

    # POLKADOT_HTTP=http://localhost:9933
    # POLKADOT_WS=ws://localhost:9944
    # PARACHAIN_PALLET_ID=0x01
    # AUTHORIZE_UPGRADE_PREFIX=0x03

## Sample runs

**Fetch a runtime from a running node**

We will start by fetching the runtime from a node.

Please note that you will likely need to connect to an **archive** node to retrieve an older runtime (`<V11`). A runtime takes around 2MB of storage on-chain and thus, older versions are pruned and will no longer be accessible if you are connecting to a non-archive node.

**Here we get the latest version of the runtime, the 3 commands do the same since they all use the default values:**

    subwasm get
    subwasm get --url http://localhost:9933
    subwasm get --url http://localhost:9933 --output runtime_000.wasm

**Here we get an older runtime, back when Polkadot was at block 20 !**

    subwasm get brew tap chevdor/subwasm --block 0x4d6a0bca208b85d41833a7f35cf73d1ae6974f4bad8ab576e2c3f751d691fe6c

By default, your runtime will be saved as `runtime_000.wasm`. Running this command again will increase the counter so we you don’t lose your previous runtime. You may also use the `--output` flag to provide the destination and filename of your choice. Beware, in this case, there will be no incremented counter.

**Get quick check of a runtime**

    # Show the runtime version and exit with status 0
    subwasm info --input kusama-2030.wasm

    # Provide a few explanations and exit with a status that is not 0
    subwasm info --input tictactoe.wasm

### Metadata JSON and jq tricks

`jq` can be used to reprocess the json output. For instance, removing all the documentation from the metadata makes it significantly smaller. The example below shows how to remove `documentation`, `value` and `default` making it much easier on the eyes for human parsing…​

        subwasm --json meta runtime.wasm | jq 'del( .. | .documentation?, .default?, .value? )'

## Alternatives

Here is a list of other projects allowing to get the raw metadata through a rpc call:

-   [PolkadotJS](https://github.com/polkadot-js/apps) from Jaco / Parity

-   [subsee](https://github.com/ascjones/subsee) from Andrew / Parity

-   [substrate-api-client](https://github.com/scs/substrate-api-client) from SCS

-   [subxt](https://github.com/paritytech/substrate-subxt) from Parity

All those alternatives require a running node and access it via jsonrpc.

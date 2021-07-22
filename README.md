# subwasm

![badge](https://github.com/chevdor/subwasm/actions/workflows/quick-check.yml/badge.svg?branch=master)

![Logo 1024](resources/logos/Logo_1024.png)

## Introduction

The metadata of a given runtime is a critical piece of information: it can be seen as the signature of a runtime.
It contains the exhaustive list of all the features publicly exposed by the runtime.

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

## Install

### Using Cargo

    cargo install --git https://github.com/chevdor/subwasm --tag v0.12.1

### Homebrew

MacOS Homebrew users can use:

    brew tap chevdor/subwasm https://github.com/chevdor/subwasm
    brew install subwasm

### Linux

    wget https://github.com/chevdor/subwasm/releases/download/v0.12.1/subwasm_linux_amd64_v0.12.1 -O subwasm.deb
    sudo dpkg -i subwasm.deb
    subwasm --help

## Usage

### Command: --help

    subwasm 0.13.0
    chevdor <chevdor@gmail.com>:Wilfried Kopp <wilfried@parity.io
    You can find all available commands below

    USAGE:
        subwasm [FLAGS] <SUBCOMMAND>

    FLAGS:
        -h, --help       Prints help information
        -j, --json       Output as json
        -q, --quiet      Less output
        -V, --version    Prints version information

    SUBCOMMANDS:
        diff        Compare 2 runtimes
        get         Get/Download the runtime wasm from a running node through rpc
        help        Prints this message or the help of the given subcommand(s)
        info        The `info` command returns summarized information about a runtime
        metadata    Returns the metadata as a json object. You may also use the "meta" alias

### Command: get

    subwasm-get 0.13.0
    chevdor <chevdor@gmail.com>:Wilfried Kopp <wilfried@parity.io
    Get/Download the runtime wasm from a running node through rpc

    USAGE:
        subwasm get [OPTIONS] [url]

    ARGS:
        <url>    The node url. Example: ws://localhost:9944 or http://localhost:9933 [default:
                 http://localhost:9933]

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    OPTIONS:
        -b, --block <block>      The optional block where to fetch the runtime. That allows fetching
                                 older runtimes but you will need to connect to archive nodes.
                                 Currently, you must pass a block hash. Passing the block numbers is not
                                 supported
            --chain <chain>      Provide the name of a chain and a random url amongst a list of known
                                 nodes will be used. If you pass a valid --chain, --url will be ignored
                                 --chain local = http://localhost:9933
        -o, --output <output>    You may specifiy the output filename where the runtime will be saved.
                                 If not provided, we will figure out an appropriate default name based
                                 on a counter: runtime_NNN.wasm where NNN is incrementing to make sure
                                 you do not override previous runtime. If you specify an existing file
                                 as output, it will be overwritten

### Command: info

    subwasm-info 0.13.0
    chevdor <chevdor@gmail.com>:Wilfried Kopp <wilfried@parity.io
    The `info` command returns summarized information about a runtime

    USAGE:
        subwasm info [OPTIONS] [source]

    ARGS:
        <source>    The wasm file to load. It can be a path on your local filesystem such as
                    /tmp/runtime.wasm or a node url such as http://localhost:9933 or
                    ws://localhost:9944 [default: runtime_000.wasm]

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    OPTIONS:
        -b, --block <block>
            --chain <chain>    Provide the name of a chain and a random url amongst a list of known
                               nodes will be used. If you pass a valid --chain, --url will be ignored
                               --chain local = http://localhost:9933

### Command: meta

    subwasm-metadata 0.13.0
    chevdor <chevdor@gmail.com>:Wilfried Kopp <wilfried@parity.io
    Returns the metadata as a json object. You may also use the "meta" alias

    USAGE:
        subwasm metadata [FLAGS] [OPTIONS] [source]

    ARGS:
        <source>    The wasm file to load. It can be a path on your local filesystem such as
                    /tmp/runtime.wasm or a node url such as http://localhost:9933 or
                    ws://localhost:9944 [default: runtime_000.wasm]

    FLAGS:
        -h, --help       Prints help information
        -j, --json       Output as json
        -V, --version    Prints version information

    OPTIONS:
        -b, --block <block>
            --chain <chain>      Provide the name of a chain and a random url amongst a list of known
                                 nodes will be used. If you pass a valid --chain, --url will be ignored
                                 --chain local = http://localhost:9933
        -m, --module <module>    Without this flag, the metadata command display the list of all
                                 modules. Using this flag, you will only see the module of your choice
                                 and a few details about it

### Command: diff

    subwasm-diff 0.13.0
    chevdor <chevdor@gmail.com>:Wilfried Kopp <wilfried@parity.io
    Compare 2 runtimes

    USAGE:
        subwasm diff [OPTIONS] [ARGS]

    ARGS:
        <src-a>    The first source [default: runtime_000.wasm]
        <src-b>    The second source [default: runtime_001.wasm]

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    OPTIONS:
        -a, --chain-a <chain-a>    Provide the name of a chain and a random url amongst a list of known
                                   nodes will be used. If you pass a valid --chain, --url will be
                                   ignored --chain local = http://localhost:9933
        -b, --chain-b <chain-b>    Provide the name of a chain and a random url amongst a list of known
                                   nodes will be used. If you pass a valid --chain, --url will be
                                   ignored --chain local = http://localhost:9933

## Sample runs

**Fetch a runtime from a running node**

We will start by fetching the runtime from a node.

Please note that you will likely need to connect to an **archive** node to retrieve older runtime. A runtime takes around 2MB of storage on-chain and thus, older versions are pruned and will no longer be accessible if you are connectin to a non-archive node.

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

`jq` can be used to reprocess the json output. For instance, removing all the documentation from the metadata makes it significantly smaller. The example below shows how to remove `documentation`, `value` and `default` for a version easier on the eyes for human parsing…​

        subwasm --json meta runtime.wasm | jq 'del( .. | .documentation?, .default?, .value? )'

## Alternatives

Here is a list of other projects allowing to get the raw metadata through a rpc call:

-   [PolkadotJS](https://github.com/polkadot-js/apps) from Jaco / Parity

-   [subsee](https://github.com/ascjones/subsee) from Andrew / Parity

-   [substrate-api-client](https://github.com/scs/substrate-api-client) from SCS

-   [subxt](https://github.com/paritytech/substrate-subxt) from Parity

All those alternatives require a running node and access it via jsonrpc.

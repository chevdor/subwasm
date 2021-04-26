# subwasm

`subwasm` is a utility that works on WASM Runtime of Substrate based chains. 

It can:
- fetch the latest wasm runtime from a running node
- fetch any older wasm runtime from a running archive node
- fetch the metadata from a running node
- fetch the medadata from a local wasm file without having to run a node

## Source 
You can find the repository for this project on [gitlab](https://www.gitlab.com/chevdor/subwasm).

## Install

You may also install it directly using:
````
cargo install --git https://www.gitlab.com/chevdor/subwasm -p subwasm
````

Use the built-in help system to know more about the commands and subcommands as well as their options.

## Project crates

The following crates are direct dependencies coming with this project.

- [`subwasmlib`]
- [`wasm_loader`]
- [`wasm_testbed`]
- [`rpc_client`]
- [`substrate_runtime_proposal_hash`]

VERSION := `toml get cli/Cargo.toml package.version | jq -r`
TARGET_DIR := "target/release"
run:
	cargo run

test:
	cargo watch -x "test -- --no-capture"

test_all:
	cargo test -- --include-ignored

get:
	cargo run -- get --url http://localhost:9933

usage:
	cargo run -q -- --help > doc/usage.adoc
	cargo run -q -- get --help > doc/usage_get.adoc
	cargo run -q -- info --help > doc/usage_info.adoc
	cargo run -q -- meta --help > doc/usage_meta.adoc

doc:usage
	cargo doc -p subwasm -p subwasmlib -p wasm-loader -p wasm-testbed -p substrate-runtime-proposal-hash --all-features --no-deps

bump:
	cargo workspaces version minor --no-individual-tags

fetch-kusama:
	echo 'Fetching latest runtime from Kusama'
	cargo run -- get --url wss://kusama-rpc.polkadot.io -o kusama.wasm

fetch-polkadot:
	echo 'Fetching latest runtime from Polkadot'
	cargo run -- get --url wss://rpc.polkadot.io -o polkadot.wasm

fetch-westend:
	echo 'Fetching latest runtime from Westend'
	cargo run -- get --url wss://westend-rpc.polkadot.io -o westend.wasm

mac:
	@echo Preparing artifacts for MacOS for v{{VERSION}}
	cargo build --release
	tar -czf {{TARGET_DIR}}/subwasm-mac-v{{VERSION}}.tar.gz {{TARGET_DIR}}/subwasm
	shasum -a 256 {{TARGET_DIR}}/subwasm-mac-v{{VERSION}}.tar.gz > {{TARGET_DIR}}/subwasm-mac-v{{VERSION}}.tar.gz.sha256
	ls -al {{TARGET_DIR}}/*{{VERSION}}*
	cat {{TARGET_DIR}}/*{{VERSION}}*.sha256

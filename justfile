VERSION := `toml get cli/Cargo.toml package.version | jq -r`
TARGET_DIR := "target/release"

# List available commands
default:
  @just --list --unsorted

# Test / watch
test:
	cargo watch -x "test -- --no-capture"

# Test including ignored tests
test_all:
	cargo test -- --include-ignored

# Generate usage samples
_usage:
	cargo run -q -- --help > doc/usage.adoc
	cargo run -q -- get --help > doc/usage_get.adoc
	cargo run -q -- info --help > doc/usage_info.adoc
	cargo run -q -- meta --help > doc/usage_meta.adoc
	cargo run -q -- diff --help > doc/usage_diff.adoc

# Generate documentation
doc:_usage
	cargo doc -p subwasm -p subwasmlib -p wasm-loader -p wasm-testbed -p substrate-runtime-proposal-hash --all-features --no-deps

# Generate demos
demos:
	#!/usr/bin/env bash
	cd scripts/demos
	pwd
	ls -al
	./run-all.sh

# Run rustfmt
_fmt:
	cargo fmt --all

# Run clippy
_clippy:
	cargo clippy

# Run checks such as clippy, rustfmt, etc...
check: _clippy _fmt

# Minor bump, can be used once the release is ready
bump:
	cargo workspaces version minor --no-individual-tags --no-git-push

# Prepare a MacOS Binary
mac:
	@echo Preparing artifacts for MacOS for v{{VERSION}}
	cargo build --release
	tar -czf {{TARGET_DIR}}/subwasm-mac-v{{VERSION}}.tar.gz -C {{TARGET_DIR}} subwasm
	shasum -a 256 {{TARGET_DIR}}/subwasm-mac-v{{VERSION}}.tar.gz > {{TARGET_DIR}}/subwasm-mac-v{{VERSION}}.tar.gz.sha256
	ls -al {{TARGET_DIR}}/*{{VERSION}}*
	cat {{TARGET_DIR}}/*{{VERSION}}*.sha256

clean:
	rm -f cli/*.wasm
	rm -f *.wasm

changelog:
	#!/usr/bin/env bash
	latest=$(git rev-list -n 1 latest)
	cog changelog -f $latest

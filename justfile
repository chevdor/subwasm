VERSION := `toml get cli/Cargo.toml package.version | jq -r`
TARGET_DIR := "target/release"

# List available commands
_default:
  just --choose --chooser "fzf +s -x --tac --cycle"

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
doc: _usage
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
	cargo workspaces version --no-git-commit

clean:
	rm -f cli/*.wasm
	rm -f *.wasm

changelog:
	#!/usr/bin/env bash
	latest=$(git rev-list -n 1 latest)
	cog changelog -f $latest

# Generate the readme as .md
md:
    #!/usr/bin/env bash
    asciidoctor -b docbook -a leveloffset=+1 -o - README_src.adoc | pandoc   --markdown-headings=atx --wrap=preserve -t markdown_strict -f docbook - > README.md

release: check test_all bump doc md demos

coverage:
	#!/usr/bin/env bash
	export RUSTFLAGS="-Zinstrument-coverage"
	export LLVM_PROFILE_FILE="chevdor-%p-%m.profraw"
	cargo +nightly build
	cargo +nightly test
	grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/
	open target/debug/coverage/index.html
	find . -type f -name '*.profraw' -exec rm '{}' \;

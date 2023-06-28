VERSION := `toml get Cargo.toml workspace.package.version | jq -r`
export TAG:=`toml get Cargo.toml "workspace.package.version" | jq -r .`

# List available commands
_default:
  just --choose --chooser "fzf +s -x --tac --cycle"

help:
	just --list

test:
	cargo nextest run --no-fail-fast

# Test & watch
test_dev:
	cargo watch -x "test -- --no-capture"

# Test including ignored tests
test_all:
	cargo test -- --include-ignored

# Generate usage samples
usage:
	cargo run -q -- --help > doc/usage.adoc
	cargo run -q -- get --help > doc/usage_get.adoc
	cargo run -q -- info --help > doc/usage_info.adoc
	cargo run -q -- version --help > doc/usage_version.adoc
	cargo run -q -- meta --help > doc/usage_meta.adoc
	cargo run -q -- diff --help > doc/usage_diff.adoc
	cargo run -q -- compress --help > doc/usage_compress.adoc
	cargo run -q -- decompress --help > doc/usage_decompress.adoc
	cargo run -q -- show --help > doc/usage_show.adoc

# Generate documentation
doc:
	./scripts/build-doc.sh

watch-doc:
	cargo watch -s './scripts/build-doc.sh && browser-sync start --ss target/doc -s target/doc --directory --no-open'

# Generate demos
demos:
	#!/usr/bin/env bash
	cd scripts/demos
	pwd
	ls -al
	./run-all.sh

# Run rustfmt
fmt:
	cargo +nightly fmt --all

# Run clippy
clippy:
	cargo +nightly clippy --features="v14" --all-targets --tests

deny:
	cargo deny check

# Run checks such as clippy, rustfmt, etc...
check: clippy fmt

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

release: check test_all bump demos doc md

coverage_html:
	#!/usr/bin/env bash
	export RUSTFLAGS="-Zinstrument-coverage"
	export LLVM_PROFILE_FILE="chevdor-%p-%m.profraw"
	cargo +nightly test
	grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/
	open target/debug/coverage/index.html

coverage_lcov:
	#!/usr/bin/env bash
	export RUSTFLAGS="-Zinstrument-coverage"
	export LLVM_PROFILE_FILE="chevdor-%p-%m.profraw"
	cargo +nightly test
	grcov . -s . --binary-path ./target/debug/deps -t lcov --branch --ignore-not-existing --ignore '../*' -o ./target/debug/tests.lcov
	# grcov . --binary-path ./target/debug/deps/ -s . -t lcov --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o coverage/tests.lcov

clean_coverage:
	find . -type f -name '*.profraw' -exec rm '{}' \;

tag:
    #!/bin/sh
    echo "Tagging version v$TAG"
    git tag "v$TAG" -f
    git tag | sort -Vr | head

# Fetch the current runtimes
get_runtimes:
	#!/bin/sh
	for chain in kusama westend polkadot statemint statemine westmint; do
		echo "Fetching current $chain runtime"
		JSON=$(subwasm info --chain $chain -j)
		SPEC_VERSION=$(echo $JSON | jq -r .core_version.specVersion)
		METADATA_VERSION=$(echo $JSON | jq -r .metadata_version)
		echo "  Spec Version: $SPEC_VERSION"
		echo "  Metadata Version: V$METADATA_VERSION"
		mkdir -p "data/$chain/V$METADATA_VERSION"
		subwasm get --chain $chain -o data/$chain/V$METADATA_VERSION/$SPEC_VERSION.wasm
	done
	find data -newermt "-15 minutes" -iname "*.wasm" -ls

tag-push:
	#!/bin/sh
	echo "Pushing version v$TAG"
	git push origin "v$TAG"

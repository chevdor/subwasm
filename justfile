run:
	cargo +nightly run

test:
	cargo +nightly watch -x "test -- --no-capture"

get:
	cargo +nightly run -- get --url http://localhost:9933

fetch-kusama:
	echo 'Fetching runtimes from Kusama'

usage:
	cargo +nightly run -q -- --help > doc/usage.adoc
	cargo +nightly run -q -- get --help > doc/usage_get.adoc
	cargo +nightly run -q -- info --help > doc/usage_info.adoc
	cargo +nightly run -q -- meta --help > doc/usage_meta.adoc

doc:usage
	cargo +nightly doc -p subwasm -p subwasmlib -p wasm-loader -p wasm-testbed -p rpc-client -p substrate-runtime-proposal-hash --all-features --no-deps
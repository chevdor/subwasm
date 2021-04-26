run:
	cargo run

test:
	cargo watch -x "test -- --no-capture"

get:
	cargo run -- get --url http://localhost:9933

fetch-kusama:
	echo 'Fetching runtimes from Kusama'

doc:
	cargo doc --workspace --all-features

usage:
	cargo run -q -- --help > doc/usage.adoc
	cargo run -q -- get --help > doc/usage_get.adoc
	cargo run -q -- info --help > doc/usage_info.adoc
	cargo run -q -- meta --help > doc/usage_meta.adoc

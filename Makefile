.PHONY: all test integration-test static lint fmt

SHELL = /usr/bin/env sh -eu

all: test

test: static integration-test

integration-test:
	RUST_BACKTRACE=1 RUST_LOG=info cargo test -p holochain-conductor-runtime -- --nocapture
	RUST_BACKTRACE=1 RUST_LOG=info cargo test -p holochain-conductor-runtime-ffi -- --nocapture

static: fmt lint
	@if [ "${CI}x" != "x" ]; then git diff --exit-code; fi

lint:
	cargo clippy -p holochain-conductor-runtime -- -Dwarnings
	cargo clippy -p holochain-conductor-runtime-ffi -- -Dwarnings

fmt:
	cargo fmt -p holochain-conductor-runtime -- --check
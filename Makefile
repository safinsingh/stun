.DEFAULT_GOAL := release

dev:
	cargo build

release:
	cargo build --release -vv

test:
	cargo test

fmt:
	cargo fmt

simple:
	cargo clippy --fix -Z unstable-options

run:
	cargo run

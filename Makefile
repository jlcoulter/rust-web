# Rust Web — Makefile

BINARY := target/release/rust-web-template

.PHONY: run build test lint clippy fmt docker clean

run:
	cargo run

build:
	cargo build --release

test:
	cargo test --all

clippy:
	cargo clippy --all-targets -- -D warnings

fmt:
	cargo fmt --all -- --check

lint: fmt clippy

docker:
	docker build -t rust-web-template .

clean:
	cargo clean
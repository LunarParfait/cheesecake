build:
	cargo build --release

install:
	cargo install --path .

test:
	cargo test

lint:
	cargo clippy

format:
	cargo fmt

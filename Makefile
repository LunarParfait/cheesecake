setup:
	cp -n .env.example .env.local
	# cd build_utils && bun install

clean:
	rm -r dist logs
	cargo clean

build:
	# rm -r dist
	# mkdir dist
	# cd build_utils && zsh build-assets.zsh
	# cd build_utils && bun make-tailwind
	node build-assets.js
	cargo build -p app --release

test:
	cargo test

check:
	cargo check

check-prod:
	cargo check --release

lint:
	cargo clippy

lint-prod:
	cargo clippy --release

dev:
	RUST_SPANTRACE=1 RUST_BACKTRACE=full RUST_LIB_BACKTRACE=1 cargo run -p app

prod: build
	cargo run -p app --release

migrate:
	cargo run -p app -- --migrate-only


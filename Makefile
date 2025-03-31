setup:
	cp -n .env.example .env.local
	mkdir -p database/db
	touch database/db/db.sqlite
	pnpm i

clean:
	rm -r database/db || true
	rm -r dist logs || true
	cargo clean

build:
	rm -r dist || true
	mkdir dist dist/public dist/templates
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
	RUST_BACKTRACE=1 cargo run -p app

prod: build
	cargo run -p app --release

migrate:
	cargo run -p app -- --migrate-only

generate-entities:
	sea-orm-cli generate entity --with-serde -o database/entities

.PHONY: setup clean build test check check-prod lint lint-prod dev prod migrate generate-entities

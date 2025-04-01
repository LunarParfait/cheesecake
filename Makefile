setup:
	cp -n .env.example .env.local
	mkdir -p storage
	mkdir -p storage/db
	touch storage/db/db.sqlite
	pnpm i

clean:
	rm -r dist logs || true
	cargo clean

build:
	rm -r dist || true
	mkdir dist dist/static dist/templates
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
	LOG_LEVEL=DEBUG RUST_BACKTRACE=1 cargo run -p app

prod: build
	RUST_BACKTRACE=1 cargo run -p app --release

migrate-all:
	cargo run -p migration

migrate-reset:
	cargo run -p migration -- reset

migrate-status:
	cargo run -p migration -- status

migrate-up:
	cargo run -p migration -- up

migrate-down:
	cargo run -p migration -- down

generate-entities:
	sea-orm-cli generate entity --with-serde both -o db/entities/src

.PHONY: setup clean build test check check-prod lint lint-prod dev prod migrate-all migrate-reset migrate-status migrate-up migrate-down generate-entities

.PHONY: check clippy test fmt lint build clean

check:
	cargo check --all-targets --exclude harness_cache 2>/dev/null || cargo check --bins --lib

clippy:
	cargo clippy --lib --bins -- --allow clippy::new_without_default --allow clippy::len_without_is_empty --allow clippy::module_name_repetitions

test:
	cargo test --all

fmt:
	cargo fmt --all

lint: fmt check clippy

build:
	cargo build --all

build-release:
	cargo build --release --all

clean:
	cargo clean

watch:
	cargo watch -x check -x test

bench:
	cargo bench

build:
	cargo build

test:
	cargo test -j 1 --no-fail-fast --all

style-check:
	cargo fmt -- --check

lint:
	cargo clippy --all-features --all --tests --examples

coverage:
	cargo tarpaulin -o Html --output-dir target/tarpaulin/ --all

bench:
	rustup run nightly cargo bench --all

.PHONY: build test style-check lint coverage bench

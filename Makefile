CARGO_BUILD_JOBS := $(nproc)
build:
	cargo build

test:
	cargo test --no-fail-fast --all -- --test-threads=1

style-check:
	cargo fmt -- --check

lint:
	cargo clippy --all-features --all --tests --examples

coverage:
	cargo tarpaulin -o Html --output-dir target/tarpaulin/ --all

bench:
	rustup run nightly cargo bench --all

clean:
	cargo clean

install:
	cargo install --git https://github.com/masato-hi/claris.git

.PHONY: build test style-check lint coverage bench install

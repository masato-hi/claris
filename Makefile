CARGO_BUILD_JOBS := $(nproc)
run:
	cargo run ${ARGS}

build:
	cargo build

test:
	cargo test --no-fail-fast --quiet --all -- --test-threads=1

style-check:
	cargo fmt -- --check

lint:
	cargo clippy --all-features --all --tests --examples

coverage:
	cargo tarpaulin -o Html --output-dir target/tarpaulin/ --all

bench:
	rustup run nightly cargo bench --no-fail-fast --quiet --all

clean:
	cargo clean

install:
	cargo install --git https://github.com/masato-hi/claris.git

.PHONY: run build test style-check lint coverage bench install

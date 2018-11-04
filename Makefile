.PHONY: all build watch clean run lint test check

default: build

all: clean build fmt lint test

build: vet
	cargo build	

clean:
	cargo clean

run:
	cargo run

watch:
	cargo watch -x run	

lint:
	cargo clippy --all-targets --all-features -- -D warnings

test:
	cargo test

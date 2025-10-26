#!/usr/bin/env just --justfile

fmt:
    cargo +nightly fmt --all -- --config format_code_in_doc_comments=true

check:
    cargo check
    cargo check --all-features
    cargo check --no-default-features --features async --target wasm32-unknown-unknown

clippy:
    cargo clippy -- -D warnings
    cargo clippy --all-features -- -D warnings
    cargo clippy --no-default-features --features async --target wasm32-unknown-unknown -- -D warnings

test:
    cargo test
    cargo test --all-features

precommit: fmt check clippy test

#!/usr/bin/env just --justfile

fmt:
    cargo +nightly fmt --all -- --config format_code_in_doc_comments=true

check:
    cargo check --no-default-features --features async
    cargo check --no-default-features --features blocking
    cargo check --no-default-features --features async --target wasm32-unknown-unknown

clippy:
    cargo clippy --no-default-features --features async -- -D warnings
    cargo clippy --no-default-features --features blocking -- -D warnings
    cargo clippy --no-default-features --features async --target wasm32-unknown-unknown -- -D warnings

test:
    cargo test --no-default-features --features async
    cargo test --no-default-features --features blocking

precommit: fmt check clippy test

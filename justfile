#!/usr/bin/env just --justfile

check:
    cargo fmt --all -- --config format_code_in_doc_comments=true
    cargo check --no-default-features --features async
    cargo check --no-default-features --features blocking
    cargo check --no-default-features --features async --target wasm32-unknown-unknown
    cargo test --no-default-features --features async
    cargo test --no-default-features --features blocking
    cargo clippy --no-default-features --features async -- -D warnings
    cargo clippy --no-default-features --features blocking -- -D warnings
    cargo clippy --no-default-features --features async --target wasm32-unknown-unknown -- -D warnings

#!/usr/bin/env just --justfile

precommit:
    cargo fmt --all -- --config format_code_in_doc_comments=true
    cargo test
    cargo clippy
    cargo clippy --features blocking

check:
    cargo fmt --all -- --config format_code_in_doc_comments=true
    cargo check
    cargo test
    cargo clippy -- -D warnings
    cargo clippy --features blocking -- -D warnings

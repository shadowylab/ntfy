#!/usr/bin/env just --justfile

precommit:
    cargo fmt
    cargo clippy
    cargo clippy --features blocking

check:
    cargo fmt --all -- --config format_code_in_doc_comments=true
    cargo check
    cargo clippy -- -D warnings
    cargo clippy --features blocking -- -D warnings
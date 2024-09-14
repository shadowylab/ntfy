#!/usr/bin/env just --justfile

check:
    cargo fmt --all -- --config format_code_in_doc_comments=true
    cargo check
    cargo test
    cargo clippy -- -D warnings

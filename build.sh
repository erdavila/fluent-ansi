#!/bin/bash
set -e

cargo test --no-fail-fast
cargo fmt
cargo clippy --all-targets
cargo doc --no-deps

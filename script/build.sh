#!/usr/bin/env bash
cargo build --release --target aarch64-apple-darwin
mv "target/aarch64-apple-darwin/release/gh-diary" "./dist/darwin-amd64"

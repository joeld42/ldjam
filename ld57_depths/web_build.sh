#!/usr/bin/env bash
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --target web \
    --out-dir ./out/ \
    --out-name "ld57_depths" \
    ./target/wasm32-unknown-unknown/release/ld57_depths.wasm
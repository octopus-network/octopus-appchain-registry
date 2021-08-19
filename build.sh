#!/bin/bash
cargo fmt --all
RUSTFLAGS='-C link-arg=-s' cargo build --all --target wasm32-unknown-unknown --release
if [ ! -d "out" ]; then
    mkdir -p "out"
fi
if [ ! -d "res" ]; then
    mkdir -p "res"
fi
cp target/wasm32-unknown-unknown/release/*.wasm ./res/
cp target/wasm32-unknown-unknown/release/appchain_registry.wasm ./out/main.wasm

if [ "$1" == "test" ]; then
    RUST_BACKTRACE=1 cargo test --test test_registry_actions -- --nocapture
fi

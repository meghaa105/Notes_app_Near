#!/bin/bash
# command ./build.sh
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/test101.wasm res
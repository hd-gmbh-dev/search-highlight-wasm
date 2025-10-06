#!/bin/bash
set -e
# export RUSTFLAGS='--cfg getrandom_backend="wasm_js"'
wasm-pack build --release --target web --reference-types --no-default-features
cp ./package.tmp.web.json pkg/package.json
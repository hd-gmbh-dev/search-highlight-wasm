#!/bin/bash
set -e
# export RUSTFLAGS='--cfg getrandom_backend="wasm_js"'
wasm-pack build --release --target nodejs --reference-types --no-default-features

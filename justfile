#!/usr/bin/env just --justfile
set shell := ["powershell.exe", "-c"]

release:
    cargo build --release

web-build:
    cargo build --target wasm32-unknown-unknown --profile web

web-pack: web-build
    wasm-bindgen --no-typescript --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/web/vector_survivor.wasm
    wasm-opt ./out/vector_survivor_bg.wasm -O4 -o ./out/vector_survivor_bg.wasm

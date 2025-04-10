clippy:
    cargo clippy --all-targets --all-features --locked -- -D warnings

build-wasm:
    wasm-pack build ./crates/wasm-app

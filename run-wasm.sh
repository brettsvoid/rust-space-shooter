set -e
cargo build --target wasm32-unknown-unknown --release --no-default-features
wasm-server-runner target/wasm32-unknown-unknown/release/rust-space-shooter.wasm

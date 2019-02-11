#!/bin/bash

ROOT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )"/.. >/dev/null 2>&1 && pwd .. )"

if ! command -v rustc >/dev/null 2>&1; then
  curl https://sh.rustup.rs -sSf | sh -s -- -y
  PATH=$PATH:$HOME/.cargo/bin
fi

if ! command -v cargo-clippy >/dev/null 2>&1; then
  rustup component add clippy
fi

if ! command -v rusty-hook >/dev/null 2>&1; then
  cargo install --path "$ROOT_DIR"
fi

echo "Environment successfully configured!"
cd "$ROOT_DIR" || exit 1
rusty-hook init
cargo test --bin rusty-hook

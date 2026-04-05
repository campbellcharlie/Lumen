#!/bin/sh
set -e
cargo build --release
mkdir -p "$HOME/.bin"
cp target/release/lumen "$HOME/.bin/lumen"
echo "Installed lumen to ~/.bin/lumen"

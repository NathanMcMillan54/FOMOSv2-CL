#!/bin/bash

echo "Updating Rust..."
rustup update
echo "Installing Nightly Toolchain..."
rustup default nightly
echo "Installing cargo-xbuild..."
cargo install cargo-xbuild
echo "Adding component rust-src"
rustup component add rust-src
echo "Installing Qemu ARM"
sudo apt install qemu-system-arm

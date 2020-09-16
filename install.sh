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

sleep 5

echo "Building FOMOS in..."
echo "3"
sleep 1
echo "2"
sleep 1
echo "1"
sleep 1
echo "Building..."
sh build.sh

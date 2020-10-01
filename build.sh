#!/bin/bash

echo "Compiling FOMOS..."
rm -rf target/
cargo xbuild --target=aarch64-unknown-none.json

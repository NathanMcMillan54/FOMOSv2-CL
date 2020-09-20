#!/bin/bash

rm -rf target/
sleep 1
cd boot/
make
cd ../
cargo xbuild --target=aarch64-unknown-none.json

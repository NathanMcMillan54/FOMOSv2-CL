#!/bin/bash

rm -rf target/
sleep 1
cargo xbuild --target=aarch64-unknown-none.json

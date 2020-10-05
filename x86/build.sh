#!/bin/bash

rm -rf target/
cargo xbuild --target=x86-unknown-none.json

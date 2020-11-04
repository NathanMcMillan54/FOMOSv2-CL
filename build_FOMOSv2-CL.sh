#!/bin/bash

cd FOMOSv2/
echo "Compiling FOMOSv2-CL for arch: arm..."
sleep 1
cargo build --target=armv7-unknown-linux-gnueabihf --release
echo "Done compiling FOMOSv2-CL."

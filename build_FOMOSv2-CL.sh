#!/bin/bash

echo "Compiling FOMOSv2-CL for arch: arm..."
sleep 1
cargo build arm7-unknown-linnux-musleabihf --release
echo "Done compiling FOMOSv2-CL."

#!/bin/bash

echo "Compiling FOMOSv2-CL..."
sleep 1
cargo build --release
nm -D ./target/release/libFOMOS.so | grep init
nm -D ./target/release/libFOMOS.so | grep shutdown_fomos
gcc -o FOMOS FOMOS.c -Isrc  -L. -l:target/release/libFOMOS.so
echo "Done compiling FOMOSv2-CL."

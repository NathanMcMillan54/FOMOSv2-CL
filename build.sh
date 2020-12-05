#!/bin/bash

ARCH=$1

echo "Compiling FOMOSv2-CL v2.3.5 for $ARCH..."
if [ "$ARCH" = 'armv6' ]; then
    cargo build --target=armv6-unknown-linux-gnueabihf
    gcc -static linux/main.c target/armv6-unknown-linux-gnueabihf/debug/libFOMOSv2_CL.a -o init
elif [ "$ARCH" = 'armv7' ]; then
    cargo build --target=armv7-unknown-linux-gnueabihf
    gcc -static linux/main.c target/armv7-unknown-linux-gnueabihf/debug/libFOMOSv2_CL.a -o init
elif [ "$ARCH" = 'x86' ]; then
    cargo build --target=x86_64-unknown-linux-gnu
    gcc -static linux/main.c target/x86_64-unknown-linux-gnu/debug/libFOMOSv2_CL.a -o init
else
    echo "$ARCH is invalid"
fi

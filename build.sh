#!/bin/bash

ARCH=$1

echo "Compiling FOMOSv2-CL v2.3.5 for $ARCH..."
if [ "$ARCH" = 'armv6' ]; then
    cargo build --target=armv6-unknown-linux-gnueabihf
elif [ "$ARCH" = 'armv7' ]; then
    cargo build --target=armv7-unknown-linux-gnueabihf
elif [ "$ARCH" = 'x86' ]; then
    cargo build --target=x86_64-unknown-linux-gnu
    gcc -static src/main.c target/x86_64-unknown-linux-gnu/debug/libFOMOSv2_CL.a -o init
    mv init initramfs/ && cd initramfs/
    find . | cpio -o -H newc | gzip > ../../FOMOSv2-CL_v2_3_5.cpio.gz
else
    echo "$ARCH is invalid"
fi

#!/bin/bash
# build.sh
#
# Description:
# Compiles and links FOMOSv2-CL
#

ARCH=$1

echo "Compiling FOMOSv2-CL v2.3.5 for $ARCH..."
if [ "$ARCH" = 'armv6' ]; then
    cargo build --target=arm-unknown-linux-gnueabihf
    arm-linux-gnueabihf-gcc -static src/main.c target/arm-unknown-linux-gnueabihf/debug/libFOMOSv2_CL.a -o init
    mv init initramfs/ && cd initramfs/
    find . | cpio -o -H newc | gzip > ../FOMOSv2-CL_v2_3_5.cpio.gz
elif [ "$ARCH" = 'armv7' ]; then
    cargo build --target=armv7-unknown-linux-gnueabihf
    arm-linux-gnueabihf-gcc -static src/main.c target/armv7-unknown-linux-gnueabihf/debug/libFOMOSv2_CL.a -o init
    mv init initramfs/ && cd initramfs/
    find . | cpio -o -H newc | gzip > ../FOMOSv2-CL_v2_3_5.cpio.gz
elif [ "$ARCH" = 'x86' ]; then
    cargo build --target=x86_64-unknown-linux-gnu
    x86_64-linux-gnu-gcc -static src/main.c target/x86_64-unknown-linux-gnu/debug/libFOMOSv2_CL.a -o init
    mv init initramfs/ && cd initramfs/
    find . | cpio -o -H newc | gzip > ../FOMOSv2-CL_v2_3_5.cpio.gz
else
    echo "$ARCH is invalid"
fi

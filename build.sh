#!/bin/bash
# build.sh
#
# Description:
# Compiles and links FOMOSv2-CL
#

ARCH=$1

echo "Compiling FOMOSv2-CL v2.3.5 for $ARCH..."
cargo clean
rm -rf builtin_commands/
rm -rf initramfs/
if [ "$ARCH" = 'armv6' ]; then
    cargo build --target=arm-unknown-linux-gnueabihf
    arm-linux-gnueabihf-gcc -static src/main.c target/arm-unknown-linux-gnueabihf/debug/libFOMOSv2_CL.a -o init
    mv init initramfs/ && cd initramfs/
    cp init boot/initramfs && cp init initramfs/FOMOS_initramfs
elif [ "$ARCH" = 'armv7' ]; then
    cargo build --target=armv7-unknown-linux-gnueabihf
    arm-linux-gnueabihf-gcc -static src/main.c target/armv7-unknown-linux-gnueabihf/debug/libFOMOSv2_CL.a -o init
    mv init initramfs/ && cd initramfs/
    cp init boot/initramfs && cp init initramfs/FOMOS_initramfs
elif [ "$ARCH" = 'x86' ]; then
    cargo build --target=x86_64-unknown-linux-gnu
    x86_64-linux-gnu-gcc -static src/main.c target/x86_64-unknown-linux-gnu/debug/libFOMOSv2_CL.a -o init
    mv init initramfs/ && cd initramfs/
    cp init boot/initramfs && cp init boot/FOMOS_initramfs
else
    echo "$ARCH is invalid"
fi

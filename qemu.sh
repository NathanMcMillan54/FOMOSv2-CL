#!/bin/bash

ARCH=$1

if [ $ARCH = 'arm' ]; then
    qemu-system-arm -drive format=raw,file=target/arm-FOMOSv2/debug/bootimage-FOMOSv2.bin
elif [ $ARCH = 'x86' ]; then
    qemu-system-x86_64 -drive format=raw,file=target/x86-FOMOSv2/debug/bootimage-FOMOSv2.bin
else
  echo "$ARCH is invalid."
fi

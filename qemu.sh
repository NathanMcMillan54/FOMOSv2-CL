#!/bin/bash

ARCH=$1

cargo bootimage
if [ $ARCH = 'arm' ]; then
    echo "$ARCH"
elif [ $ARCH = 'x86' ]; then
    qemu-system-x86_64 -drive format=raw,file=target/x86-FOMOSv2/debug/bootimage-FOMOSv2.bin
else
  echo "$ARCH is invalid."
fi

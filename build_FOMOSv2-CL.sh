#!/bin/bash

echo "Compiling Linux kernel 5.9..."
echo "This might take a while."
sleep 1
make ARCH=arm versatile_defconfig
make ARCH=arm CROSS_COMPILE=arm-none-eabi-

echo "Done compiling kernel."
echo "Compiling FOMOSv2-CL..."
# Something to start compiling FOMOS

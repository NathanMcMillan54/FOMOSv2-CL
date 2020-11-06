#!/bin/bash

echo "Compiling Linux kernel 5.9..."
echo "This might take a while."
sleep 1
make ARCH=arm menuconfig
make ARCH=arm CROSS_COMPILE=arm-none-eabi-
echo "Done compiling kernel."
echo " "
echo "Run build_FOMOSv2-CL.sh to finish building."

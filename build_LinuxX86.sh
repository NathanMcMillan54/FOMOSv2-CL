#!/bin/bash

echo "Compiling Linux kernel 5.9..."
echo "This might take a while."
sleep 1
# Makefile defaults everything to x86
make menuconfig
make all
echo "Done compiling kernel."
echo " "
echo "Run build_FOMOSv2-CL.sh to finish building."

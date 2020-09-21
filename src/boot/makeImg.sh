#!/bin/bash

# stuff to make kernel.s into an img
echo "Making kernel.img"
as -o kernel.o '$<'
ld --oformat binary -o '$@' -Ttext 0x7C00 kenel.o
echo "Moving kernel.img"
mv kenel.img ../../

#!/usr/bin/env bash

# don't run this
# if you do delete everything in this directory after you're done
echo "Compiling init.S..."
as --64 -o init.o init.S
ld -o d/init init.o

echo "Compiling start,c"
gcc -static start.c -o start.c

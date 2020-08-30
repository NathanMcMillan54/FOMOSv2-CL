#!/usr/bin/env bash

echo "Compiling main.c"
gcc -Wall FOMOS.c -o FOMOS
gcc -Wall boot.boot.c -o boot/boot.c
# compile it to .iso
# compile it to .img

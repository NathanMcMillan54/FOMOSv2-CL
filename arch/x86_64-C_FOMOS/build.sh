#!/usr/bin/env bash
set -eux
as -ggdb3 --32 -o entry.o entry.S
gcc -c -ggdb3 -m16 -ffreestanding -fno-PIE -nostartfiles -nostdlib -o main.o -std=c99 main.c
ld -m elf_i386 -o main.elf -T linker.ld entry.o main.o
objcopy -O binary main.elf main.img

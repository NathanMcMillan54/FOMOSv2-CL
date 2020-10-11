#!/usr/bin/env bash
set -eux
as -ggdb3 --32 -o entry.o entry.S
rustc main.rs
ld -m elf_i386 -o main.elf -T linker.ld entry.o main
objcopy -O binary main.elf main.img

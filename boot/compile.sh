#!/usr/bin/env bash

# don't run this
# if you do delete everything in this directory after you're done
as --64 -o init.o init.S
ld -o d/init init.o

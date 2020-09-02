#!/bin/bash

echo "Compiling inti.S"
as --64 -o init.o init.S
ld -o d/init init.o

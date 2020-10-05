#!/bin/bash

qemu-system-aarch64 -machine virt \
  -m 1024M \
  -cpu intel-i5 \
  -nographic \
  -k en \
  -kernel target/x86-unknown-none/debug/FOMOSv2-CL

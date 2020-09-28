#!/bin/bash

qemu-system-aarch64 -machine virt \
  -m 1024M \
  -cpu cortex-a53 \
  -nographic \
  -k en \
  -kernel target/aarch64-unknown-none/debug/FOMOSv2-CL


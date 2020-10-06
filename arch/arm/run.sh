#!/bin/bash

qemu-system-aarch64 -machine virt \
  -m 1024M \
  -cpu cortex-a53 \
  -nographic \
  -k en \
  -kernel target/arm-FOMOSv2/debug/FOMOSv2-CL


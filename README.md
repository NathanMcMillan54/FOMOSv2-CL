# FOMOSv2-CL v2.3.5  Linux kernel v5.9


## About
FOMOS stands for: Free, Opensource, Mobile, Operating, System. FOMOS is an OS based off Linux that is meant to run on
mobile devices.

Most mobile devices have ARM CPUs so FOMOS will be supported on ARM devices first.

The Linux kernel is mainly written in C which is almost 50 years old, FOMOS will be written in Rust which just came out 
so it will hopefully be supported longer. Eventually parts of the Linux kernel will have to be replaced with Rust so 
they can work together easier.

## Setup
``Documentation/Setup/`` talks about how to compile and run FOMOSv2-CL.

### TODO
- [x] Run FOMOS as initramfs in Linux kernel (very important)
- [x] Add proper memory management (very important)
- [x] x86 compiles (very important)
- [x] x86 runs as Linux initramfs (very important)
- [ ] armv6/7 compiles (very important)
- [ ] armv6/7 runs as Linux initramfs (very important)  
- [ ] Write Documentation for FOMOS (not very important now)

## Contributions
``Documentation/editing/`` talks a lot about requirements for contributing to FOMOSv2-CL. It also talks about how to 
edit FOMOS and shows some examples of how to add things to FOMOSv2-CL.
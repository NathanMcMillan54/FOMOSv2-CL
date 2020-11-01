# FOMOSv2-CL v2.3.5 
# Linux kernel v5.9


## About
FOMOS stands for: Free, Opensource, Mobile, Operating, System. FOMOS is an OS based off Linux that is meant to run on
mobile devices.

Most mobile devices have ARM CPUs so FOMOS will be supported on ARM devices first.

The Linux kernel is mainly written in C which is almost 50 years old, FOMOS will be written in Rust which just came out 
so it will hopefully be supported longer. Eventually parts of the Linux kernel will have to be replaced with Rust so 
they can work together easier.

## Setup
``Documentation/Setup/`` talks about user and developer for FOMOSv2-CL. In that directory, it'll talk about how to 
compile FOMOS and it on a device or run it in qemu.

### TODO
- [ ] Get Linux to run a simple Rust file (important)
- [ ] Make std support for FOMOS (important)
- [ ] Write Documentation for FOMOS (not very important now)

## Contributions
``Documentation/editing/`` talks a lot about requirements for contributing to FOMOSv2-CL. It also talks about how to 
edit FOMOS and shows some examples of how to add things to FOMOSv2-CL.
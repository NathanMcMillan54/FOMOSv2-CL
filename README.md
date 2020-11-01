# FOMOSv2-CL v2.3.5 
# Linux kernel v5.9


## About
FOMOS stands for: Free, Opensource, Mobile, Operating, System. FOMOS is an OS based off Linux that is meant to run on
mobile devices.

Most mobile devices have ARM CPUs so most development will be done in ``arch/arm/``, ``arch/arm64``, and ``arch/x86/``.
FOMOS will probably be supported on a simple device like the RPi first.

The Linux kernel is mainly written in C which is almost 50 years old, FOMOS will be written in Rust which just came out 
so it will hopefully be supported longer. Eventually parts of the Linux kernel will have to be replaced with Rust so 
they can work together easier.

## Setup
``Documentation/Setup/`` talks about user and developer for FOMOSv2-CL. In that directory, it'll talk about how to 
compile FOMOS and it on a device or run it in qemu.

### TODO
- [ ] Get Linux to run a simple Rust file (important)
- [ ] Make std support for FOMOS (important)
- [ ] Rewrite printk function in Rust (important)
- [ ] Write Documentation for FOMOS (not very important now)

## Contributions
If you are contributing to FOMOSV2-CL you need to run your code at least 5 times and it needs to work everytime. If you are
trying to solve a problem and you absolutely need to push you code which isn't working, make a new branch that will be 
dedicated to solving that one problem. 

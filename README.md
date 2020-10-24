## FOMOSv2-CL v2.3.5 
## Linux kernel v5.9


### About
FOMOS stands for: Free, Opensource, Mobile, Operating, System. FOMOS is an OS based off Linux that is meant to run on
mobile devices.

Most mobile devices have ARM CPUs so most development will be done in ``arch/arm/``, ``arch/arm64``, and ``arch/x86/``.
FOMOS will probably be supported on a simple device like the RPi first.

The Linux kernel is mainly written in C which is almost 50 years old, FOMOS will be written in Rust which just came out 
so it will hopefully be supported longer. Eventually parts of the Linux kernel will have to be replaced with Rust so 
they can work together easier.

#### TODO
- [ ] Get Linux to run a simple Rust file (important)
- [ ] Rewrite printk function in Rust (important)
- [ ] Write Documentation for FOMOS (not very important now)

##### printk
``FOMOS/src/kernel/printk/`` is going to be the printk function from Linux rewritten in Rust. When printk is done being 
worked on, it will make development for FOMOS easier and that part of the Linux kernel supported longer.
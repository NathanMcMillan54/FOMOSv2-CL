## FOMOSv2-CL v2.3.5 running on Linux kernel v5.9


### FOMOS
FOMOS stands for: Free, Opensource, Mobile, Operating, System. FOMOS is an OS based off Linux that is meant to run on
mobile devices.

Most mobile devices have ARM CPUs so most development will be done in ``arch/arm/`` or ``arch/arm64``. FOMOS will 
probably be supported on a simple device like the RPi first.

The Linux kernel is mainly written in C which is almost 50 years old, FOMOS will be written in Rust which just came out 
so it will hopefully be supported longer. Eventually parts of the Linux kernel will have to be replaced with Rust so 
they can work together easier.

### Linux
There are several guides for kernel developers and users. These guides can be rendered in a number of formats, like HTML
and PDF. Please read ``Documentation/admin-guide/README.rst`` first.

In order to build the documentation, use ``make htmldocs`` or ``make pdfdocs``. The formatted documentation can also be 
read online at:

[https://www.kernel.org/doc/html/latest/](https://www.kernel.org/doc/html/latest/)

There are various text files in the Documentation/ subdirectory, several of them using the Restructured Text markup 
notation.

Please read the ``Documentation/process/changes.rst`` file, as it contains the requirements for building and running the
kernel, and information about the problems which may result by upgrading your kernel.

#### TODO
- [ ] Get Linux to run a simple Rust file (important)
- [ ] Write Documentation for FOMOS (not very important now)

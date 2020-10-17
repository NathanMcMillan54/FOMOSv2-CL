# FOMOSv2-CL
FOMOS stands for: Free, Open source, Moblie, Operating, System

Website: [sbfomos.org](https://sbfomos.org/fomos)

#

FOMOSv2-CL is the second version of FOMOS.
[FOMOSv1-Yellow](https://github.com/NathanMcMillan54/FOMOSv1-Yellow) was a GUI replacement for Ubuntu and it could be 
used on mobile devices. This will be an actual operating system. The CL in FOMOSv2-CL stands for: Command Line. FOMOSv2 
won't have a GUI it'll just be a terminal.


# Setup
In ``Documentation/Setup/`` it'll explain how to setup FOMOSv2-CL. It's recommended that you compile on Linux or a Linux
like OS.

## 2.3.5
FOMOSv2-CL v2.2.5 is it's own OS. But because of that there isn't support for anything. FOMOSv2-CL v2.3.5 will be based
off [Linux kernel v5.9](https://github.com/torvalds/linux/releases/tag/v5.9). Click 
[here](https://github.com/NathanMcMillan54/FOMOSv2-CL/tree/FOMOS_linux) to see the branch where FOMOSv2-CL v2.3.5 is
being developed.

FOMOSv2-CL v2.3.5 will still use Rust as it's main language like it does as of right now. Eventually some parts of the
Linux kernel will be rewritten in Rust so it'll work better with FOMOS.

If you're thinking about contributing to FOMOS don't do it on the master branch unless it's very important. The only 
reason why you should be working on this branch is if you know how to make drivers for everything needed to make it 
usable. Before FOMOS_linux is merged to master, the master branch will be added to a branch that will have FOMOSv2-CL 
v2.2.5.

## TODO

### FOMOS
- [ ] Write proper documentation
- [ ] Make all commands work on most CPUs

### ARM
- [x] Get setup working
- [ ] Get drivers working
- [x] Get command line working
- [x] Get boot working
- [ ] Make OS image
- [ ] Make support for RPi


### x86
- [ ] Get setup working
- [ ] Get drivers working
- [ ] Get command line working
- [ ] Get boot working (from assembly)
- [x] Make OS image


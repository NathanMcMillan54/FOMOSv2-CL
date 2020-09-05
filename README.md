# About

FOMOS stands for: Free, Open source, Mobile, Operating, System.

FOMOS should only works on ARM CPU's

The first version of FOMOS is just a GUI replacement for Linux but this will be a real OS.

In version 3 of FOMOS it should be a mix of FOMOSv1-Yellow and FOMOSv2-CL (this version of FOMOS).

FOMOSv2-CL is based of Linux, most of what is being copied from linux is from [linux/arch/arm/](https://github.com/torvalds/linux/tree/master/arch/arm).

# Requirements

The requirements are unknown right now but are likely:
    
- 128MB storage
- 16MB memory

Definitely ARM CPU

# Setup

```commandline
sh buildFOMOS.sh
```

## TODO

FOMOS is supposed to be for mobile devices. Most mobile devices have an ARM CPU so the bootloader has to work on ARM CPU devices.
The most powerful ARM device that runs any software I can think of is the RaspberryPi. So that will probably be the first device to support FOMOS.  

- [ ] Get boot working
- [ ] Make support for RPi 3
- [ ] Edit all the ``.txt`` files in ``FOMOS/commands/help`` because that's pretty much the documentation for FOMOS
- [x] Get everything from [FOMOSv2-CL v2.1.1-beta](https://github.com/NathanMcMillan54/FOMOSv2-CLtest) into this FOMSOv2-CL

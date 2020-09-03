# About

FOMOS stands for: Free, Open source, Mobile, Operating, System.

FOMOS is based off Arch Linux and RaspberryPi OS.

Arch Linux, because it has good support for ARM CPU's.
RaspberryPi OS, because a RPI is literally a mobile device without a screen and battery.

The first version of FOMOS is just a GUI replacement for Linux but this will be a real OS.

In version 3 of FOMOS it should be a mix of FOMOSv1-Yellow and FOMOSv2-CL (this version of FOMOS).

# Setup

Run:
```commandline
sh FOMOS.sh
```

That'll explain most things about FOMOS.

```commandline
make all
make run
```

That'll tell you if your CPU can run FOMOS.

#

Compile with GCC (GNU, Compiler, Collection) on Linux.

```commandline
cd FOMOS/
make all
make run
```

That'll compile and run FOMOS.


## TODO

FOMOS is supposed to be for mobile devices. Most mobile devices have an ARM CPU so the bootloader has to work on ARM CPU devices.
The most powerful ARM device that runs any software I can think of is the RaspberryPi. So that will probably be the first device to support FOMOS.  

- [ ] Get bootloader working
- [ ] Get everything from [FOMOSv2-CL v2.1.1-beta](https://github.com/NathanMcMillan54/FOMOSv2-CLtest) into this FOMSOv2-CL

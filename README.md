# About

FOMOS stands for: Free, Open source, Mobile, Operating, System.

The first version of FOMOS is just a GUI replacement for Linux but this will be a real OS.

In version 3 of FOMOS it should be a mix of FOMOSv1-Yellow and FOMOSv2-CL (this version of FOMOS).

# Setup

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
- [ ] Make support for RPi 3 and 4
- [ ] Get everything from [FOMOSv2-CL v2.1.1-beta](https://github.com/NathanMcMillan54/FOMOSv2-CLtest) into this FOMSOv2-CL

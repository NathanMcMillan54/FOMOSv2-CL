# About

FOMOS stands for: Free, Open source, Mobile, Operating, System.

FOMOS should only works on ARM CPU's

The first version of FOMOS is just a GUI replacement for Linux but this will be a real OS.

In version 3 of FOMOS it should be a mix of FOMOSv1-Yellow and FOMOSv2-CL (this version of FOMOS).

# Setup

```commandline
sh buildFOMOS.sh
```

Running that will compile all the C files in FOMOS

```commandline
cd boot/
sh compileAll.sh
./makeImage
```

That compiles all files in ``boot/`` then turns ``FOMOS/`` into a ``.img`` file.

## TODO

FOMOS is supposed to be for mobile devices. Most mobile devices have an ARM CPU so the bootloader has to work on ARM CPU devices.
The most powerful ARM device that runs any software I can think of is the RaspberryPi. So that will probably be the first device to support FOMOS.  

- [ ] Get boot working (for ARM CPU)
- [ ] Make support for RPi 3
- [ ] Edit all the ``.txt`` files in ``FOMOS/commands/help`` because that's pretty much the documentation for FOMOS
- [x] Get everything from [FOMOSv2-CL v2.1.1-beta](https://github.com/NathanMcMillan54/FOMOSv2-CLtest) into this FOMSOv2-CL (all commands)

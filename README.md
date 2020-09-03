# About

FOMOS stands for: Free, Open source, Mobile, Operating, System.

FOMOS is based off Arch Linux and RaspberryPi OS.

Arch Linux, because it has good support for ARM CPU's.
RaspberryPi OS, because a RPI is literally a mobile device without a screen and battery.

FOMOS only works on ARM CPU's

The first version of FOMOS is just a GUI replacement for Linux but this will be a real OS.

In version 3 of FOMOS it should be a mix of FOMOSv1-Yellow and FOMOSv2-CL (this version of FOMOS).

# Setup

```commandline
sh BUILDFOMOS.sh
```

```commandline
cd buildroot
make menuconfig
```

## TODO

FOMOS is supposed to be for mobile devices. Most mobile devices have an ARM CPU so the bootloader has to work on ARM CPU devices.
The most powerful ARM device that runs any software I can think of is the RaspberryPi. So that will probably be the first device to support FOMOS.  

- [ ] Get boot
- [ ] Get everything from [FOMOSv2-CL v2.1.1-beta](https://github.com/NathanMcMillan54/FOMOSv2-CLtest) into this FOMSOv2-CL

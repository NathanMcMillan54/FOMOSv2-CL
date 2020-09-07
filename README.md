# About

FOMOS stands for: Free, Open source, Mobile, Operating, System.

Most mobile devices have ARM CPU's, because FOMOS is meant to run on mobile devices it should have good support for them.

The first version of FOMOS is just a GUI replacement for Linux but this will be a real OS.

In version 3 of FOMOS it should be a mix of FOMOSv1-Yellow and FOMOSv2-CL (this version of FOMOS).

## Setup requirements

- Linux v2.6 +
- C v18 +
- gcc v9.3.0 +
- Rust v1.38.0 +
- Cargo v1.46.0 +

# Setup
## Use Linux for setup

```commandline
sh buildFOMOS.sh
```

Running that will compile all the C files in FOMOS

```commandline
cd FOMOSimg/
sh compileAll.sh
./makeImage
```

That compiles all files in ``FOMOSimg/`` then turns ``FOMOS/`` into a ``.img`` file.

```commandline
cd boot
cargo build
```

That compiles all the startup files in ``boot/``. Make sure ``cargo`` commands work on your device.

## TODO

FOMOS is supposed to be for mobile devices. Most mobile devices have an ARM CPU so the bootloader has to work on ARM CPU devices.
The most powerful ARM device that runs any software I can think of is the RaspberryPi. So that will probably be the first device to support FOMOS.  

- [ ] Get boot working (for ARM CPU)
- [x] Make support for RPi 3
Boot probably works, FOMOS probably uses to much memory so that might not work.
But at least it boots.
- [ ] Edit all the ``.txt`` files in ``FOMOS/commands/help`` because that's pretty much the documentation for FOMOS
- [x] Get everything from [FOMOSv2-CL v2.1.1-beta](https://github.com/NathanMcMillan54/FOMOSv2-CLtest) into this FOMSOv2-CL (all commands)

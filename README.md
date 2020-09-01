# About

FOMOS stands for: Free, Open source, Mobile, Operating, System.

The first version of FOMOS is just a GUI replacement for Linux but this will be a real OS.

In version 3 of FOMOS it should be a mix of FOMOSv1-Yellow and FOMOSv2-CL (this version of FOMOS).

# Setup
```commandline
cd FOMOS/
```

```commandline
make all
```

```commandline
make run
```

That'll compile and run FOMOS

#

```commandline
make all
```

If you do this in ```FOMOSv2-CL``` (very top of the the project) 

That will compile ```main.c```, ```start.ld```, and ```startup.s``` and put them in ```lib/``` that will run everything on startup.

It's for ARM CPUs only.

## TODO

- [ ] Get boot working
- [ ] Get everything from [FOMOSv2-CLtest](https://github.com/NathanMcMillan54/FOMOSv2-CLtest) into this FOMSOv2-CL

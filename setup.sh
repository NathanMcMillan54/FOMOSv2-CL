#!/usr/bin/env bash

echo "Running requirements.sh"
sh requirements.sh

echo "Starting FOMOS..."
# this doesn't start fomos
# it just starts a tutorial for fomos that will make the user setup fomos by themself
gcc -Wall setup.c -o setup
./setup

echo "Deleting setup.c"
rm -rf setup.c
rm -rf setup

echo "Running FOMOSCL/startup/CompileFOMOSCL.sh"
sh FOMOSCL/startup/CompileFOMOSCL.sh

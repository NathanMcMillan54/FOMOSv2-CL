#!/usr/bin/env bash

cd FOMOSCL/
echo "Compiling Files"
gcc -Wall CL/usrLogin.c -o CL/usrLogin
gcc -Wall CL/main.c -o CL/main
gcc -Wall helpFiles/help.c -o helpFiles/help
gcc -Wall power/powerOff.c -o power/powerOff
gcc -Wall power/restart.c -o power/restart
gcc -Wall iwi/iwi.c -o iwi/iwi
gcc -Wall usrNam/usrNam.c -o usrNam/usrNam
gcc -Wall netConnect/netConnect.c -o netConnect/netConnect
gcc -Wall Fopen/Fopen.c -o Fopen/Fopen

sh startup/StartCL.sh

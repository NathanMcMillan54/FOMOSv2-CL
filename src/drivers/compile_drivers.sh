#!/bin/bash

echo "Compiling keyboard..."
cd keyboard/
make all
sleep 1
echo "Test keyboard_drivers drivers..."
./keyboard_drivers
echo $?
sleep 1
cd ../

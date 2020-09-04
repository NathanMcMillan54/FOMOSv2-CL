#!/bin/bash

echo "Updating FOMOS..."
git pull
echo "Done"

make all
make run

function askYoN() {
    read -p "Do you want to continue? (y/n) " input
    if [ "$input" = "y" ]; then
        echo "Making files"
        cd boot/
        make all
        cd ../
        cd FOMOS/
        make all
        cd commands/
        make all
        cd ../
        cd drivers/
        cd power/
        make all
        cd../../../
        echo "Done"
    elif [ "$input" = "n" ]; then
        echo "You will have to manually build FOMOS"
    else
        echo "Enter y or n"
        askYoN
    fi
}

echo "  "
echo "Done building FOMOS"
echo "Put FOMOS onto a USB/SD card then in run it on an ARM device"

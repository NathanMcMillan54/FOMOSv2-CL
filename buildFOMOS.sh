#!/bin/bash

echo "Updating FOMOS..."
git pull
echo "Done"

make all
make run

echo "  "

function askYoN() {
    read -p "Do you want to continue? (y/n) " input
    if [ "$input" = "y" ]; then
        echo "Making files"
        cd FOMOS/
        make all
        cd commands/
        echo "Installing FMOE the FOMOS text editor"
        git clone https://github.com/NathanMcMillan54/FMOE
        echo "Done"
    elif [ "$input" = "n" ]; then
        echo "You will have to manually build FOMOS and install FMOE"
    else
        echo "Enter y or n"
        askYoN
    fi
}

echo "  "
echo "Done building FOMOS"
echo "Put FOMOS onto a USB/SD card then in run it on an ARM device"

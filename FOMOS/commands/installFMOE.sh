#!/bin/bash

echo "If you answer 'n' you will not be able to go back after FOMOS has been loaded onto a device."
read -p "Would you like to install FMOE the FOMOS text editor? (y/n) " input
if [ "$input" = "y" ]; then
    echo "Installing FMOEv1.0.1"
    git clone https://github.com/NathanMcMillan54/FMOE.git
    cd FMOE/
    sh setup.sh
elif [ "$input" = "n" ]; then
    exit
else
    echo "Did not enter y/n"
    echo "Installing FMOE anyway"
    echo "Installing FMOEv1.0.1"
    git clone https://github.com/NathanMcMillan54/FMOE.git
    cd FMOE/
    sh setup.sh
fi

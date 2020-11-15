#!/bin/bash

requirements() {
    # Install all build requirements
    echo "Installing requirements..."
}

linux() {
    echo "Installing Linux..."
    git clone https://github.com/sbFomos/linux.git
    cd linux/
    make all ARCH=arm
}

main() {
  requirements
  linux
}

main

#!/bin/bash

function requirements() {
    # Install all build requirements
    echo "Installing requirements..."
}

function linux() {
    echo "Installing Linux..."
    git clone https://sbFomos/linux.git
    cd linux/
    sh build_arm.sh
}

function main() {
  requirements
  linux
}

main

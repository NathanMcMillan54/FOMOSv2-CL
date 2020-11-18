#!/bin/bash

requirements() {
    # Install all build requirements
    echo "Installing requirements..."
}

linux() {
    echo "Installing Linux..."
    git clone https://github.com/sbFomos/linux.git
}

main() {
  requirements
  linux
}

main

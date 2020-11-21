#!/bin/bash

clean() {
    rm -rf linux/
    rm -rf target/
}

requirements() {
    echo "Installing requirements..."
    sudo apt-get upgrade
    rustup update
    rustup target add x86_64_linux_gnu
    rustup target add armv7-unknown-linux-gnueabihf
}

linux() {
    echo "Installing Linux..."
    git clone https://github.com/sbFomos/linux.git
    cd linux/
    git reset --hard origin/v5.9

}

main() {
  clean
  requirements
  linux
}

main

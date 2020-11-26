#!/bin/bash

clean() {
    rm -rf target/
    rm -rf init
    rm -rf rootfs.cpio.gz
}

requirements() {
    echo "Installing requirements..."
    sudo apt-get upgrade
    rustup update
    rustup target add x86_64_linux_gnu
    rustup target add armv7-unknown-linux-gnueabihf
    rustup target add armv6-unknown-linux-gnueabihf
    echo "Installing FOMOS-dev-library..."
    git clone https://gtihub.com/sbFomos/FOMOS-dev-library.git
}

linux() {
    echo "Installing Linux..."
    cd ../
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

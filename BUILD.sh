#!/bin/bash

ARCH=$1

function main() {
    if [ $ARCH = 'arm' ] || [ $ARCH = 'x86' ]; then
        echo "Building for CPU archetype: $ARCH"
    else
        echo "$ARCH is invalid"
        exit
    fi
    sh install.sh
    make all
}

main

#!/bin/bash

echo "Compiling drivers..."
cd src/drivers/
sh compile_drivers.sh
cd ../../
echo "Compiling FOMOS..."
rm -rf target/
cargo xbuild --target=arm-FOMOSv2.json

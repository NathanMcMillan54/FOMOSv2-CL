#!/bin/bash

# compressed.sh
#
# Description
# Compresses FOMOS for Linux
#

echo "Compressing FOMOSv2-CL..."
cd initramfs/
find . | cpio -o -H newc | gzip > ../FOMOSv2-CL_v2_3_5.cpio.gz

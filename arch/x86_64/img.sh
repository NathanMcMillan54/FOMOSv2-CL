#!/bin/bash

read -p "Inout USB device name " USB_name

dd if=target/x86_64-FOMOSv2/debug/bootimage-FOMOSv2-CL.bin of=/dev/"$USB_name" && sync

#!/usr/bin/env bash

echo "Local networks: "
for i in $(ls /sys/class/net/ | egrep -v ^lo$); do sudo iw dev $i scan | grep SSID | awk '{print substr($0, index($0,$2)) }'; done 2>/dev/null | sort -u

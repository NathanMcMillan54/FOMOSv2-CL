#!/usr/bin/env bash

read -p "Type network name: " ssid
read -p "Type $ssid password: " password


sudo nmcli dev wifi connect ${ssid} password ${password}

echo "Connected to $ssid"
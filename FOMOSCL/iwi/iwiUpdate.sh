#!/usr/bin/env bash

echo "This file is for updating things installed with iwi"

read -p "UPDATE " iwiURL

if [ "$iwiURL" = "FOMOS" ]; then
    git pull
else
    cd CL/usrFiles/$iwiURL
    git pull
fi

#!/bin/bash

cargo build --target x86_64-FOMOSv2.json
cargo bootimage

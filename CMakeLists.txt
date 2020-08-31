cmake_minimum_required(VERSION 3.16)
project(FOMOSv2-CL C)

set(CMAKE_C_STANDARD 11)

add_executable(FOMOSv2-CL main.c include/system/config.h boot/main.c include/system/cpu.h include/system/ram.h include/system/disk.h)
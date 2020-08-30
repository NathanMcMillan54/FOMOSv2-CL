cmake_minimum_required(VERSION 3.16)
project(FOMOSv2-CL C)

set(CMAKE_C_STANDARD 11)

add_executable(FOMOSv2-CL FOMOS.c boot/boot.c include/system/startFOMOS.h include/system/stdlib.h include/output/stdio.h include/output/stdarg.h)

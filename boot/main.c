/* This file will run on startup soon */

#include <stdio.h>
#include "../include/system/cpu.h"
#include "../include/system/ram.h"
#include "../include/system/disk.h"

void main(void) {
    printf("Starting FOMSOv2-CL... \n");
    cpu();
    ram();
    disk();
    system("../main");
}

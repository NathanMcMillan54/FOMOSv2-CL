#include <stdio.h>
#include <stdlib.h>
#include "strtCPU.h"

int main(void) {
    printf("Starting FOMOS.img... \n");
    strtCPU();
    // don't actually start setup.c
    // just do this for now unitl the os can be figured out
    printf("Starting setup... \n");
    system("./FOMOS/setup.c");
}

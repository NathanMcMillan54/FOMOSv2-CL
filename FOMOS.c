#include <stdio.h>
#include <stdlib.h>
#include "strtCPU.h"

int main(void) {
    printf("Starting FOMOS.img... \n");
    strtCPU();
    printf("If startup has gotten this far everything should be working");
    printf("Starting setup... \n");
    system("./FOMOS/setup");
}

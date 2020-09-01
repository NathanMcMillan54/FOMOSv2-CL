/* This file is for shutting down and/or restarting fomos */

#ifndef FOMOSV2_CL_POWER_H
#define FOMOSV2_CL_POWER_H

#include <stdio.h>
#include <stdlib.h>

void restart() {
    printf("Restarting... \n");
    system("sh ../../makePower.sh");
    system("../../power/restart")
}

void shutdown() {
    printf("Shutting down... \n");
    system("sh ../../makePower.sh");
    system("../../power/shutdown")
}

#endif //FOMOSV2_CL_POWER_H

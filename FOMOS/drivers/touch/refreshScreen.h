/* This file is used for refreshing the screen */

#ifndef FOMOSV2_CL_REFRESH_H
#define FOMOSV2_CL_REFRESH_H

#include <unistd.h>
#include <stdio.h>
#include "../../include/FOMOS/power/power.h"

int fps;

void refresh_screen() {
    for (;;) {
        if (fps == 0) {
            printf("Screen refresh rate is very low \n");
        } else if (fps < 0) {
            printf("Screen refresh rate has broken \n");
            printf("Restarting to try to fix the problem \n");
            restart();
        }
        usleep(60);
    }
}

#endif //FOMOSV2_CL_REFRESH_H

/* This file is for determining weather the screen is being used or not */

#ifndef FOMOSV2_CL_ACTIVESCREEN_H
#define FOMOSV2_CL_ACTIVESCREEN_H

#include <stdio.h>
#include "../../include/FOMOS/power/power.h"

// the screen has been active/used recently
int active = 1;

void check_if_screen_is_active() {
    if (active == 0) {
        printf("Screen has been inactive \n");
        printf("Shutting down to save power... \n");
        shutdown();
    } else if (active == 1) {
        // nothing
    } else if (active > 1) {
        printf("There is a problem \n");
        printf("Restarting to try to fix problem \n");
        restart();
    } else if (active < 0) {
        printf("There is a problem \n");
        printf("Restarting to try to fix problem \n");
        restart();
    }
}

#endif //FOMOSV2_CL_ACTIVESCREEN_H

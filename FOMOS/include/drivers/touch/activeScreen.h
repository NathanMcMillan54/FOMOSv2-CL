/* This file is for detirmening wether the screen is being used or not */

#ifndef FOMOSV2_CL_ACTIVESCREEN_H
#define FOMOSV2_CL_ACTIVESCREEN_H

#include <stdio.h>
#include "../../FOMOS/power.h"

// the screen has been active/used recently
int active = 1;

void check_if_screen_is_active() {
    if (active == 0) {
        printf("Screen has been inactive \n");
        printf("Shutting down to save power... \n");
        shutdown();
    }
}

#endif //FOMOSV2_CL_ACTIVESCREEN_H

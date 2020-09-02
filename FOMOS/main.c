#include <stdio.h>
#include <stdlib.h>
#include "include/FOMOS/finishSetup.h"
#include "include/FOMOS/power/power.h"

int power = 1;

int main() {
    // finish_setup();
    int f = 0;
    printf("\a");
    printf("FOMOSv2-CL \n");

    for (;;) {
        if (power == 0) {
            exit(0);
        } else {
            // nothing
        }
    }

}

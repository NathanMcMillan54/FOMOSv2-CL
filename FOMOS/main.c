#include <stdio.h>
#include <stdlib.h>
#include "include/FOMOS/finishSetup.h"
#include "include/FOMOS/power/power.h"
#include "include/time/countTime.h"

int power = 1;

int main() {
    finish_setup();
    int f = 0;
    printf("\a");
    printf("FOMOSv2-CL \n");

    s1();
    for (;;) {
        f ++;
        m1();
        if (f == 8) {
            printf("FOMOS is shutting down \n");
            power = 0;
            exit(0);
        } else {
            // nothing
        }
        if (power == 0) {
            shutdown();
        }
    }
}

#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include "include/FOMOS/finishSetup.h"
#include "include/FOMOS/power/power.h"

int power = 1;

int main() {
    // finish_setup();
    int f = 0;
    printf("\a");
    printf("FOMOSv2-CL \n");
    system("sh users/user.sh");
    for (;;) {
        f ++;
        sleep(60);
        if (f == 8) {
            printf("Shutting down FOMOS... \n");
            shutdown();
            exit(0);
        } else {
            // nothing
        }
    }

}

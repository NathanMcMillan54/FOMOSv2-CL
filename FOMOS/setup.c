#include <stdio.h>
#include "include/FOMOS/countTime.h"
#include "include/FOMOS/makeUser.h"

int startSetup() {
    makeUser();
    s1();
    return 0;
}

int startFomos() {
    return 0;
}

int restart() {
    return 0;
}

int main() {
    int setup = 1;

    if (setup = 1) {
        startSetup();
    } else if (setup = 0) {
        startFomos();
    } else {
        printf("\a");
        printf("There was a problem \n");
        printf("Restarting your device now \n");
        restart();
    }

    return 0;
}

#include <stdio.h>
#include <unistd.h>
#include "include/FOMOS/user/makeUser.h"

int startSetup() {
    makeUser();
    sleep(1);
    printf("Starting FOMOS 2/2 Finished \n");
    system("./main");
    return 0;
}

void main(void) {
    startSetup();
    return 0;
}

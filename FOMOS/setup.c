#include <stdio.h>
#include <unistd.h>
#include "include/FOMOS/commands/power.h"
#include "include/FOMOS/user/makeUser.h"
#include "include/FOMOS/user/makeUserName.h"

int startSetup() {
    // make the user
    // (name, password)
    makeUser();
    // make the user name for usrNam command
    make_user_name();
    sleep(1);
    return 0;
}

void main(void) {
    startSetup();
    printf("Starting FOMOS 2/2 Finished \n");
    printf("Restarting to apply changes... \n");
    restart();
    return 0;
}

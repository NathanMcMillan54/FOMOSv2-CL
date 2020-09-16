#include <stdio.h>
#include <unistd.h>
#include "include/FOMOS/commands/power.h"
#include "include/FOMOS/user/makeUser.h"
#include "include/FOMOS/user/makeUserName.h"
#include "include/FOMOS/user/FOMOS_sudo.h"

int startSetup() {
    // make the user
    // (name, password)
    makeUser();
    // make the user name for usrNam command
    make_user_name();
    make_sudo_password();
    sleep(1);
    return 0;
}

void main(void) {
    startSetup();
    printf("Starting FOMOS 2/4 Finished \n");
    printf("Restarting to apply changes... \n");
    restart();
    return 0;
}

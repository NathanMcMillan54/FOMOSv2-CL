// this is for all the user commands
#ifndef FOMOSV2_CL_USER_H
#define FOMOSV2_CL_USER_H

#include <stdio.h>
#include <stdlib.h>

void sudo() {
    system("sh users/sudo.sh");
}

void usrNam() {
    system("sh users/userName.sh");
}

#endif //FOMOSV2_CL_USER_H

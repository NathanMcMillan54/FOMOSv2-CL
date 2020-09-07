/* this file is for making the usrName command from FOMOS v2.1.1-beta work
 * if you tried running usrNam in FOMOSv2.1.1-beta it's actually running a linux command.
 * During setup for FOMOS this file will run, it'll ask the user for their name and add it to a shellscript,
 * that shellscript will run if you type usrNam
 * (hopefully) */

#ifndef FOMOSV2_CL_MAKEUSERNAME_H
#define FOMOSV2_CL_MAKEUSERNAME_H

#include <stdio.h>

void make_user_name() {
    char userName[100];

    printf("Enter your User Name: ");
    scanf("%s", userName);

    FILE *userNameFile;

    if ((userNameFile = fopen("users/userName.sh", "a")) == NULL)
        printf("Cannot make User Name");
    else {
        fputs(userName, userNameFile)
        fclose(userNameFile);
    }

    printf("User Name created successfully \n");

}

#endif //FOMOSV2_CL_MAKEUSERNAME_H

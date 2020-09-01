/* This file is for making all the stuff the user needs */

#ifndef FOMOSV2_CL_MAKEUSER_H
#define FOMOSV2_CL_MAKEUSER_H

#include <stdio.h>
#include <stdlib.h>

void makeUser() {
    char userName[100];
    char userPassword[6];

    printf("Enter your name: ");
    scanf("%s", userName);

    printf("Enter a 6 digit password: ");
    scanf("%s", userPassword);

    FILE *usrFile;
    char l8p33[100] = "if [ "$password" = ";
    char l8p66[100] = userPassword;
    char l8p99[100] = " ]; then \n";

    if ( (usrFile = fopen("users/user.sh", "a") ) == NULL)
        printf("Cannot open add user");
    else {
        fputs(l8p33, usrFile);
        fputs(l8p66, usrFile);
        fputs(l8p99, usrFile);
        fclose(usrFile);
    }

    printf("User created successfully \n");

}

#endif //FOMOSV2_CL_MAKEUSER_H

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
    // this is adding stuff to user/sh
    char l8p33[100] = "if [ $password = ";
    // char l8p66[100] = userPassword[6];
    char l8p66[100] = " ]; then \n";
    char l9p33[100] = "    ./main\n";
    char l10p33[100] = "else \n";
    char l11p33[100] = "    echo Wrong password\n";
    char l12p33[100] = "    ./power/restart\n";

    if ( (usrFile = fopen("users/makePower.sh", "a") ) == NULL)
        printf("Cannot open add user");
    else {
        fputs(l8p33, usrFile);
        fputs(userPassword, usrFile);
        fputs(l8p66, usrFile);
        fputs(l9p33, usrFile);
        fputs(l10p33, usrFile);
        fputs(l11p33, usrFile);
        fputs(l12p33, usrFile);
        fclose(usrFile);
    }

    printf("User created successfully \n");

}

#endif //FOMOSV2_CL_MAKEUSER_H

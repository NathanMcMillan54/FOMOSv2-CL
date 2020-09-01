/* This file is for making all the stuff the user needs */

#ifndef FOMOSV2_CL_MAKEUSER_H
#define FOMOSV2_CL_MAKEUSER_H

#include <stdio.h>
#include <stdlib.h>

void makeUserName() {
    FILE * usrFile;
    char usrName[100];

    printf("Enter your name: ");
    scanf("%s", usrName);

    usrFile = fopen(usrName, "w");

    if(usrFile == NULL) {
        printf("Unable to make user name \n");
        exit(EXIT_FAILURE);
    }

    fclose(usrFile);


    printf("User name created successfully \n");

}

void makeUserPassword() {

}

#endif //FOMOSV2_CL_MAKEUSER_H

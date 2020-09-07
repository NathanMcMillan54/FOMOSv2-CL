/* This file is for getting FOMOS's version of sudo to work */
#ifndef FOMOSV2_CL_SUDO_H
#define FOMOSV2_CL_SUDO_H

#include <stdio.h>
#include <stdlib.h>

void sudo_password() {
    char userSudoPassword[6];
    printf("Enter your password ");
    scanf("%s", userSudoPassword);

    FILE *sudoFile;
    // this is adding stuff to user/sudo.sh
    char l8p33[100] = "if [ $password = ";
    char l8p66[100] = userSudoPassword;
    char l8p66[100] = " ]; then \n";
    char l9p33[100] = "    ./main\n";
    char l10p33[100] = "else \n";
    char l11p33[100] = "    echo Wrong password\n";

    if ((sudoFile = fopen("users/sudo.sh", "a")) == NULL)
        printf("Cannot  add user");
    else {
        fputs(l8p33, sudoFile);
        fputs(userPassword, sudoFile);
        fputs(l8p66, sudoFile);
        fputs(l9p33, sudoFile);
        fputs(l10p33, sudoFile);
        fputs(l11p33, sudoFile);
        fputs(l12p33, sudoFile);
        fclose(sudoFile);
    }

    printf("Sudo password created successfully \n");

}

#endif //FOMOSV2_CL_SUDO_H

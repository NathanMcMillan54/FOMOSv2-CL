#include <stdio.h>

int addPassword(char password) {
    FILE *fp;
    fp = fopen("/configs/user/password", "r+");
    fputs(&password, fp);
    fclose(fp);
    return 0;
}

int makePassword() {
    char inputPassword[24];
    scanf("%s", inputPassword);
    addPassword(*inputPassword);
}


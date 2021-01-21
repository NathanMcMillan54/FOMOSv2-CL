#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

#include "config.h"

void printName() {
    char str[MAXCHAR];
    FILE *fp;
    fp = fopen("/configs/user/name", "r");
    while (fgets(str, MAXCHAR, fp) != NULL) {
        printf("Hello %s\n", str);
    }
}

void makeUserName() {
    FILE *fp;
    char inputName[100];
    scanf("%[^\n]", inputName);
    fp = fopen("/configs/user/name", "a");
    fputs(inputName, fp);
    fclose(fp);
    printName();
}

void makePassword() {
    FILE *fp;
    char str[MAXCHAR];
    char inputPassword[24];
    scanf("%s", inputPassword);
    fp = fopen("/configs/user/password", "a");
    fputs(inputPassword, fp);
    while (fgets(str, MAXCHAR, fp) != NULL)
        printf("\nPassword = %s\n\n", str);
    fclose(fp);
}

void updateStartupTimes(int updated) {
    FILE *fp;
    char updateChar[] = {'0', updated, '\0'};
    fp = fopen("/configs/boot/startupTimes", "a");
    fputs(updateChar, fp);
    fclose(fp);
}

void configSetup() {
    FILE *fp;
    char str[MAXCHAR];
    int inte;
    int updatedInte;

    fp = fopen("/configs/boot/startupTimes", "r");
    while (fgets(str, MAXCHAR, fp) != NULL)
        printf("\nStartup times = %s\n\n", str);
    inte = atoi(str);
    updatedInte = inte + 1;
    if (inte == 0) {
        updateStartupTimes(updatedInte);
        fclose(fp);
        first_setup();
    } else {
        updateStartupTimes(updatedInte);
        fclose(fp);
        regular_setup();
    }
}

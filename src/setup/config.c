#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

#include <linux/fs.h>

#include "config.h"


void printName() {
    char str[MAXCHAR];
    FILE *fp;
    fp = fopen("/configs/user/name", "r");
    while (fgets(str, MAXCHAR, fp) != NULL) {
        printf("Hello %s\n", str);
    }
    fclose(fp);
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
    fclose(fp);
}

void updateStartupTimes(int updated) {
    FILE *fp;
    printf("This function was called with the argument updated which = %d\n", updated);
    printf("Adding %d to /configs/boot/startupTimes...\n", updated);
    fp = fopen("/configs/boot/startupTimes", "w");
    fputs("1", fp);
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
    fclose(fp);
    if (inte == 0) {
        updateStartupTimes(updatedInte);
        fp = fopen("/configs/boot/startupTimes", "r");
        while (fgets(str, MAXCHAR, fp) != NULL)
            printf("\nStartup times is now = %s\n\n", str);
        fclose(fp);
        first_setup();
    } else {
        updateStartupTimes(updatedInte);
        regular_setup();
    }
}

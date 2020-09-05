#include <stdio.h>
#include <stdlib.h>
#include "../../include/FOMOS/openFile.h"

int oFOMOS() {
    FILE *fp;
    char str[MAXCHAR];
    char *fileName = "commands/help/FOMOS.txt";

    fp = fopen(fileName, "r");
    if (fp == NULL){
        printf("Could not open file %s", fileName);
        return 1;
    }
    while (fgets(str, MAXCHAR, fp) != NULL)
        printf("%s", str);
    fclose(fp);
    return 0;
}

int oFMOE() {
    FILE *fp;
    char str[MAXCHAR];
    char *fileName = "commands/help/FMOE.txt";

    fp = fopen(fileName, "r");
    if (fp == NULL){
        printf("Could not open file %s", fileName);
        return 1;
    }
    while (fgets(str, MAXCHAR, fp) != NULL)
        printf("%s", str);
    fclose(fp);
    return 0;
}

int oRestart() {
    FILE *fp;
    char str[MAXCHAR];
    char *fileName = "commands/help/restart.txt";

    fp = fopen(fileName, "r");
    if (fp == NULL){
        printf("Could not open file %s", fileName);
        return 1;
    }
    while (fgets(str, MAXCHAR, fp) != NULL)
        printf("%s", str);
    fclose(fp);
    return 0;
}

int oShutdown() {
    FILE *fp;
    char str[MAXCHAR];
    char *fileName = "commands/help/shutdown.txt";

    fp = fopen(fileName, "r");
    if (fp == NULL){
        printf("Could not open file %s", fileName);
        return 1;
    }
    while (fgets(str, MAXCHAR, fp) != NULL)
        printf("%s", str);
    fclose(fp);
    return 0;
}

int main() {
    int num;
    printf("Help\n");
    printf("______________________________________\n");
    printf("| 1) About FOMOS   2) About FMOE     |\n");
    printf("| 3) About restart 4) About shutdown |\n");
    printf("--------------------------------------\n");

    printf("Enter a number \n");
    scanf("%d", &num);

    switch (num) {
        case 1:
            oFOMOS();
            break;
        case 2:
            oFMOE();
            break;
        case 3:
            oRestart();
            break;
        case 4:
            oShutdown();
            break;
        default:
            printf("NaN \n");
            exit(0);
    }

    return 0;
}

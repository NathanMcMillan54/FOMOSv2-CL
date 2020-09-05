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

int oFopen() {
    FILE *fp;
    char str[MAXCHAR];
    char *fileName = "commands/help/Fopen.txt";

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

int oUsrNam() {
    FILE *fp;
    char str[MAXCHAR];
    char *fileName = "commands/help/usrName.txt";

    fp = fopen(fileName, "r");
    if (fp == NULL) {
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
    printf("+------------------------------------+\n");
    printf("| 1) About FOMOS   2) About FMOE     |\n");
    printf("| 3) About restart 4) About shutdown |\n");
    printf("| 5) About Fopen   6) About usrNam   |\n");
    printf("+------------------------------------+\n");

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
        case 5:
            oFopen();
            break;
        case 6:
            oUsrNam();
            break;
        default:
            printf("NaN \n");
    }

    return 0;
}

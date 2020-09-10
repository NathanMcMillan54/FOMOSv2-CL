
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include "../headerFiles/openFile.h"

int oFOMOSCL() {
    FILE *fp;
    char str[MAXCHAR];
    char *fileName = "helpFiles/FOMOSCL.txt";

    fp = fopen(fileName, "r");
    if (fp == NULL){
        printf("Could not open this help file %s", fileName);
        return 1;
    }
    while (fgets(str, MAXCHAR, fp) != NULL)
        printf("%s", str);
    fclose(fp);
    return 0;
}

int oCommands() {
    FILE *fp;
    char str[MAXCHAR];
    char *fileName = "helpFiles/commands.txt";

    fp = fopen(fileName, "r");
    if (fp == NULL){
        printf("Could not open this help file %s", fileName);
        return 1;
    }
    while (fgets(str, MAXCHAR, fp) != NULL)
        printf("%s", str);
    fclose(fp);
    return 0;
}

int oProblems() {
    FILE *fp;
    char str[MAXCHAR];
    char *fileName = "helpFiles/problems.txt";

    fp = fopen(fileName, "r");
    if (fp == NULL){
        printf("Could not open this help file %s", fileName);
        return 1;
    }
    while (fgets(str, MAXCHAR, fp) != NULL)
        printf("%s", str);
    fclose(fp);
    return 0;
}

int oIwi() {
    FILE *fp;
    char str[MAXCHAR];
    char *fileName = "helpFiles/iwi.txt";

    fp = fopen(fileName, "r");
    if (fp == NULL){
        printf("Could not open this help file %s", fileName);
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
    char *fileName = "helpFiles/usrNam.txt";

    fp = fopen(fileName, "r");
    if (fp == NULL){
        printf("Could not open this help file %s", fileName);
        return 1;
    }
    while (fgets(str, MAXCHAR, fp) != NULL)
        printf("%s", str);
    fclose(fp);
    return 0;
}

int oPowerOff() {
    FILE *fp;
    char str[MAXCHAR];
    char *fileName = "helpFiles/powerOff.txt";

    fp = fopen(fileName, "r");
    if (fp == NULL){
        printf("Could not open this help file %s", fileName);
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
    char *fileName = "helpFiles/restart.txt";

    fp = fopen(fileName, "r");
    if (fp == NULL){
        printf("Could not open this help file %s", fileName);
        return 1;
    }
    while (fgets(str, MAXCHAR, fp) != NULL)
        printf("%s", str);
    fclose(fp);
    return 0;
}

int oNetConnect() {
    FILE *fp;
    char str[MAXCHAR];
    char *fileName = "helpFiles/netConnect.txt.txt";

    fp = fopen(fileName, "r");
    if (fp == NULL){
        printf("Could not open this help file %s", fileName);
        return 1;
    }
    while (fgets(str, MAXCHAR, fp) != NULL)
        printf("%s", str);
    fclose(fp);
    return 0;
}

int main() {
    int option;

    printf("1) FOMOSCL \n");
    printf("2) Commands \n");
    printf("3) Problems \n");
    printf("4) iwi \n");
    printf("5) usrNam \n");
    printf("6) powerOff \n");
    printf("7) restart \n");
    printf("8) netConnect.txt \n");

    printf("Enter a number: ");
    scanf("%d", &option);

    switch(option) {
        case 1 :
            oFOMOSCL();
            break;
        case 2 :
            oCommands();
            break;
        case 3 :
            oProblems();
            break;
        case 4:
            oIwi();
            break;
        case 5:
            oUsrNam();
            break;
        case 6:
            oPowerOff();
            break;
        case 7:
            oRestart();
            break;
        case 8:
            oNetConnect();
            break;
        default :
            printf("NaN \n" );
            printf("Exiting help menu... \n");
    }

    return 0;
}

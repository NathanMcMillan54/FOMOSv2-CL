
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include "../headerFiles/openFile.h"

int exitInstall = 1;

int help() {
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

void option() {
    printf("I ) Install\n");
    printf("U ) Update\n");

    char input[4];
    printf("Enter I or U: ");
    scanf("%s", input);
    if(!strcmp(input, "help")) {
        help();
    } else if (!strcmp(input, "exit")) {
        printf("Exiting iwi...\n");
        exitInstall = 0;
    } else if (!strcmp(input, "I")) {
        system("sh iwi/iwiInstall.sh");
    } else if (!strcmp(input, "U")) {
        system("sh iwi/iwiUpdate.sh");
    }
    else {
        printf("Enter I or U \n");
    }

}

int main() {
    for (;;) {
        option();
        if (exitInstall == 0) {
            exit(0);
        }
    }
}

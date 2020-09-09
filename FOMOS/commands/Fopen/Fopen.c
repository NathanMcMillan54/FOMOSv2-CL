#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "openFile.h"

int main() {
    FILE *fp;
    char str[MAXCHAR];
    char fileName[50];

    printf("\nFile name: ");
    scanf("%s", fileName);

    if (!strcmp(fileName, "2020")) {
        printf("You are a terrible person \n");
        exit(0);
    } else {
        // nothing
    }

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

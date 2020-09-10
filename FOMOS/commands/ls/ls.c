#include <stdio.h>
#include "../../include/FOMOS/openFile.h"

int main() {
    FILE *fp;
    char str[MAXCHAR];
    char fileName[50] = "ls.txt";

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

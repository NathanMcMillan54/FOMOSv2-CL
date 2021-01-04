#include <unistd.h>
#include <stdio.h>

#include "../../include/files.h"

int readFile(char *fileName) {
    FILE *fp;
    char str[MAXCHAR];

    if (access(fileName, F_OK) != -1) {
        fp = fopen(fileName, "r");
        while (fgets(str, MAXCHAR, fp) != NULL)
            printf("%s - %s\n", fileName, str);
        fclose(fp);
        return 1;
    } else {
        printf("%s - File not found\n", fileName);
        return 0;
    }
}

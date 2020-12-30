#include <stdio.h>

char fileContents[999];

int readFile(char *fileName) {
    FILE *fp;
    fp = fopen(fileName, "r");
    fgets(fileContents, 999, (FILE*)fp);
    return *fileContents;
}

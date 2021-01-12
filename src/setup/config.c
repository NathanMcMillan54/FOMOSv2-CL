#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

#include "config.h"

void updateStartupTimes(int updated) {
    FILE *fp;
    char updateChar[] = {'0', updated, '\0'};
    fp = fopen("/configs/boot/startupTimes", "w");
    fputs(updateChar, fp);
    fclose(fp);
}

int configSetup() {
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
        first_setup();
    } else {
        updateStartupTimes(updatedInte);
        regular_setup();
    }
    fclose(fp);
}

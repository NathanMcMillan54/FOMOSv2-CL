#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

#include "config.h"

int configSetup() {
    FILE *fp;
    char str[MAXCHAR];
    int inte;

    fp = fopen("/configs/boot/startupTimes", "r");
    while (fgets(str, MAXCHAR, fp) != NULL)
        printf("Startup times = %s\n", str);
    inte = atoi(str);
    if (inte == 0) {
        first_setup();
    } else {
        regular_setup();
    }
    fclose(fp);
}

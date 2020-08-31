#include <stdio.h>
#include <unistd.h>
#include "include/FOMOS/time.h"

int main() {
    printf("\a");
    printf("FOMOSv2-CL \n");

    time_t rawtime;
    struct tm *timeinfo;
    time(&rawtime);
    timeinfo = localtime(&rawtime);

    char *currentTime = asctime(timeinfo);

    printf("%s", currentTime);

    sleep(0xFFFFFFFF);
    return 0;
}

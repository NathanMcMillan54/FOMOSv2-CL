#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

int powerOff = 1;

void CL() {

    char command[50];

    printf("Connect to wifi by typing 'netConnect.txt' if you need help type 'help' in the command line.");
    printf("\nEnter command: ");
    scanf("%s", command);
    if(!strcmp(command, "help")) {
        system("gcc -Wall helpFiles/help.c -o helpFiles/help");
        system("./helpFiles/help");
    } else if (!strcmp(command, "powerOff")) {
        system("gcc -Wall FOMOSCL/power/powerOff.c -o FOMOSCL/power/powerOff");
        system("sudo ./FOMOSCL/power/powerOff");
        powerOff = 0;
    } else if (!strcmp(command, "restart")) {
        system("gcc -Wall FOMOSCL/power/restart.c -o FOMOSCL/power/restart");
        printf("Enter your password \n");
        system("sudo ./FOMOSCL/power/restart");
        powerOff = 0;
    } else if (!strcmp(command, "netConnect.txt")) {
        system("gcc -Wall FOMOSCL/netConnect.txt/netConnect.txt.c -o FOMOSCL/netConnect.txt/netConnect.txt");
        system("./FOMOSCL/netConnect.txt/netConnect.txt");
    } else {
        printf("Unknown command \n");
    }
}

void printTime() {
    time_t rawtime;
    struct tm *timeinfo;
    time(&rawtime);
    timeinfo = localtime(&rawtime);

    char *currentTime = asctime(timeinfo);

    printf("%s", currentTime);
}

int main() {
    printf("______________ \n");
    printf("| FOMOSv2-CL | \n");
    printf("-------------- \n");

    printTime();

    for (;;) {
        CL();
        if (powerOff == 0) {
            printf("Goodbye \n");
            exit(0);
        }
    }
}

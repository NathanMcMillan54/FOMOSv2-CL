#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "include/FOMOS/finishSetup.h"
#include "include/FOMOS/commands/power.h"
#include "include/FOMOS/commands/commands.h"
#include "include/FOMOS/commands/user.h"
#include "include/FOMOS/commands/filesCommand.h"

int power = 1;

void CL() {
    char command[50];

    printf("\n$ ");
    scanf("%s", command);
    if(!strcmp(command, "help")) {
        help();
    } else if (!strcmp(command, "shutdown")) {
        sudo();
        shutdown();
        power = 0;
    } else if (!strcmp(command, "restart")) {
        sudo();
        restart();
        power = 0;
    } else if (!strcmp(command, "rndmstf")) {
        system("sh waouegfw93333333333333ouawoeuyr320k.sh");
    } else if (!strcmp(command, "Fopen")) {
        Fopen();
    } else if (!strcmp(command, "usrNam")) {
        usrNam();
    } /* this is if FMOE was installed */ else if (!strcmp(command, "FMOE")) {
        sudo();
        FMOE();
    } else if (!strcmp(command, "pwd")) {
        pwd();
    } else if (!strcmp(command, "ls")) {
        ls();
    } else if (!strcmp(command, "mkdir")) {
        mkdir();
    }
    else {
        printf("%s is not a command\n", command);
    }

}

int main() {
    // before v2 release uncomment this part
    // finish_setup();
    printf("\a");
    printf("FOMOSv2-CL \n");
    system("sh users/user.sh");
    for (;;) {
        CL();
        if (power == 0) {
            exit(0);
        } else if (power > 1) {
            printf("There was a problem \n");
            printf("Shutting down to solve problem... \n");
            shutdown();
        } else if (power < 0) {
            printf("power < 0 \n");
            printf("Shutting down immediately \n");
            shutdown();
        }
    }

}

#include <stdio.h>
#include <string.h>
#include <unistd.h>

char commandInput[999];
char *binPath = "/bin/";
char commandPath[1004];
char commandArg[1000];
char fullCommand[2004];

void run_command();
void commandExists();

void cl_input() {
    commandPath[0] = '\0';
    scanf("%s %s", commandInput, commandArg);
    strcat(commandPath, binPath);
    strcat(commandPath, commandInput);
    commandExists();
}

void commandExists() {
    if (access(commandPath, F_OK) != -1) {
        run_command();
    } else {
        printf("%s - Command not found\n", commandInput);
    }
}

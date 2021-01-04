#include <stdio.h>
#include <string.h>
#include <unistd.h>

char commandInput[999];
char *binPath = "/bin/";
char commandPath[1004];

void run_command();
void commandExists();

void cl_input() {
    commandPath[0] = '\0';
    scanf("%s", commandInput);
    strcat(commandPath, binPath);
    strcat(commandPath, commandInput);
    commandExists();
}

void commandExists() {
    // printf("%s\n", commandPath);
    if (access(commandPath, F_OK) != -1) {
        run_command();
    } else {
        printf("%s - Command not found\n", commandInput);
    }
}
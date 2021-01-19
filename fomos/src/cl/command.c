#include <stdio.h>
#include <string.h>
#include <unistd.h>

char commandName;
char commandArg;

void run_command();
void commandExists();

void cl_input() {
    scanf("%s %s", &commandName, &commandArg);
    printf("\nfomos/src/cl/command.c command: %s argument[s]: %s\n", &commandName, &commandArg)
    // commandExists();
}

void commandExists() {
    if (access(commandPath, F_OK) != -1) {
        run_command(*commandInput, *commandArg);
    } else {
        printf("%s - Command not found\n", fullCommand);
    }
}

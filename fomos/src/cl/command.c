#include <stdio.h>
#include <string.h>
#include <unistd.h>

char commandName[999];
char commandArg[999];

extern void run_command(char *command, char *argument);

void cl_input() {
    scanf("%s %s", commandName, commandArg);
    run_command(commandName, commandArg);
}

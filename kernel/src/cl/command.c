#include <stdio.h>
#include <string.h>

char commandInput[999];
char *binPath = "/bin/";
char commandPath[1004];

void command_exists(char command);

void cl_input() {
    scanf("%s", commandInput);
    strcat(commandPath, binPath);
    strcat(commandPath, commandInput);
    command_exists(*commandPath);
}

#include <stdio.h>

char commandInput[999];

void command_exists(char command);

void cl_input() {
    scanf("%s", commandInput);
    command_exists(*commandInput);
}

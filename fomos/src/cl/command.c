#include <stdio.h>
#include <string.h>
#include <unistd.h>

extern void run_command(int command, char *argument);

void cl_input() {
    char commandName[50];
    char commandArguments[50];
    scanf("%s %[^\n]", commandName, commandArguments);
    if (!strcmp(commandName, "print")) {
        run_command(1, commandArguments);
    } else {
        run_command(0, commandArguments);
    }
}

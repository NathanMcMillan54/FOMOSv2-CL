#include <stdio.h>
#include <string.h>
#include <unistd.h>

extern void run_command(int command, char *argument);

void cl_input() {
    char commandName[50];
    char commandArguments[50];
    scanf("%s %s", commandName, commandArguments);
    if (!strcmp(commandName, "print")) {
        run_command(1, commandArguments);
    } else {
        printf("It's not print, it's: %s\n", commandName);
    }
}

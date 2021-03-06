/* fomos/src/cl/command.c
 *
 * Build/compile:
 * cargo build
 *
 * Description:
 * This file gets command input then sends the command and argument to cl.rs so it can run.
 */

#include <stdio.h>
#include <string.h>
#include <unistd.h>

#include "command.h"

extern void run_command(int command, char *argument);

void cl_input() {
    char commandName[50];
    char commandArguments[50];
    scanf("%s %[^\n]", commandName, commandArguments);
    if (!strcmp(commandName, "print")) {
        run_command(1, commandArguments);
    } else if (!strcmp(commandName, "asku")) {
        run_command(2, commandArguments);
    } else if (!strcmp(commandName, "shutdown")) {
        run_command(3, commandArguments);
    } else if (!strcmp(commandName, "help")) {
        help_command(commandArguments);
    } else if (!strcmp(commandName, "makefile")) {
        run_command(5, commandArguments);
    }
    else {
        run_command(0, commandArguments);
    }
}

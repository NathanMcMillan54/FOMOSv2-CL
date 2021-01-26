#ifndef FOMOSV2_CL_COMMAND_H
#define FOMOSV2_CL_COMMAND_H

#include <string.h>

extern void help_main(int command);

void help_command(char *command) {
    if (!strcmp(command, "all")) {
        help_main(0);
    } else if (!strcmp(command, "print")) {
        help_main(1);
    } else if (!strcmp(command, "asku")) {
        help_main(2);
    } else if (!strcmp(command, "shutdown")) {
        help_main(3);
    } else if (!strcmp(command, "help")) {
        help_main(4);
    } else {
        help_main(0);
    }
}

#endif //FOMOSV2_CL_COMMAND_H

/* This file is for calling all the commands for FOMOS
 * A FOMOS commands is actually just calling a file from the commands directory */

#ifndef FOMOSV2_CL_COMMANDS_H
#define FOMOSV2_CL_COMMANDS_H

#include <stdlib.h>
#include "power.h"

void help() {
    system("./commands/help/help");
}

void Fopen() {
    system("./commands/Fopen/Fopen");
}

#endif //FOMOSV2_CL_COMMANDS_H

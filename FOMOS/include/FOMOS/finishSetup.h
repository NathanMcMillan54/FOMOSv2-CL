/* This file is for "deleting" the setup file */

#ifndef FOMOSV2_CL_FINISHSETUP_H
#define FOMOSV2_CL_FINISHSETUP_H

#include <stdio.h>
#include "power/power.h"

void finish_setup() {
    FILE *usrFile;
    char setupF[200] = "#include <stdio.h>\n"
                       "#include <stdlib.h>\n"
                       "\n"
                       "void main(void) {\n"
                       "\tprintf('Starting FOMOS 2/2 Finished \\n');\n"
                       "\tsystem('make all');\n"
                       "\tsystem('./main');\n"
                       "\treturn 0;\n"
                       "}";

    if ((usrFile = fopen("setup.c", "w")) == NULL) {
        printf("Cannot finish setting up FOMOS \n");
        printf("Restarting to try to fix this problem \n");
        restart();
    } else {
        fputs(setupF, usrFile);
        fclose(usrFile);
    }
}

#endif //FOMOSV2_CL_FINISHSETUP_H

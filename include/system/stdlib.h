/* FOMOS hasn't been tested yet but this is just a copy of stdlib.h just in case the real stdarg.h can't be put onto the
 * device that is running FOMOS.
 * Here https://pubs.opengroup.org/onlinepubs/009695399/basedefs/stdlib.h.html you can find all the stuff you need to know
 * about stdlib.h */

#ifndef FOMOSV2_CL_STDLIB_H
#define FOMOSV2_CL_STDLIB_H

extern int system (const char *__command);

#endif //FOMOSV2_CL_STDLIB_H

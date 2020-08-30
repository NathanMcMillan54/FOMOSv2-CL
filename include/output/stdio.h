/* FOMOS hasn't been tested yet but this is just a copy of stdio.h just in case the real stdio.h can't be put onto the
 * device that is running FOMOS.
 * Here https://pubs.opengroup.org/onlinepubs/7908799/xsh/stdio.h.html you can find all the stuff you need to know
 * about stdio.h */

#ifndef FOMOSV2_CL_STDIO_H
#define FOMOSV2_CL_STDIO_H

#include "stdarg.h"

#define	ENOMEM		12
#define	EINVAL		22
#define ENOSPC		28

extern int printf(const char *fmt, ...) __attribute__((format(printf, 1, 2)));


#endif //FOMOSV2_CL_STDIO_H

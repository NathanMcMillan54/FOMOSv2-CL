#ifndef FOMOSV2_CL_MISC_H
#define FOMOSV2_CL_MISC_H

#include <linux/compiler.h>

void error(char *x) __noreturn;
extern unsigned long free_mem_ptr;
extern unsigned long free_mem_end_ptr;

#endif //FOMOSV2_CL_MISC_H

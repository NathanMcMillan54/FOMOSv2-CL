#ifndef FOMOSV2_CL_STRT_FOMOS_KENREL_H
#define FOMOSV2_CL_STRT_FOMOS_KENREL_H

#include <linux/module.h>
#include <linux/kernel.h>

#include "err_FOMOS.h"

int power = 1;


extern int main();
extern int system_off(char *arch);

void strt_FOMOS() {
	for (;;) {
		if (power == 1) {
			main();
			power = 0;
		} else if (power == 0) {
			system_off("arm");
		} else {
			err_loop(999999);
			power = 1;
		}
	}
}

#endif //FOMOSV2_CL_STRT_FOMOS_KENREL_H

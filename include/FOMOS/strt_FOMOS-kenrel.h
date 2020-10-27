#ifndef FOMOSV2_CL_STRT_FOMOS_KENREL_H
#define FOMOSV2_CL_STRT_FOMOS_KENREL_H

#include <linux/module.h>
#include <linux/kernel.h>

void FOMOS_main() {
	for (;;) {

	}
}

extern int strt_FOMOS() {
    	printk("Starting FOMOSv2-CL");
	FOMOS_main();
    return 0;
}



#endif //FOMOSV2_CL_STRT_FOMOS_KENREL_H

#ifndef FOMOSV2_CL_STRT_FOMOS_KENREL_H
#define FOMOSV2_CL_STRT_FOMOS_KENREL_H

#include <linux/module.h>
#include <linux/kernel.h>

void FOMOS_main() {
	// Loop for now
	// Figure out how to get this to call FOMOS_main in FOMOS/src/main.rs
	for (;;) {	}
}

extern int strt_FOMOS() {
    	printk("Starting FOMOSv2-CL");
	FOMOS_main();
    return 0;
}



#endif //FOMOSV2_CL_STRT_FOMOS_KENREL_H

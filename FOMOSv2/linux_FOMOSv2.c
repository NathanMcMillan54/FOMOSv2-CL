#include <kernel.h>
#include "linux_FOMOSv2.h"

int power = 1;

int linix_FOMOSv2(void) {
	printk("Starting FOMOSv2-CL v2.3.5\nRunning on Linux Kernel v5.9\n");
	for (;;) {
		if (power == 1) {
			init();
		} else if (power == 0) {
			// Shutdown
		} else {
			// Restart
		}
	}
}

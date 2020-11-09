#include <linux/module.h>
#include <linux/kernel.h>

#include "linux_FOMOSv2.h"


// Start FOMOS function from src/lib.rs
extern void init(void);


int linux_FOMOSv2CL(void) {
    init();
    return 0;
}

int _start() {
    linux_FOMOSv2CL();
    return 0;
}

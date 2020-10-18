// FOMOS
#include "../../include/FOMOS/kernel_modules/kernel_modules.h"

// Linux
#include <linux/module.h>
#include <linux/kernel.h>


int init_module(void) {
    printk(KERN_INFO "Starting modules...\n");
    setup_FOMOS();
    setup_user();
    return 0;
}

void cleanup_module(void) {
    printk(KERN_INFO "Ending modules...\n");
}

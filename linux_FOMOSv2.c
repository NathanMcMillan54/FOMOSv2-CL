#include <linux/init.h>
#include <linux/module.h>
#include <linux/kernel.h>
#include <linux/slab.h>
#include <linux/bug.h>

#include "linux_FOMOSv2.h"

void abort(void)
{
    BUG();
}

// Start FOMOS function from src/lib.rs
extern void init(void);

static int linux_FOMOSv2_start(void)
{
    // This is a fake loading screen
    char loading[3];
    for (int i = 0; i < 3; ++i)
    {
        loading + "-";
        printk(KERN_INFO "Linux kernel ");
        printk(KERN_INFO "%d", loading);
        printk(KERN_INFO "> FOMSOv2-CL CL\n");
    }
    init();
    return 0;
}

static int linux_FOMOSv2_end(void)
{
    printk(KERN_INFO "Shutting down...\n");
    end_shutdown();
}

module_init(linux_FOMOSv2_start);
module_exit(linux_FOMOSv2_end);

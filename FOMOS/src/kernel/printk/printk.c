#include <linux/module.h>
#include <linux/kernel.h>

extern int linux_printk(argument) {
	printk(argument);
	return 0;
}

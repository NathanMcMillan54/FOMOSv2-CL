// src/power.h
#ifndef FOMOSV2_CL_POWER_H
#define FOMOSV2_CL_POWER_H

#include <linux/reboot.h>
#include <unistd.h>
#include <sys/reboot.h>

void shutdown() {
    reboot(LINUX_REBOOT_CMD_POWER_OFF);
}

void restart() {
    sync();
    reboot(RB_AUTOBOOT);
}

#endif //FOMOSV2_CL_POWER_H

#ifndef FOMOSV2_CL_REBOOT_H
#define FOMOSV2_CL_REBOOT_H

extern void call_with_stack(void (*fn)(void *), void *arg, void *sp);
extern void _soft_restart(unsigned long addr, bool disable_l2);

#endif //FOMOSV2_CL_REBOOT_H

/* This file is for counting time */

#ifndef FOMOSV2_CL_COUNTTIME_H
#define FOMOSV2_CL_COUNTTIME_H

extern unsigned int sleep (unsigned int __seconds);

int s1() {
    sleep(1);
    return 0;
}

int m1() {
    sleep(60);
    return 0;
}

int h1() {
    sleep(3600);
    return 0;
}

#endif //FOMOSV2_CL_COUNTTIME_H

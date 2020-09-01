#include <stdio.h>
#include <stdlib.h>
#include "include/FOMOS/countTime.h"

int main() {
    int f = 0;
    printf("\a");
    printf("FOMOSv2-CL \n");

    s1();
    for (;;) {
        f ++;
        printf("%d \n", f);
        m1();
        if (f == 8) {
            exit(0);
        } else {
            // nothing
        }
    }

}

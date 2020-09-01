/* #include <stdio.h>
 * // #include "include/textOutAndInput/print.h"
 * // #include "include/FOMOS/countTime.h"
 *
 * void main(void) {
 *     printf("FOMOSv2-CL\n");
 *     m1();
 * }
 * */

#include <stdio.h>
#include <unistd.h>

int main(void) {
    printf("\a");
    printf("Starting FOMOSv2-CL... \n")
    // this never ends
    sleep(0xFFFFFFFF);
    return 0;
}

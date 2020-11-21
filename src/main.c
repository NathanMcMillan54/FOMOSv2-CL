/* main.c
 *
 * Build:
 * gcc -static main.c -o init
 *
 * Description:
 * This is the initramfs file that the Linux kernel will run to start FOMOSv2-CL.
 * */

#include <stdio.h>

extern void init_main();

int main()
{
    printf("Starting initramfs...\n");
    // init_main();
    while (1) {    }
}

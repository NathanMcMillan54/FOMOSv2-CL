/* main.c
 *
 * Build:
 *
 * gcc -static main.c -o init
 *
 * */

#include <stdio.h>

extern void init_main();

int main()
{
    printf("Starting initramfs...\n");
    // init_main();
    while (1) {    }
}

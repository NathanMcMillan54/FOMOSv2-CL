/* src.main.c
 *
 * Build/compile:
 * <arch>-gcc -static src/main.c target/<arch>/debug/libFOMOSv2-CL.a -o init
 *
 * Description:
 * After the Linux fomos is done it's setup, this file starts to run and calls init_main() from src/lib.rs to start
 * FOMOS.
 */

#include <stdio.h>
#include <unistd.h>

extern void init_main();

int main() {
    sleep(1);
    printf("Starting initramfs...\n");
    sleep(1);
    init_main();
    while (1) {  }
}

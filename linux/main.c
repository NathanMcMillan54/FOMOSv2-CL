#include <stdio.h>
#include <unistd.h>

extern void init_main();

int main(void) {
    sleep(2);
    printf("Starting initramfs...\n")
    init_main();
    while (1) {    }
}

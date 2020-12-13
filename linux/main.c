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

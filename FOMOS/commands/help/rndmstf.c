#include <stdio.h>
#include <unistd.h>

int main() {
    printf("$s is not a command\n");
    sleep(86400);
    printf("y did u wait this long\n");
    printf("did u actually think something would happen?\n");
    sleep(5);
    for (;;) {
        fprintf(stdout, "\aLol\n");
    }
}

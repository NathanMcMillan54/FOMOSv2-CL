#include <stdio.h>
#include <unistd.h>

void lol() {
    fprintf(stdout, "\aLol\n" );
}

int main() {
    sleep(86400);
    printf("y did u wait this long\n");
    printf("did u actually think something would happen?\n");
    sleep(5);
    for (;;) {
        lol();
    }
}

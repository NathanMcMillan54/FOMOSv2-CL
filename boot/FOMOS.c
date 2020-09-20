#include "../include/stdio.h"

// _start() is form src/start.s
extern int _start();

int main() {
    printf("Starting FOMOS... \n");

    // this is actually fomos
    _start();
    return 0;
}

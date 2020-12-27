#include <stdio.h>
#include <unistd.h>

// #include <include/types.h>
#include "../types.h"

int filePath(char filename) {
    if (access(&filename, F_OK)) {
        return true;
    } else {
        return false;
    }
}

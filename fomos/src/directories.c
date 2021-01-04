#include <stdio.h>
#include <dirent.h>

int readDir(char *dirName) {
    DIR *d;
    struct dirent *dir;
    d = opendir("/configs/");
    if (d) {
        while ((dir = readdir(d)) != NULL) {
            printf("%s\n", dir->d_name);
        }
        closedir(d);
    } else {
        printf("%s - Not found\n", dirName);
    }
    return 0;
}

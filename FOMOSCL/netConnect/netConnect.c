#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>
#include <string.h>

int exitNetConnect = 1;

int aQ() {
    char yOn[1];

    printf("Would you like to connect to a network? (y/n) ");
    scanf("%s", yOn);

    if (!strcmp(yOn, "y")) {
        system("sh netConnect/connect2Network.sh");
    } else if (!strcmp(yOn, "n")) {
        exitNetConnect = 0;
    } else {
        printf("Enter y or n \n");
    }

    return 0;
}


int main() {
    printf("Starting netConnect... \n");
    system("sh requirements.sh");
    sleep(1);
    system("sh netConnect/localNetworks.sh");

    for (;;) {
        aQ();
        if (exitNetConnect == 0) {
            exit(0);
        }
    }

}

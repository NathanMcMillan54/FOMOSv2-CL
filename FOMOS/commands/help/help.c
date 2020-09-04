#include <stdio.h>

int main() {
    int num
    printf("Help\n");
    printf("______________________________________\n");
    printf("| 1) About FOMOS   2) About FMOE     |\n");
    printf("| 3) About restart 4) About shutdown |\n");
    printf("--------------------------------------\n");
    printf("Enter a number \n");
    scanf("%d", &num);

    switch (num) {
        case 1:
            // open FOMOS.txt
            break;
        case 2:
            // open FMOE.txt
            break;
        case 3:
            // open restart.txt
            break;
        case 4:
            // open shutdown.txt
            break;
        default:
            printf("NaN \n");
    }

    return 0;
}

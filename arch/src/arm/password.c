#include <stdio.h>
#include <string.h>
#include <unistd.h>

int passwordOption;
char inputUserPassword[6];

int passwordOptionInput() {
    scanf("%d", &passwordOption);
    if (passwordOption == 1) {
        return passwordOption;
    } else if (passwordOption == 2) {
        return passwordOption;
    } else {
        return 1;
    }
}

int shortPassword() {
    scanf("%s", inputUserPassword);
    ssize_t password = strlen(inputUserPassword);
    if (password == 4) {
        return *inputUserPassword;
    } else if (password == 5) {
        return *inputUserPassword;
    } else if (password == 6) {
        return *inputUserPassword;
    } else {
        printf("Enter 4-6 digit password\n");
        shortPassword();
    }
    return 0;
}
